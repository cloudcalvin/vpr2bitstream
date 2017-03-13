
use types::*;
use errors::*;

use std::iter::Map;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::io;
use std::io::prelude::*;

pub use regex::{Regex,Captures};



pub fn load_placement<P : AsRef<Path>>(file_path: P) -> Result<(u32, String, String, Vec<Placement>, Vec<Vec<BlockBuilder>>)>{

  //Setup regex's
  lazy_static! { // TODO  :make these regex look better.
    static ref PLACE_FILE_REGEX_files : Regex = Regex::new(
      r"^Netlist file: (?P<netlist_file>.+) Architecture file: (?P<arch_file>.+)"
    ).unwrap();
    static ref PLACE_FILE_REGEX_array_size : Regex = Regex::new(
      r"^Array size:\s+(?P<size>\d+)\s+"
     ).unwrap();
    static ref PLACE_FILE_REGEX_data_lines: Regex = Regex::new(
      r"^(?P<name>(?-u:\b).+(?-u:\b))[[:space:]]+(?P<x>\d)[[:space:]]+(?P<y>\d)[[:space:]]+(?P<sub_blk>\d)[[:space:]]+#(?P<blk_nr>\d)"
    ).unwrap();
  }
  // Init variable
  let mut n : u32 = 0;
  let mut netlist_file = String::new();
  let mut arch_file = String::new();
  let mut placement_list : Vec<Placement> = Vec::new();
  let mut blocks : Vec<Vec<BlockBuilder>> = Vec::new();//with_capacity(N); //how to make lazy?
  let file_name =  (*file_path.as_ref()).to_str().unwrap();// {

  //Read File into Buffer
  let f = try!(File::open(Path::new(&file_name)));
  let mut file = BufReader::new(&f);

  let mut lines_enumerated = file.lines().enumerate();

  let mut line_count : usize = 0;
  let mut lines_zipped : Vec<(usize,String)> = Vec::new();

  for (i,line) in lines_enumerated{
    lines_zipped.push((i,String::from(line?)));
    line_count = i;
  }



  let mut lines_zip_iter = lines_zipped.iter();
  while let Some(&(idx,ref line)) = lines_zip_iter.next(){
    match idx {
      // IF LINE 0 : capture file names.
      0 =>
        {
          let captured : Option<Captures> = PLACE_FILE_REGEX_files.captures(line); //captures, executes the regex query defined in 'util.rs'
          match captured{
            Some(ref cap) => {
              let net = try!(Captures::name(cap,"netlist_file")
                  .ok_or::<Error>("No Netlist file specified in .place file.".into()));

              let arch = try!(Captures::name(cap,"arch_file")
                  .ok_or::<Error>("No Architecture file specified in .place file.".into()));
              netlist_file = String::from(net.as_str());
              arch_file = String::from(arch.as_str());
              println!("Netlist file used : {}", &netlist_file);
              println!("Architecture file used : {}", &arch_file);
            },
            _ => println!("Error parsing")//Err("Malformed .parse file".into())
          }
        },
      // IF LINE 1 : capture lut array size.
      1 =>
        {
          let captured : Option<Captures> = PLACE_FILE_REGEX_array_size.captures(&line); //captures, executes the regex query defined in 'util.rs'
          match captured{
            Some(ref cap) => {
              let array_size  = try!(Captures::name(cap, "size")
                  .ok_or::<Error>("No array size specified in .place file.".into()));
              n = array_size.as_str().parse::<u32>()?;
              println!("LUT array width : {}", &array_size.as_str());

            },
            _ => println!("Malformed .parse file")
          }
        },
      2 | 3 | 4 => { //skip lines 2,3 and 4.

      },
      _ => { //exit when reached body.
        break;
      }
    };

  }

  //Initialise block matrix. //todo : use ndarray.
  for y in 0..n {
    let mut y_row : Vec<BlockBuilder> = Vec::with_capacity(n as usize);
    for x in 0..n {
      let mut block = BlockBuilder::default();
      block.xy(Point(x,y));
      y_row.push(block.clone())
    }
    blocks.push(y_row);
  };

  // Parse Body
  let mut lines_zip_iter = lines_zipped.iter();
  while let Some(&(idx,ref line)) = lines_zip_iter.next(){
    if idx >= 5 {
      let captured : Option<Captures> = PLACE_FILE_REGEX_data_lines.captures(line); //captures, executes the regex query defined in 'util.rs'
      match captured{
        Some(ref cap) => {
          let name = try!(Captures::name(cap,"name")
              .ok_or::<Error>(format!("{} (LINE: {}) : No blk name specified",&file_name,&idx).into())); //todo : revise the errors.

          let xs = try!(Captures::name(cap,"x")
              .ok_or::<Error>(format!("{} (LINE: {}) : No x coordinate specified",&file_name,&idx).into()));

          let ys = try!(Captures::name(cap,"y")
              .ok_or::<Error>(format!("{} (LINE: {}) : No y coordinate specified",&file_name,&idx).into()));

          let sub_blk = try!(Captures::name(cap,"sub_blk")
              .ok_or::<Error>(format!("{} (LINE: {}) : No subblk specified",&file_name,&idx).into()));

          let blk_nr = try!(Captures::name(cap,"blk_nr")
              .ok_or::<Error>(format!("{} (LINE: {}) : No blk_nr specified",&file_name,&idx).into()));

          let x = xs.as_str().parse::<u32>().unwrap();
          let y = ys.as_str().parse::<u32>().unwrap();

          blocks[x.clone() as usize][y.clone() as usize]
              .name(name.as_str())
              .sub_blk(sub_blk.as_str().parse::<u8>().unwrap());

          placement_list.push((String::from(name.as_str()),Point(x.clone(),y.clone())));
        }
        _ => break
      }
    }else{
      break
    }
  }
  Ok((n,netlist_file,arch_file,placement_list,blocks))
}
/////////////////////////////////////////////////////////////////////////////////////////////////
//Parse Header
/////////////////////////////////////////////////////////////////////////////////////////////////
// pub fn load_blif<P : AsRef<Path>>(file_path: P) -> Result<(u32, String, String, Vec<Placement>, Vec<Vec<BlockBuilder>>)> {
//   //Setup regex's
//   Ok(())
// }
/*
//Nets are populated from the .route file.
pub struct Net{
  src_loc : Source,
  src_pin : u8,
  route_tree : Vec<(Vec<(Point,Point)>,Sink)>, //each outer vec is a new sub source. each inner vec is a connection.
}

This function uses regex's to capture the info from the .route file. To populate graph/vector. It could be used directly to create bitstream but lets rather keep it modular.

*/

/////////////////////////////////////////////////////////////////////////////////////////////////
//Parse .route file :
/////////////////////////////////////////////////////////////////////////////////////////////////

///
///
///
///
///
/// for every net in data :
///   you get 3 pieces :
///     1) net header : net nr and name
///     2) nodes : lines of Node : nr, position etc...
///     3) whitespace :
///
///
///
/// Example :
///
///
///
///
///
///
///
///

pub fn load_routing<P : AsRef<Path>>(file_path: P) -> Result<(u32, Vec<Net>)> {

  // Init return variables and regex
  let mut n: u32 = 0;
  let mut nets: Vec<Net> = Vec::new();

  lazy_static! {
    static ref RE_net_seperator : Regex = Regex::new(
       r"Net"
    ).unwrap();

    static ref RE_file_header_array_size : Regex = Regex::new(
      r"Array size: (?P<arr_size>\d+) x"
    ).unwrap();

    static ref RE_node_seperator: Regex = Regex::new(
       r"\n\n"
    ).unwrap();

    static ref RE_net_header: Regex = Regex::new(
      r"^(?P<net_nr>\d+) ((?P<net_name>[[:graph:]]+))"
    ).unwrap();
  }

  //Read File into Buffer
  let file_name = (*file_path.as_ref()).to_str().unwrap();// {
  let mut f = try!(File::open(Path::new(&file_name)));
  // let mut file = BufReader::new(&f);
  let mut contents = String::new();
  f.read_to_string(&mut contents).unwrap();
  println!("Read file : {}", &file_name);
  // println!("{}",&contents);
  // let mut buf = vec![];
  // file.read_to_end(&mut buf);
  //Split file into header and data
  let (header, data) = {
    let mut parts = RE_net_seperator.split(&contents);
    let h = parts.next().ok_or("Malformed .route file").unwrap();
    (h,parts)
  };
  //Parse header
  let header_line = header.lines().next().ok_or("Malformed .route file").unwrap(); //is making it an iterator when only need the first line more/less costly? probably more.
  let captured: Option<Captures> = RE_file_header_array_size.captures(&header_line);
  match captured {
    Some(ref cap) => {
      let array_size = try!(Captures::name(cap, "arr_size")
          .ok_or::<Error>("No array size specified in .place file.".into()));
      n = array_size.as_str().parse::<u32>()?;
    },
    _ => println!("Malformed .parse file")
  }



  let mut nets : Vec<Net> = Vec::new();

  //Parse Data
  for net_text in data {
    nets.push(parse_net(net_text)?);
  }


  Ok((n, nets))
}


/// Parsing Nets :
/// Returns the parsed data from a text repesentation of a net.
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
fn parse_net(text : &str) -> Result<Net>{

  //setup regex
  lazy_static! {
    static ref RE_node_separator: Regex = Regex::new(
       r"\n\n"
    ).unwrap();

    static ref RE_net_header: Regex = Regex::new(
      r"(?P<net_nr>\d+)[[:blank:]]+\((?P<net_name>.+)\)"
    ).unwrap();
  }

  //split each net string into 3 pieces.
  let mut net_parts = RE_node_separator.split(&text);

  //choose part 1 for net header.
  let mut net_header = net_parts
      .next().ok_or::<Error>("error : malformed file. Could not find node data in net :".into()).unwrap();

  //choose part 2 for net body:
  let mut nodes = net_parts
      .take(1).next().ok_or::<Error>("error : malformed file. Could not find node data in net :".into()).unwrap()
      .lines();

  //parse net header
//  let mut c : ref Captures = Captures::
//  let mut net_nr : u32 = 0;
//  let mut net_name = String::new();
//  let captured: Option<Captures> = RE_net_header.captures(&net_header);

//  match captured {
//    Some(ref cap) => {
//      println!("helo");
//      let net_nr = Captures::name(cap, "net_nr")
//          .ok_or::<Error>("Net number missing in file: , line : .".into())?
//          .as_str().parse::<u32>()?;//.map_err(|e| e.into())?;
////          .ok_or::<Error>("Could not parse net number to u32")
////          .unwrap();
//      println!("helo");
//
//      let net_name = try!(Captures::name(cap, "net_name")
//          .ok_or::<Error>("Net name missing in file: , line : .".into()))
//          .as_str();
//      println!("helo");
//
//    },
//    _ => panic!("could not parse .place file : invalid net header")
//  }

  let ref net_cap = RE_net_header.captures(&net_header).ok_or::<Error>("error with regex capture".into()).unwrap();

  let net_nr = try!(Captures::name(net_cap, "net_nr")
      .ok_or::<Error>("Net number missing in file: , line : .".into()))
      .as_str().parse::<u32>()?;

  let net_name = try!(Captures::name(net_cap, "net_name")
      .ok_or::<Error>("Net name missing in file: , line : .".into()))
      .as_str();


  // pub type RouteTree = Vec<(Vec<(Channel,Track)>,Sink)>;

  // the first two lines describe the source
  let src           = nodes.next().ok_or::<Error>("Malformed .route file: Missing SOURCE node".into())?;
  let src_pin       = nodes.next().ok_or::<Error>("Malformed .route file: missing IPIN node".into())?;
  let src_data      = parse_node(src)?;
  let pin_data      = parse_node(src_pin)?;

  //init route tree
  let mut route_tree = RouteTree::new();
  //the rest of the nodes are channels and (IPin+Sink)'s
  let mut new_route = true;

  while let Some(node) = nodes.next(){
    let Node{node_nr,node_type,xy,meta_nr} = parse_node(node)?;
    let _ = match node_type {
      NodeType::IPin =>
        {
          let sink_node = nodes.next().unwrap_or("Malformed .route file: No SINK node after IPIN node");
          let Node{meta_nr : class_nr , ..} = parse_node(sink_node)?;

          let pin_nr  = meta_nr;
          let route = route_tree.last_mut().unwrap();
          route.sink = Some(Sink(xy,class_nr,pin_nr));
          new_route = true;

          // Ok(())
        },
      NodeType::Chan(orientation) =>
        {
          let mut track = meta_nr;
          let mut ch = Channel{
            id: node_nr,
            xy: xy,
            orientation: orientation,
            track : track,
          };
          if new_route {
            new_route = false;
            route_tree.push(Route{
              channels: Vec::new(),
              sink: None,
            });
          }
          let route = route_tree.last_mut().unwrap();
          route.channels.push(ch);
          // Ok(())
        },
      NodeType::OPin => {
        //dont think i need to know this..
        println!("Ignoring secondary OPIN nodes found in net : {} {} {:?} {} with the SOURCE OPIN at {:?} {}",node_nr,&"OPIN",xy,meta_nr, src_data.xy, pin_data.meta_nr)
      }

      _ => panic!("Malformed .route file")
    };
  }
  
//   for node in nodes{

//     //      if node is a sink. add sink and push an empty
//     //      if not sink, edit .last()
// //    if node_type.as_str() == "SINK" {
// //      //you wil have to read the next node at this point... maybe its better to put all nodes into an indexable array.

//   }

  Ok(Net{
      name : net_name.to_owned(),
      src: Source(src_data.xy, src_data.meta_nr, pin_data.meta_nr) ,
      route_tree: route_tree,
    })
  }


/// Parsing Nodes :

fn parse_node(line : &str) -> Result<Node> {

  lazy_static!(
    static ref ROUTE_FILE_REGEX_NET_data_line: Regex = Regex::new(r"(?x)
      Node:
      [[:blank:]]+
      (?P<node_nr>\d+)
      [[:blank:]]+
      (?P<node_type>.+)
      [[:blank:]]+
      \((?P<x>\d+),(?P<y>\d+)\)
      [[:blank:]]+
      (?P<meta_name>.+):
      [[:blank:]]+
      (?P<meta_nr>\d+)"
    ).unwrap();
  );

  if let Some(ref cap) = ROUTE_FILE_REGEX_NET_data_line.captures(line) {
    //captures, executes the regex query defined in 'util.rs'
    //TODO : change all these errors to regex error.
    let node_nr = try!(Captures::name(cap, "node_nr")
        .ok_or::<Error>(format!("No node specified").into()))         //  Option<Match> -> Result<Match>
        .as_str()                                                                                       //  Match -> String
        .parse::<u32>()                                                                                 //  String -> unsigned int
        .unwrap();                                                                                      //  Result<u32> -> u32
    let node_type_str = try!(Captures::name(cap, "node_type")
        .ok_or::<Error>(format!("No node type specified").into()))
        .as_str();
//    println!("node_type_str {} ",&node_type_str);

    let x = try!(Captures::name(cap, "x")
        .ok_or::<Error>(format!("No node x coordinate type specified").into()))
        .as_str()
        .parse::<u32>()
        .unwrap();

    let y = try!(Captures::name(cap, "y")
        .ok_or::<Error>(format!("No node y coordinate type specified").into()))
        .as_str()
        .parse::<u32>()
        .unwrap();
    //not actually using this
    /*let meta_name = try!(Captures::name(cap, "meta_name")
        .ok_or::<Error>(format!("{} (LINE: {}) : Pin/pad/track/class not specified", &file_name, &idx).into()))
        .as_str(); */

    let meta_nr = try!(Captures::name(cap, "meta_nr")
        .ok_or::<Error>(format!("Pin/pad/track/class not specified").into()))
        .as_str()
        .parse::<u32>()
        .unwrap();

    let node_type : Result<NodeType>  = match node_type_str{
      "SOURCE" => Ok(NodeType::Source),
      "SINK" => Ok(NodeType::Sink),
      "CHANX" => Ok(NodeType::Chan(XY::X)),
      "CHANY" => Ok(NodeType::Chan(XY::Y)),
      "IPIN" => Ok(NodeType::IPin),
      "OPIN" => Ok(NodeType::OPin),
      _ => bail!("Unsupported node type")
    };

    Ok(Node{
      node_nr: node_nr,
      node_type: node_type?,
      xy: Point(x,y),
      meta_nr: meta_nr
    })

  } else {
    println!("error : malformed file");
    Err("Malformed .route file".into())
  }
}

#[test]
fn split_test() {
  let re = regex::Regex::new(r"bb|cc").unwrap();
  for part in re.split("aabbaacaaaccaaa") {
    println!(">{}<", part);
  }
}
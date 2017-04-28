use types::*;
use errors::*;
#[macro_use]
use global::*;

use std::iter::Map;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

pub use regex::{Regex, Captures, RegexSet};

/////////////////////////////////////////////////////////////////////////////////////////////////
//Parse .place file :
/////////////////////////////////////////////////////////////////////////////////////////////////
/// Returns  ```Ok((n,netlist_file,arch_file,placement_list))```
///   n               => width of LUT array
///   netlist_file    => netlist file
///   arch_file       => architecture file
///   placement_list  => a vector of placements (the locations of the blocks)
///
///
/// <explain what happens to the blocks in more detail>
///
/// look at file to see header :
/// 1) netlist file
/// 2) architecture file
/// 3) array size
///
/// for every line in data :
///   you get following columns : | #block | name	| x	 | y | 	subblk	| block number |
///
/// Example :
///
/// ``` #block name	x	y	subblk	block number
///     #----------	--	--	------	------------
///     sum[1]		  7	4	0	#0
///     n29		      2	1	0	#1
///     sum[2]		  6	2	0	#2
///     n31		      7	3	0	#3
///     sum[3]		  7	1	0	#4
///     n33		      7	2	0	#5
///     sum[4]		  6	5	0	#6
///     n35		      5	3	0	#7
/// ``` note: not as well aligned in real file.
///
pub fn parse_place_file<P: AsRef<Path>>(file_path: P) -> Result<(u16, String, String, Placement)> {
  //Setup regex's
  lazy_static! { // TODO  :make these regex look better.
    static ref PLACE_FILE_REGEX_files : Regex = Regex::new(
      r"^Netlist file: (?P<netlist_file>.+) Architecture file: (?P<arch_file>.+)"
    ).unwrap();
    static ref PLACE_FILE_REGEX_array_size : Regex = Regex::new(
      r"^Array size:\s+(?P<size>\d+)\s+"
     ).unwrap();
    static ref PLACE_FILE_REGEX_data_lines: Regex = Regex::new(
      r"^(?P<name>[[:graph:]]+)\s+(?P<x>\d+)\s+(?P<y>\d+)\s+(?P<sub_blk>\d+)\s+#(?P<blk_nr>\d+)"
    ).unwrap();
  }
  // Init variable
  let mut n: u16 = 0;
  let mut netlist_file = String::new();
  let mut arch_file = String::new();
  let mut placement: Placement = HashMap::new();
  let file_name = (*file_path.as_ref()).to_str().unwrap();// {
  println!("Read file : {}", &file_name);

  //Read File into Buffer
  let f = try!(File::open(Path::new(&file_name)));
  let mut file = BufReader::new(&f);


  //  let mut lines_enumerated = file.lines().enumerate();
  let mut lines = file.lines();

  let mut line_count: usize = 0;
  let mut lines_zipped: Vec<(usize, String)> = Vec::new();


  for line in lines {
    let line = line?;
    //    println!(".parse file body : {}",&line);
    lines_zipped.push((line_count, String::from(line)));
    line_count = line_count + 1;
  }
  //  for (i,line) in lines_enumerated{
  //    let line = line?;
  //    println!(".parse file body : {}",&line);
  //
  //    lines_zipped.push((i,String::from(line)));
  //    line_count = i;
  //  }
  let mut lines_zip_iter = lines_zipped.iter();

  //Parse Header
  while let Some(&(idx, ref line)) = lines_zip_iter.next() {
    match idx {
      // IF LINE 0 : capture file names.
      0 =>
        {
          let captured: Option<Captures> = PLACE_FILE_REGEX_files.captures(line); //captures, executes the regex query defined in 'util.rs'
          match captured {
            Some(ref cap) => {
              let net = try!(Captures::name(cap, "netlist_file")
                  .ok_or::<Error>("No Netlist file specified in .place file.".into()));

              let arch = try!(Captures::name(cap, "arch_file")
                  .ok_or::<Error>("No Architecture file specified in .place file.".into()));
              netlist_file = String::from(net.as_str());
              arch_file = String::from(arch.as_str());
              //
            },
            _ => println!("Error parsing")//Err("Malformed .parse file".into())
          }
        },
      // IF LINE 1 : capture lut array size.
      1 =>
        {
          let captured: Option<Captures> = PLACE_FILE_REGEX_array_size.captures(&line); //captures, executes the regex query defined in 'util.rs'
          match captured {
            Some(ref cap) => {
              let array_size = try!(Captures::name(cap, "size")
                  .ok_or::<Error>("No array size specified in .place file.".into()));
              n = array_size.as_str().parse::<u16>()?;
            },
            _ => println!("Malformed .parse file")
          }
        },
      _ => {
        //exit when reached body.
        break;
      }
    };
  }

  // //Initialise block matrix. //todo : use ndarray.
  // for y in 0..n {
  //   let mut y_row : Vec<BlockBuilder> = Vec::with_capacity(n as usize);
  //   for x in 0..n {
  //     let mut block = BlockBuilder::default();
  //     block.xy(Point(x,y));
  //     y_row.push(block.clone())
  //   }
  //   blocks.push(y_row);
  // };
  // Parse Body
  //  let mut lines_zip_iter = lines_zipped.iter();
  while let Some(&(idx, ref line)) = lines_zip_iter.next() {
    //    if idx >= 5 {
    let captured: Option<Captures> = PLACE_FILE_REGEX_data_lines.captures(line); //captures, executes the regex query defined in 'util.rs'
    match captured {
      Some(ref cap) => {
        //          println!(".parse file body : {}",&line);
        let name = try!(Captures::name(cap, "name")
            .ok_or::<Error>(format!("{} (LINE: {}) : No blk name specified", &file_name, &idx).into())); //todo : revise the errors.

        let xs = try!(Captures::name(cap, "x")
            .ok_or::<Error>(format!("{} (LINE: {}) : No x coordinate specified", &file_name, &idx).into()));

        let ys = try!(Captures::name(cap, "y")
            .ok_or::<Error>(format!("{} (LINE: {}) : No y coordinate specified", &file_name, &idx).into()));

        let sub_blk = try!(Captures::name(cap, "sub_blk")
            .ok_or::<Error>(format!("{} (LINE: {}) : No subblk specified", &file_name, &idx).into()));

        let x = xs.as_str().parse::<u16>().unwrap();
        let y = ys.as_str().parse::<u16>().unwrap();

//        placement_list.push((String::from(name.as_str()), Point(x.clone(), y.clone())));
        placement.insert(String::from(name.as_str()), Point(x.clone(), y.clone()));
        //          println!("{:#?}",(String::from(name.as_str()),Point(x.clone(),y.clone())));
      }
      None => info_println!("skipping BLIF lines : >{}<", &line)
    }
    //    }else{
    //      break
    //    }
  }
  Ok((n, netlist_file, arch_file, placement))
}

/////////////////////////////////////////////////////////////////////////////////////////////////
//Parse .route file :
/////////////////////////////////////////////////////////////////////////////////////////////////

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

pub fn parse_route_file<P: AsRef<Path>>(file_path: P) -> Result<(u16, Vec<Net>)> {
  // Init return variables and regex
  let mut n: u16 = 0;
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

//
  }

  //Read File into Buffer
  let file_name = (*file_path.as_ref()).to_str().unwrap();// {
  let mut f = try!(File::open(Path::new(&file_name)));
  // let mut file = BufReader::new(&f);
  let mut contents = String::new();
  f.read_to_string(&mut contents).unwrap();
  println!("Read file : {}", &file_name);

  //split file into header vs data
  let (header, data) = {
    let mut parts = RE_net_seperator.split(&contents);
    let h = parts.next().ok_or("Malformed .route file : Unable to split file into nets.").unwrap();
    (h, parts)
  };

  //Parse header
  let header_line = header.lines().next().ok_or("Malformed .route file : No Header found.").unwrap(); //is making it an iterator when only need the first line more/less costly? probably more.
  let captured: Option<Captures> = RE_file_header_array_size.captures(&header_line);
  match captured {
    Some(ref cap) => {
      let array_size = try!(Captures::name(cap, "arr_size")
          .ok_or::<Error>("No array size specified in .place file.".into()));
      n = array_size.as_str().parse::<u16>()?;
    },
    _ => println!("Malformed .route file")
  }


  //Parse Data
  for net_text in data {

//    let parsed parse_net(net_text)(|net| net)
    nets.push(parse_net(net_text)?);
  }
  //return
  Ok((n, nets))
}


/// Returns the parsed data from a text representation of a net.
fn parse_net(text: &str) -> Result<Net> {
  //setup regex
  lazy_static! {
    static ref RE_node_separator: Regex = Regex::new(
      r"\n\n"
    ).unwrap();
    static ref RE_net_header: Regex = Regex::new(
      r"(?x)
      (?P<net_nr>\d+)[[:blank:]]+\((?P<net_name>[[:graph:]]+)\)             # Node net
      #|                                                                     # or
      #(?P<net_nr>\d+)[[:blank:]]+\((?P<net_name>[[:graph:]]+)\):
      #[[:blank:]]+?global[[:blank:]]+?net[[:blank:]]+?conneting             # Global net
      "
    ).unwrap();
    static ref RE_global_net: Regex = Regex::new(
      r"global net connecting"
    ).unwrap();
  }

  //split each net string into 3 pieces.
  let mut net_parts = RE_node_separator.split(&text);

  //choose part 1 for net header.
  let mut net_header = net_parts
      .next().ok_or::<Error>("error : malformed file. Could not find node data in net :".into()).unwrap();

  //choose part 2 for net body:
  let mut body = net_parts
      .take(1).next().ok_or::<Error>("error : malformed file. Could not find node data in net :".into()).unwrap()
      .lines();

  let ref net_cap = RE_net_header.captures(&net_header).ok_or::<Error>("error with regex capture".into()).unwrap();

  let net_nr = try!(Captures::name(net_cap, "net_nr")
      .ok_or::<Error>("Net number missing in file: , line : .".into()))
      .as_str().parse::<u64>()?;

  let net_name = try!(Captures::name(net_cap, "net_name")
      .ok_or::<Error>("Net name missing in file: , line : .".into()))
      .as_str();


  if RE_global_net.is_match(&net_header) {
    //net is a global net..  so dont call parse node.. call parse globals
    println!("skipping global net");
    Ok(Net::Global(NetGlobal))

  } else {
    // the first two lines describe the source
    let src = body.next().ok_or::<Error>("Malformed .route file: Missing SOURCE node".into())?;
    let src_pin = body.next().ok_or::<Error>("Malformed .route file: missing first OPIN node".into())?;
    let src_data : Node = parse_node(src)?;
    let pin_data : Node = parse_node(src_pin)?;

    //init route tree
    let mut route_tree = RouteTree::new();
    //the rest of the nodes are channels(tracks actually) and (IPin+Sink)'s
    let mut new_route = true;
    let mut opin_found = true;

    while let Some(node) = body.next() {
      let parse_result = parse_node(node);
      let _ = match parse_result {
        Ok(Node { node_nr, node_type, xy, meta_type, meta_nr }) => {
          let _: Result<()> = match node_type {
            NodeType::IPin =>
              {
                let sink_node = body.next().unwrap_or("Malformed .route file: No SINK node after IPIN node");
                let Node { meta_nr: class_nr, .. } = parse_node(sink_node)?;

                let pin_nr = meta_nr;
                let mut route = route_tree.last_mut().unwrap();
                route.sink = Some(Sink(xy, meta_type, class_nr, pin_nr));
                new_route = true;
                Ok::<(), Error>(())
              },
            NodeType::Chan(orientation) =>
              {
                let mut track_nr = meta_nr;
                let mut track = Track {
                  id: node_nr,
                  nr: track_nr,
                  xy: xy,
                  orientation: orientation,
                  //            track: (),
                };
                //if a sink was found, push a new Vec to the route tree;
                if new_route {
                  new_route = false;
                  route_tree.push(Route {
                    tracks: Vec::new(),
                    from_opin: opin_found,
                    sink: None,
                  });
                }
                opin_found = false;
                //if a sink was found, the 'last_mut' will get the latest route tree.
                let route = route_tree.last_mut().unwrap();
                route.tracks.push(track);
                Ok::<(), Error>(())
              },
            NodeType::OPin => {
              //dont think i need to know this..
              info_println!("Ignoring secondary OPIN nodes found in net : \
                  {} {} {:?} {} with the SOURCE OPIN at {:?} {}",
                       node_nr, &"OPIN", xy, meta_nr, src_data.xy, pin_data.meta_nr);
              // route.opins.push()
              opin_found = true;
              Ok::<(), Error>(())
            }

            _ => Err("Malformed .route file".into())
          };
          Ok::<(), Error>(())
        },

        //      Err(e) if &e == &"BLOCK".into() => Ok(()),
        //      Err(e) => Err(e.into())
        //      Err(e) => match Error::from(e) {
        //        Error(ErrorKind::Msg(m), _) => match &*m.to_string() {
        //          "BLOCK" => Ok::<(), Error>(()),
        //          m @ _ => Err(m.into())
        //        },
        //        //        Error::from("BLOCK")
        //        _ => Err(e.into())
        //      }
        Err(e) => Err(e.into())
      };
    }
    Ok(Net::Local(NetLocal {
      name: net_name.to_owned(),
      src: Source(src_data.xy, pin_data.meta_type ,src_data.meta_nr, pin_data.meta_nr),
//      net_type: NetType::NonGlobal,
      route_tree: route_tree,
    }))
  }

}


/// Parsing Nodes :

fn parse_node(line: &str) -> Result<Node> {
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
      (?P<meta_nr>\d+)
      |Block(?P<block_data>.+)"
    ).unwrap();
  );

  if let Some(ref cap) = ROUTE_FILE_REGEX_NET_data_line.captures(line) {
    /*    if let Some(capture) = Captures::name(cap, "block_data"){
    //      Err("BLOCK".into())
          Ok(Node{
            node_nr: node_nr,
            node_type: node_type?,
            xy: Point(x,y),
            meta_type : meta_type?,
            meta_nr: meta_nr
          })

        }else{*/
    //TODO : change all these errors to regex error.
    let node_nr = try!(Captures::name(cap, "node_nr")
        .ok_or::<Error>(format!("No node specified").into()))                                       //  Option<Match> -> Result<Match>
        .as_str()                                                                                   //  Match -> String
        .parse::<u64>()                                                                             //  String -> unsigned int
        .unwrap();                                                                                  //  Result<u32> -> u32
    let node_type_str = try!(Captures::name(cap, "node_type")
        .ok_or::<Error>(format!("No node type specified").into()))
        .as_str();

    let x = try!(Captures::name(cap, "x")
        .ok_or::<Error>(format!("No node x coordinate type specified").into()))
        .as_str()
        .parse::<u16>()
        .unwrap();

    let y = try!(Captures::name(cap, "y")
        .ok_or::<Error>(format!("No node y coordinate type specified").into()))
        .as_str()
        .parse::<u16>()
        .unwrap();

    let meta_type_str = try!(Captures::name(cap, "meta_name")
        .ok_or::<Error>(format!("No node type specified").into()))
        .as_str();

    let meta_nr = try!(Captures::name(cap, "meta_nr")
        .ok_or::<Error>(format!("Pin/pad/track/class not specified").into()))
        .as_str()
        .parse::<u16>()
        .unwrap();

    let node_type: Result<NodeType> = match node_type_str {
      "SOURCE" => Ok(NodeType::Source),
      "SINK" => Ok(NodeType::Sink),
      "CHANX" => Ok(NodeType::Chan(XY::X)),
      "CHANY" => Ok(NodeType::Chan(XY::Y)),
      "IPIN" => Ok(NodeType::IPin),
      "OPIN" => Ok(NodeType::OPin),
      _ => bail!("Unsupported node type")
    };

    let meta_type: Result<NodeMetaType> = match meta_type_str {
      "Pin" => Ok(NodeMetaType::Pin),
      "Pad" => Ok(NodeMetaType::Pad),
      "Class" => Ok(NodeMetaType::Class),
      "Track" => Ok(NodeMetaType::Track),
      _ => bail!("Unsupported node type")
    };

    Ok(Node {
      node_nr: node_nr,
      node_type: node_type?,
      xy: Point(x, y),
      meta_type: meta_type?,
      meta_nr: meta_nr
    })
  } else {
    println!("skipping line : >{}<", line);
    Err("Malformed .route file".into())
  }
  //    }
}
/////////////////////////////////////////////////////////////////////////////////////////////////
//Parse .blif file :
//contains '.names' id's and lut content. content mapped to input ports..
/////////////////////////////////////////////////////////////////////////////////////////////////
/// Returns (populates) an array of blocks partially with the LUT content.
///
/// <explain what happens to the blocks in more detail>
///
/// look at file to see header : nothing..
///
/// for every line in data :
///   you get following columns : | #block | name	| x	 | y | 	subblk	| block number |
///
/// Example :
///
/// ``` #block name	x	y	subblk	block number
///     #----------	--	--	------	------------
///     sum[1]		  7	4	0	#0
///     n29		      2	1	0	#1
///     sum[2]		  6	2	0	#2
///     n31		      7	3	0	#3
///     sum[3]		  7	1	0	#4
///     n33		      7	2	0	#5
///     sum[4]		  6	5	0	#6
///     n35		      5	3	0	#7
/// ``` note: not as well aligned in real file.
///
///
pub fn parse_blif_file<'a, P: AsRef<Path>>(file_path: P) -> Result<Vec<Model>> {
  let mut models: Vec<Model> = Vec::new();
  //Setup regex's
  lazy_static! {
    static ref RE_split_on_model : Regex = Regex::new(
      r"\.model\s+"
    ).unwrap();
    static ref RE_blif_header : Regex = Regex::new(
      r"[[:blank:]]+(on)[[:blank:]]+(\w+)[[:blank:]]+(?<month>\w+)[[:blank:]]+(?<day>\d{2})[[:blank:]]+(?<hour>\d{2}):(?<min>\d{2}):(?<sec>\d{2})[[:blank:]]+(?<year>\d{4})"
    ).unwrap();

  }


  let file_name = (*file_path.as_ref()).to_str().unwrap();// {

  //Read File into Buffer
  let f = try!(File::open(Path::new(&file_name)));
  let mut file = BufReader::new(&f);

  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  info_println!("Read file : {}", &file_name);


  //split file into header vs data
  let (header, body) = {
    let mut parts = RE_split_on_model.split(&contents);
    let h = parts.next().ok_or("Malformed .blif file").unwrap();
    (h, parts)
  };

  //Parse header
  let header_line = header.lines().next().ok_or("Malformed .blif file").unwrap(); //is making it an iterator when only need the first line more/less costly? probably more.
  //  let captured: Option<Captures> = RE_blif_header.captures(&header_line);
  //  match captured {
  //    Some(ref cap) => {
  //      let array_size = try!(Captures::name(cap, "arr_size")
  //          .ok_or::<Error>("No array size specified in .place file.".into()));
  //      n = array_size.as_str().parse::<u32>()?;
  //    },
  //    _ => println!("Malformed .parse file")
  //  }

  //Parse body
  for model in body {
    models.push(parse_blif_model(model)?)
  }
  //  Ok(Vec<Model>)
  Ok(models)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
///  // split on .model, //TODO : update this little explanation
///  //   split on .*
///  //   [0][0] => header
///  //   [1][0] => model_name
///  //   [1][1] => inputs N*<input> (\ \n <additional_input>+)*
///  //   [1][2] => outputs (<output>)+ (\ \n <additional_output>+)*
///  //   [1][3] => first lut
///  //   [1][4] => second lut
fn parse_blif_model(content: &str) -> Result<Model> {
  lazy_static! {
    static ref RE_split_on_dot : Regex = Regex::new(
//      r"\s*\.\w+\s+" // this should also remove the word that is part of the delimeter.
      r"(?m)^\s*\." // this should also remove the word that is part of the delimeter.
    ).unwrap();
    static ref RE_blif_lut_ports: Regex = Regex::new(
      r"(?P<name>(?-u:\b).+(?-u:\b))[[:space:]]+(?P<x>\d)[[:space:]]+(?P<y>\d)[[:space:]]+(?P<sub_blk>\d)[[:space:]]+#(?P<blk_nr>\d)"
    ).unwrap();
    static ref RE_blank :  Regex = Regex::new(r"\s").unwrap();
    static ref RE_linefeed : Regex = Regex::new(r"\\").unwrap();
    static ref RE_primitive : Regex = Regex::new(r"(?s)names\s+(?P<lut_line>.+)|(?s)latch\s+(?P<latch_line>.+)|(?s)subckt\s+(?P<subskt_line>.+)").unwrap();

  }
  fn parse_list(str: &str) -> Result<Vec<String>> {
    let re: Regex = Regex::new(r" \\\n |\s").unwrap();
    let s: Vec<String> = re.split(&str).map(|s| s.to_owned()).skip(1).collect();
    Ok(s)
  };

  //split model into model header and primitives
  let (mut model, model_parts) = {
    let mut model_parts = RE_split_on_dot.split(&content);
    let mut parts = RE_split_on_dot.split(&content);
    for p in parts {
      debug_println!("part : {:?}", p);
    }
    let model_name_line = model_parts.next().ok_or("Malformed blif file : Incorrect model header.").unwrap();
    let inputs = model_parts.next().ok_or("Malformed blif file : Incorrect model header.").unwrap();
    let outputs = model_parts.next().ok_or("Malformed blif file : Incorrect model header.").unwrap();

    let model_name = Regex::new(r"(?P<name>[[:graph:]]+)\s*\n").unwrap().captures(model_name_line).unwrap().name("name").unwrap();
    //setup model without the LUT data.
    let model_struct = Model {
      name: model_name.as_str().to_owned(),
      inputs: parse_list(inputs)?,
      outputs: parse_list(outputs)?,
      latched: Vec::new(),
      logic: Vec::new(),
      //meant to store LogicBlocks
    };


    (model_struct, model_parts)
  };

  //parse primitives
  for primitive in model_parts {
    if let Some(ref cap) = RE_primitive.captures(primitive) {
      if let Some(lut_line) = Captures::name(cap, "lut_line") {
        debug_println!("captured a lut : {:?}", lut_line.as_str());
        let mut inputs: Vec<String> = Vec::new();
        let mut output = String::new();
        let mut lines = lut_line.as_str().lines(); //remember : iterator, thus keeps state..

        //parse input names.
        let mut seed: &str = lines.next().unwrap();
        while let Some(_) = Regex::new(r"\\")?.captures(seed) {
          // if '\' present extend inputs parsing into next line.
          inputs.append(&mut RE_blank.split(seed).map(|s| s.to_owned()).collect::<Vec<String>>());
          seed = lines.next().unwrap();
        }
        inputs.append(&mut RE_blank.split(&seed).map(|s| s.to_owned()).collect());

        //output is last value from the parsed inputs
        output = inputs.pop().ok_or::<Error>("No output".into())?;


        lazy_static! {
          static ref k : usize = 3;
          static ref tt_size : usize = (2 as usize).pow(3);
        }

        let truth_lines = lines.map(|s| s.to_owned()).collect();

        let mut truth_idxs = Vec::with_capacity(*tt_size);
        let mut truth_table = vec![false; *tt_size];

        let mut tt_full = Vec::new();
        for i in 0..*tt_size {
          tt_full.push(format!("{:01$b}", i, k));
        }


//        for line in truth_lines {
//          //      println!("line: {}",&line);
//          if let Some(ref cap) = Regex::new(r"(?P<in>\d+) (\d+)").unwrap().captures(line) {
//            let _in = Captures::name(cap, "in")
//                .ok_or::<Error>(format!("Could not fine output value in truth table").into()).unwrap()
//                .as_str().to_owned();
//            let mut _in = _in;
//            let mut idx = 0u16;
//            for i in 0.._in.len(){
//              let char = _in.pop().unwrap();
//              if char == '1'{
//                idx = idx + 2u16.pow(i as u32)
//              }
//            }
//
//          } else {
//            panic!("Could not capture any inputs or outputs from truth-table")
//          }
//        }

        //dont-care replacement
        let mut lut_data = replace(truth_lines);


        //assumption made here that truth table is in right order.
        for line in lut_data.iter() {
          //      println!("line: {}",&line);
          if let Some(ref cap) = Regex::new(r"(?P<in>\d+) (?P<out>\d+)").unwrap().captures(line) {
            let _out = Captures::name(cap, "out")
                .ok_or::<Error>(format!("Could not fine output value in truth table").into()).unwrap()
                .as_str();
            let _in = Captures::name(cap, "in")
                .ok_or::<Error>(format!("Could not fine output value in truth table").into()).unwrap()
                .as_str().to_owned();
            let mut _in = _in;
            while _in.len() != 3 {
              _in = _in.chars().rev().collect();     // need to swop around the order so that I
              _in.push('0');                         //     can push from the front..
              _in = _in.chars().rev().collect();     // then swop back. // todo : find pushn(0,'0')
            }
//            truth_indexes = std::num::from_str_radix::<usize>(&_out, 2).unwrap();
            let idx = tt_full.binary_search(&_in).unwrap();
            truth_table[idx] = true;
            truth_idxs.push(idx);
          } else {
            panic!("Could not capture any inputs or outputs from truth-table")
          }
        }

        model.logic.push(LogicBlock {
          inputs: inputs,
          output: output,
          //      truth_table: truth.iter().map(|t| t.unwrap()).collect(),
          //      latched: false,
          truth_idxs : truth_idxs,
          truth_table: truth_table,
        })
      } else if let Some(latch_line) = Captures::name(cap, "latch_line") {
        debug_println!("captured a latch : {:?}", latch_line.as_str());
      } else if let Some(subskt_line) = Captures::name(cap, "subckt_line") {
        debug_println!("captured a subskt : {:?}", subskt_line.as_str());
      } else {
        print!("Malformed blif - Could not parse line : >{}<", &primitive);
      }
    }
  }

  //  println!("{:#?}",&model);
  Ok(model)
}

/// dont-care replacement (text based)
fn replace(vec: Vec<String>) -> Vec<String> {
  let mut new_lut = Vec::new();
  let mut done = 1;
  for line in vec {
    if let Some(i) = line.rfind("-") {
      done = 0;
      let mut new_line_0: Vec<char> = line.chars().collect();
      let mut new_line_1: Vec<char> = line.chars().collect();
      new_line_0[i] = '0' as char;
      new_lut.push(new_line_0.into_iter().collect::<String>());
      new_line_1[i] = '1' as char;
      new_lut.push(new_line_1.into_iter().collect::<String>());
    } else {
      new_lut.push(line)
    }
  }
  if done == 1 {
    new_lut.sort();//just remove dups
    new_lut
  } else {
    replace(new_lut)
  }
}

#[test]
fn split_test() {
  let re = regex::Regex::new(r"bb|cc").unwrap();
  for part in re.split("aabbaacaaaccaaa") {
    println!(">{}<", part);
  }
}
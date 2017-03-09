

//read the .place file, and generate the placement bitstream based on input parameters and mapping file.
//read the .route file and generate the routing bitstream based on input parameters and mapping file.

/*

settings (coordinate system (vpr-style == cartesian))
read file ( .place )
  place file consists of:
    file name
    array size
    space
    header
    placement-data

read file ( .route )

  route file consists of:
    array size
    \n
    'Routing'
    \n
    routing-data


read file (.pre-vpr.blif)
  1) model name
  2) inputs port names
  3) output port names
  4) .names :
    .name nInputs output
  5) .end


  1) Want to be able to match the BLE placement(in .place) with the blif data for that BLE(in .pre-vpr.blif).
  2) want to know the output pins for a BLE and the routing ch_X or ch_y that it goes to as well as whether it connects above or below.

*/

// single double-node fpga tree? maybe routing tree and placement tree.

/*
 1 ) read blif. build out configurations.
 2 ) get placement details.
 3 ) read .route
 4 ) mangle routing

*/

/*
  y
  ^
  |
  |
  |
  |
  o ---------->x

*/

mod errors;
mod types;

use errors::*;
use types::*;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate derive_builder;
//#[macro_use]
//extern crate approx; // For the macro relative_eq!
extern crate nalgebra as na;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use na::{Vector3, Rotation3};
use na::{Dynamic, MatrixArray, MatrixVec, DMatrix};
use regex::{Regex,Captures};


use std::iter::Map;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
//use std::error::Error;
use std::io;
use std::io::prelude::*;


//struct BitStreamBuilder{
//  tile_matrix : Vec<Vec<Tile>>,
//  bitstream : Vec<bool>,
//}
//impl BitStreamBuilder{
//  fn new(n : u16) -> BitStreamBuilder{
//    let mut tiles : Vec<Vec<Tile>> = Vec::capacity(N);
//    tiles = for y in N {
//      let y_row : Vec<Tile> = Vec::capacity(N);
//      for x in N{
//        y_row.push(TileBuilder::default().xy(x,y))
//      }
//      y_row
//    }
//  }
//  fn flatten(&self){
//    //flatten in the correct way for easy output.
//  }
//  fn get_bitstream(&self) -> Vec<u8>{
//
//  }
//}
//
//impl IntoTokens for Path{
//  fn to_tokens<'a>(&'a self) -> Option<Cow<'a, [u8]>>{
//    // let mut buffer : Box<[u8]> = vec![];
//    let mut buffer = vec![];
//    let mut f: File = File::open(&self).unwrap();
//    f.read_to_end(&mut buffer).unwrap();
//    // let buf : &'a Box<[u8]> = &buffer.into_boxed_slice();
//    Some(buffer.into())
//  }
//}
//
//fn get_file() -> Result<(), io::Error> {
//  let f = try!(File::open(file));
//  Ok()
//}
//
//fn parse_blif()
//
//fn parse_routing()
//
//fn get_place(p : Path) {
//  let f = try!(File::open(p));
//  let mut file : BufReader = BufReader::new(&f);
//  for line in file.lines() {
//    let l = line.unwrap();
//    println!("{}", l);
//  }
//}
//fn init_matrices<Type>(size: u32){
//
//}


//use errors::*;

type Placement = (String, Point);
//fn load_placement(file_path: Path) -> Result<(u32, String, String, Vec<Placement>, Vec<Vec<BlockBuilder>>)>{
fn load_placement<P : AsRef<Path>>(file_path: P) -> Result<(u32, String, String, Vec<Placement>, Vec<Vec<BlockBuilder>>)>{
  //Setup regex
//  type P = AsRef<Path>;
  lazy_static! { // TODO  :make these regex look better.
      static ref PLACE_FILE_REGEX_files : Regex = Regex::new(r"Netlist file: (?P<netlist_file>.+) Architecture file: (?P<arch_file>.+)$").unwrap();
      static ref PLACE_FILE_REGEX_array_size : Regex = Regex::new(r"Array size: (?P<size>\d{[0-9]+}) x $").unwrap();
      static ref PLACE_FILE_REGEX_data_lines: Regex = Regex::new(r"(?P<name>(?-u:\b).+(?-u:\b))[[:space:]]+(?P<x>\d)[[:space:]]+(?P<y>\d)[[:space:]]+(?P<sub_blk>\d)[[:space:]]+#(?P<blk_nr>\d)$").unwrap();
//      static ref PLACE_FILE_REGEX_data_lines: Regex = Regex::new(r"(?P<0>(?-u:\b).+(?-u:\b))[[:space:]]+(?P<1>\d)[[:space:]]+(?P<2>\d)[[:space:]]+(?P<3>\d)[[:space:]]+#(?P<4>\d)$").unwrap();
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
//  let f = try!(File::open(file_path));
  let mut file = BufReader::new(&f);

//  let lines = file.lines();
  let mut lines_enumerated = file.lines().enumerate();

  let mut line_count : usize = 0;
  let mut lines_zipped : Vec<(usize,String)> = Vec::new();

  for (i,line) in lines_enumerated{
    lines_zipped.push((i,String::from(line.unwrap_or("error".into()))));
    line_count = i;
  }


  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Parse Header //todo: put this in a function.
  /////////////////////////////////////////////////////////////////////////////////////////////////
  let mut lines_zip_iter = lines_zipped.iter();
  while let Some(&(idx,ref line)) = lines_zip_iter.next(){
//    let line = String::from(try!(lines.next()));
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
//              Ok(())
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
//              Ok(())
            },
//            _ => Err("Malformed .parse file".into())
            _ => println!("Malformed .parse file")
          }
      },
      2 | 3 | 4 => { //skip lines 2,3 and 4.
        //do nothing..
//        Ok(())
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
  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Parse Body //todo: put this in a function. -- it is hiding the matrix initialization code.
  /////////////////////////////////////////////////////////////////////////////////////////////////
  let mut lines_zip_iter = lines_zipped.iter();
  while let Some(&(idx,ref line)) = lines_zip_iter.next(){
    if (idx >= 5){
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
//  while idx.has_next(){
//
//  }
//
//  for (i,line) in lines.enumerate() {
//
//  }
//  //read header (5 lines)
//  for line in lines[0..4]{
//    let l = line.unwrap();
//    println!("{}", l);
//  }


}

fn main() {
  //
  // 0) read placement and create blocks and channels and tiles with proper 'array size'.
  // 1) read placement to allocate .names to blocks. keep reverse lookup (hashmap (name, pos), hashmap (name
  // 2) read routing. Connect the blocks and nets in the tiles.
  // 3) add the blif data to the right tiles.
  // 4) give each tile an x,y and a row columm. Create the bitstream from the correct connections and blif data.
  // 5) rearrange the tiles according to row,col and flatten row major.
//
//  let ch : ChannelEx = ChannelExBuilder::default()
//      .special_info(42u8)
//      .token(19124)
//      .build()
//      .unwrap();
//
//  println!("{:?}", ch);
  let name = "test".to_owned();

  println!("hello world");
  let place_file = format!("{}{}",&name,".place");
  let route_file = format!("{}{}",&name,".route");
  let blif_file  = format!("{}{}",&name,"pre-vpr.blif");


  match load_placement(Path::new(&place_file)) {
    Ok((N,net_file,arch_file,place,blocks)) => {

    }
    _ => println!("error")
  }
//  let (N,n_nets,nets) = load_routing(route_file);
//  let (count, models) = load_blif(blif_file);
//  let names : Vec<bool> = parse_blif(models);

  //rather use ndarray of Blocks/blockbuilders?

//  let blocks: DMatrix<BlockBuilder> = DMatrix::from_element(BlockBuilder::default(),BlockBuilder::default(),
//                                                            BlockBuilder::default(),BlockBuilder::default());


//  let b : Matrix<BlockBuilder, Dynamic, Dynamic, MatrixVec<BlockBuilder, Dynamic, Dynamic>>;


//
//  let mut tiles : Vec<Vec<TileBuilder>> = Vec::capacity(N);
//  tiles = for y in N {
//    let y_row : Vec<TileBuilder> = Vec::capacity(N);
//    for x in N{
//      y_row.push(TileBuilder::default().xy(x,y))
//    }
//    y_row
//  };
//
//
//  let blocks : Vec<Block> = BlockBuilder::default();
//  blocks
//      .set(N)
//      .parse_placement(place_data);
//
//
//  let fpga_lut_dim = 2;
//  let N = fpga_lut_dim + 1; //from file
//
//  let ch_x : Vec<Vec<Channel>> = Vec::capacity(N);
//  let ch_y : Vec<Vec<Channel>> = Vec::capacity(N);
//
//
//  parse_routing()
//
//  named!(multi< Vec<&str> >, many0!( map_res!(tag!( "abcd" ), str::from_utf8) ) );
//
//  let blif_file = "something";
//  let mut f: File = File::open(&blif_file).unwrap();
//  f.read(&mut buffer).unwrap();
//
//  fn parse_routing(routing : &[str]){
//
//
//    for net in routing {
//      //handle source
//      let src_xy = net.source.
//          //handle output pin
//          // channel routing
//          // input pin
//          // sink
//          //repeat
//
//          source.xy
//    }
//    /*
//      create 2  2x2 nets : ch_x nets and ch_y.
//      for every output (pad, blk)
//
//     */
//  }
//  fn populate_tiles( nets: Vec<Net>, blks : Vec<Vec<Block>>){
//
//  }



//  let N = 5;
//  // let mut blocks : [[Block; N]; N]=
//  let blocks = vec![
//    vec![Block; N]
//  ];
//  let nets = vec![
//    vec![Net; N]
//  ];

//  let mut luts : Map<id, Vec<bool>>
//
//
//  let fn get_place : Map< (x,y) , PlaceTree > {
//    (net_file,arch_file,N,header,data) = load_placement(name + ".place");
//    // data format  : (space+tab delimeted)
//    //        block-name, x, y, subblk, blk number (commented with #)
//
//    //for each line in data:
//    data.map(|line| {
//      let tokenised = line.split(" ");
//      let id = tokenised[0];
//      let (x,y) = (tokenised[1],tokenised[2])
//      map.insert((x,y), id)
//        //for each line look at (x,y). If within array LUT array area, assign
//    })
//  }
//
//  /*
//    Each net starts at a source and ends at a sink..
//    'nets' is a collection of lines that have a header (net nr and port name).
//    lines following the net header describe the connections of the net.
//
//    # types of nets : CH_x, CH_y
//
//
//  */
//  let fn get_routes : Map< (x,y) , PlaceTree > {
//    let (N,n_nets,nets) = load_routing(name + ".route") //contains SOURCE -> SINK description
//    for net in nets {
//      //populate the routing entries in the bitstream?
//      //NO_FLIP_METHOD :  if read ch_x(x,y) then map to tile[row=(N-1)-y][col=x] where N = n_tiles. (2x2luts -> 3)
//      //FLIP_method : ch_x(x,y) maps to tile[x,y] but requires that all blocks be treated as if they were flipped. ch_x() ch_y() y's are flipped, as well as ch_y(lanes 1->2 2->1 3->4 4->3)
//
//      //1) flip the mapping so that indices alighn
//      //2) dont flip, but map the tiles into different programming order?
//    }
//
//
//  }
//  let blif = load_blif(name + ".pre-vpr.blif") //contains '.names' id's and lut content. content mapped to input ports..
//
//
//  /// if 2x2. Then there VPR coords work on a grid of 4x4. Thus 3:0. Where we are mapping it to an array of tiles of 3x3.
//  //// Thus N = 2; n_luts = N*N. lut[N-1:0][N-1:0] and N+1 == n_tiles == n_vpr - 1 thus n_vpr == N+2;
//
//
//
//
//  let bitstream = {
//    //scan van 0,0 na N,N en access die place/route/lut-data trees/maps soos nodig om die single/multiple bitstream te generate
//
//  }

}


mod errors;
mod types;
mod bitstream;
mod parse;

use errors::*;
use types::*;
use parse::*;
use bitstream::*;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate derive_builder;
extern crate nalgebra as na;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate clap;

use clap::App;

use std::path::Path;


fn main() {
  /////////////////////////////////////////////////////////////////////////////////////////////////
  // Initialisation
  /////////////////////////////////////////////////////////////////////////////////////////////////
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  // Gets a value for config if supplied by user, or defaults to "default.conf"
  let config = matches.value_of("config").unwrap_or("default.conf");

  let name = match matches.value_of("INPUT"){
    Some(file_name) => file_name.to_owned(),
    None => String::from("none")
  };

  let blif = match matches.value_of("blif"){
    Some(file_name) => file_name.to_owned(),
    None => name.to_owned()
  };

  let route = match matches.value_of("route"){
    Some(file_name) => file_name.to_owned(),
    None => name.to_owned()
  };

  let place = match matches.value_of("place"){
    Some(file_name) => file_name.to_owned(),
    None => name.to_owned()
  };


  let mut blif_file  = format!("{}{}",&blif,".pre-vpr.blif");
  let mut place_file = format!("{}{}",&place,".place");
  let mut route_file = format!("{}{}",&route,".route");


  println!("Value for config: {:?}", &config);
  println!("VPR project name: {:?}", &name);
  println!("Using blif file : {:?}", &blif_file);
  println!("Using place file: {:?}", &place_file);
  println!("Using route file: {:?}", &route_file);


  // Vary the output based on how many times the user used the "verbose" flag
  // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
  println!("occurences of v : {} ", matches.occurrences_of("verbose").to_string());

  match matches.occurrences_of("verbose") {
    0 => println!("No verbose info"),
    1 => println!("Some verbose info"),
    2 => println!("Tons of verbose info"),
    3 | _ => println!("Don't be crazy"),
  }

  // You can handle information about subcommands by requesting their matches by name
  // (as below), requesting just the name used, or both at the same time
  if let Some(matches) = matches.subcommand_matches("test") {
    if matches.is_present("debug") {
      println!("Printing debug info...");
    } else {
      println!("Printing normally...");
    }
  }

  println!("place{:?}",place_file);
  println!("route{:?}",route_file);
  println!("blif{:?}",blif_file);

  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Load the placement file into the Block matrix.
  /////////////////////////////////////////////////////////////////////////////////////////////////
  let (N,net_file,arch_file,place,blocks) = match load_placement(Path::new(&place_file)) {
    Ok(data) => data,
//    Err(e) => println!("Error loading placement file : {} ", e.to_string())
    Err(e) => panic!("Could not load placement file : {}",e.to_string())
  };

  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Load the blif file into the block matrix
  //contains '.names' id's and lut content. content mapped to input ports..
  /////////////////////////////////////////////////////////////////////////////////////////////////
  // let blif = match load_blif(Path::new(&blif_file)){

  // };



  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Load the routing file into the Nets matrix.
  /////////////////////////////////////////////////////////////////////////////////////////////////
  //contains SOURCE -> SINK description
  let nets = match load_routing(Path::new(&route_file)) {
    Ok(n) => n,
    Err(e) => panic!("Could not load .route file")
  };


  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Generate the bitstream from the 3 matrices.
  /////////////////////////////////////////////////////////////////////////////////////////////////
  // for net in nets {
  //   //populate the routing entries in the bitstream?
  //   //NO_FLIP_METHOD :  if read ch_x(x,y) then map to tile[row=(N-1)-y][col=x] where N = n_tiles. (2x2luts -> 3)
  //   //FLIP_method : ch_x(x,y) maps to tile[x,y] but requires that all blocks be treated as if they were flipped. ch_x() ch_y() y's are flipped, as well as ch_y(lanes 1->2 2->1 3->4 4->3)

  //   //1) flip the mapping so that indices alighn
  //   //2) dont flip, but map the tiles into different programming order?
  // }







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
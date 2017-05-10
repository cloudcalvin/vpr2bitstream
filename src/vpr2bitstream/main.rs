#[macro_use]
extern crate vpr_extra;

use vpr_extra::errors::*;
use vpr_extra::types::*;
use vpr_extra::parse::*;
use vpr_extra::global::*;
use vpr_extra::bitstream::*;
use vpr_extra::output::*;


mod load;
use load::*;

#[macro_use]
pub extern crate lazy_static;
#[macro_use]
pub extern crate error_chain;
#[macro_use]
pub extern crate clap;
#[macro_use]
pub extern crate yaml_rust;
use yaml_rust::yaml::Yaml;

use clap::{App,ArgMatches};
use clap::YamlLoader;
use std::path::Path;
use std::cell::RefCell;
use std::thread;
use std::sync::{Mutex,MutexGuard};




// IMPORTANT THAT CONFIG VALUES ARE SET BEFORE USE, OR THE VALUE OF THE CONST WIL BE FIXED TO THE DEFAULT.
lazy_static! {
  pub static ref YAML_CONTENT               : &'static str        = include_str!("cli.yml");
  pub static ref YAML                       : Yaml                = YamlLoader::load_from_str(*YAML_CONTENT).unwrap()[0].clone();
}



/////////////////////////////////////////////////////////////////////////////////////////////////
// Program Start
/////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
  // Program Initialisation
  init(&*YAML,load_inputs);

  println!("VPR project name: {:?}",  &(*MODULE_NAME).as_str());
  // info_println!("Config file : {:?}", &user_config);
  info_println!("Using blif file : {:?}", &(*BLIF_FILE).as_str());
  info_println!("Using place file: {:?}", &(*PLACE_FILE).as_str());
  info_println!("Using route file: {:?}", &(*ROUTE_FILE).as_str());

  //Load the placement file into the Block matrix.
  let (N,netlist_file,arch_file,place) = match parse_place_file(Path::new((*PLACE_FILE).as_str())) {
    Ok(data) => {
      data
    },
    Err(e) => panic!("Could not load placement file : {}",e.to_string())
  };

  println!("Netlist file used : {}", &netlist_file);
  println!("Architecture file used : {}", &arch_file);
  println!("LUT array width : {}", &N);

//  let blif_file = String::from(&*BLIF_FILE.as_str()) + ".pre-vpr";
//  let blif_file = String::from(&*BLIF_FILE.as_str());

  //Load the blif file into the block matrix
  let blif = match parse_blif_file(Path::new((*BLIF_FILE).as_str())){
    Ok(data) => data,
    Err(e) => panic!("Could not load blif file : {}",e.to_string())
  };

  //Load the routing file into the Nets matrix.
  //contains SOURCE -> SINK descriptions
  let (N2,nets) = match parse_route_file(Path::new((*ROUTE_FILE).as_str())) {
    Ok(n) => n,
    Err(e) => panic!("Could not load route file : {}",e.to_string())
  };


  match (*MATCHES).occurrences_of("verbose") {
    2 => {
      println!("PLACEMENT FILE DATA: ");
      for p in place.iter() {
        println!("{:?}",p); //dont pretty print this.
      }
      println!("ROUTING FILE DATA: ");
      for r in &nets {
        println!("{:#?}",r);
      }
      println!("BLIF FILE DATA: ");
      for b in &blif {
        println!("{:#?}",b);
      }
    },
    _ => println!("No verbose info")
  }

  // If we specified the multiple() setting we can get all the values
  if let Some(in_debug) = (*MATCHES).values_of("debug") {
    for debug_option in in_debug {
      println!("Debug option enabled : {}", debug_option);
      match debug_option{
        "c" | "config" => {
          println!("TODO : CONFIG FILE DATA: ");
        },
        "b" | "blif" => {
          println!("BLIF FILE DATA: ");

          for b in &blif {
            println!("{:#?}",b);
          }
          thread::spawn(move || {
            GL_CONFIG.lock().unwrap().loglevel_blif = true;
          }).join().expect("thread::spawn failed");
        },
        "p" | "place" => {
          println!("PLACEMENT FILE DATA: ");
          for p in place.iter() {
            println!("{:?}",p); //dont pretty print this.
          }
           thread::spawn(move || {
            GL_CONFIG.lock().unwrap().loglevel_place = true;
          }).join().expect("thread::spawn failed");
        },
        "r" | "route" => {
          println!("ROUTING FILE DATA: ");
          for r in &nets {
            println!("{:#?}",r);
          }
          thread::spawn(move || {
            GL_CONFIG.lock().unwrap().loglevel_route = true;
          }).join().expect("thread::spawn failed");
        },
        "s" | "stream" => {
          thread::spawn(move || {
            GL_CONFIG.lock().unwrap().loglevel_bitstream = true;
          }).join().expect("thread::spawn failed");

        },
        _ => break

      }
    }
  }


  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Problem Initialisation
  /////////////////////////////////////////////////////////////////////////////////////////////////
  thread::spawn(move || {
    let mut config : MutexGuard<Config> = GL_CONFIG.lock().unwrap();

    config.arch_file                                      = arch_file;
    config.fpga_width                                     = N+1;
    config.channel_width                                  = 4; //set these from user config file
    config.k_lut                                          = 3; //set these from user config file
    config.grid_width                                     = 4; //set these from user config file
    config.n_rail                                         = 2; //set these from user config file
    config.ble_local_clk_en_index                         = 1; //set these from user config file
    config.ble_global_clk_en_index                        = 0; //set these from user config file
    config.ble_clk_en_size                                = 2;
  }).join().expect("thread::spawn failed");



  //from the config file, build the Tile Matrix, setting the correct types that wil determine
  // which mapping process' to follow.

  info_println!("(*N_TILES as usize) = {}",(*N_TILES as usize)); //todo: make the N_TILES unlock and continue..
  info_println!("finished generating sink bitstreams");

  use SwitchBlockType::*;
  let mut tiles : Vec<Vec<Tile>> = Vec::with_capacity(*N_TILES as usize);
  for y in 0..(*N_TILES as usize) {
    let mut y_row : Vec<Tile> = Vec::with_capacity(*N_TILES as usize);
    for x in 0..(*N_TILES as usize){
      let mut tile = Tile{
        xy : Point(x as u16,y as u16),
        sw_blk: WiltonSwitchBlockBitstream::new(),
        con_blk_top: Vec::new(),
        con_blk_right: Vec::new(),
        ble: Vec::new(),
      };
      tile.sw_blk.ref_mut().resize((3*4*(*CH_WIDTH)/2) as usize, false);
      tile.con_blk_top.resize((4*(*CH_WIDTH)*(*N_RAIL)) as usize, false);
      tile.con_blk_right.resize((4*(*CH_WIDTH)*(*N_RAIL)) as usize, false);
      tile.ble.resize( (*BLE_ADDR_SIZE + *BLE_CLK_CTRL_SIZE) as usize, false);
      tile.set_ble_clk_mode(ClockMode::Async);
      y_row.push(tile);

    }
    tiles.push(y_row);
  }
  info_println!("finished setting up tile matrix");



  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Generate the bitstream :
  /////////////////////////////////////////////////////////////////////////////////////////////////
  //todo : to pre-generate the tile matrix or not to pre-generate te tile matrix, that is the question.
  info_println!("Bitstream generation start");
  build_bitstream(&mut tiles,&nets,&blif,&place);

  if let Some(in_debug) = (*MATCHES).values_of("debug") {
    for debug_option in in_debug {
      match debug_option{
        "a" | "all" => {
          println!("Debug option enabled : {}", debug_option);
          println!("TODO : ALL DATA: ");

        },
        "t" | "tile" => {
          println!("Debug option enabled : {}", debug_option);
          println!("TILE DATA: ");
          for t in &tiles {
            println!("{:#?}",t);
          }
        },
        _ => break

      }
    }
  }

  

  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Output the bitstream
  /////////////////////////////////////////////////////////////////////////////////////////////////
  println!("Output bitstream");
  output_bitstream(&tiles).expect("Could not create bit stream file : {}");


  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Output FPGA pad mapping (Actually pad mapping can be read by PERL and can find&replace template values.)
  /////////////////////////////////////////////////////////////////////////////////////////////////
  // println!("Output bitstream");
  // output_bitstream(&tiles).expect("Could not create bit stream file : {}");
  for model in blif {
    //create file
    //output port to grid location mapping;
     
  } 


}
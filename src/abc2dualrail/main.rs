
#[macro_use]
extern crate vpr_extra;

pub use vpr_extra::errors::*;
pub use vpr_extra::types::*;
pub use vpr_extra::parse::*;
pub use vpr_extra::global::*;

mod model;
mod input;
mod output;

use model::*;
use model::NoDangle;
use input::*;
use output::*;

use std::path::Path;
use std::cell::RefCell;
use std::thread;
use std::sync::{Mutex,MutexGuard};

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




// IMPORTANT THAT CONFIG VALUES ARE SET BEFORE USE, OR THE VALUE OF THE CONST WIL BE FIXED TO THE DEFAULT.
lazy_static! {
  pub static ref YAML_CONTENT               : &'static str        = include_str!("cli.yml"); //todo: find  a way to make a static compile time runable function
  pub static ref YAML                       : Yaml                = YamlLoader::load_from_str(*YAML_CONTENT).unwrap()[0].clone();
}

/////////////////////////////////////////////////////////////////////////////////////////////////
// Program Start
/////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {
  /////////////////////////////////////////////////////////////////////////////////////////////////
  // Program Initialisation
  /////////////////////////////////////////////////////////////////////////////////////////////////
  // Global Config Initialisation
  setup_global_context(&*YAML);
  
  // Set logging levels.
  init_logging(); 

  // Store input files globally.
  set_input_files(); 


  // (issue that cuased this to move here ): 
  // the blif file parsing requires the knowledge of the size of the LUT. This might be avoided if a scan 
  // is done during parsing to determine the lut size. it might differ from the requisted size, and should then be 
  // translated to the requisted size AFTER it has been parsed.
  /////////////////////////////////////////////////////////////////////////////////////////////////
  // Problem Setup Initialisation 
  /////////////////////////////////////////////////////////////////////////////////////////////////
  thread::spawn(move || {
    let mut config: MutexGuard<Config> = GL_CONFIG.lock().unwrap();
    let matches = (*MATCHES).clone();

    if let Ok(lut_k) = value_t!(matches, "lut_k", u16){
      config.lut_k = lut_k;
    }else{
      config.lut_k = 3; //set these from user config file
    }
  }).join().expect("thread::spawn failed");

  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Load the blif file into the block matrix
  /////////////////////////////////////////////////////////////////////////////////////////////////
  let mut blif = match parse_blif_file(Path::new((*BLIF_FILE).as_str())) {
    Ok(data) => data,
    Err(e) => panic!("Could not load blif file : {}", e.to_string())
  };

  /////////////////////////////////////////////////////////////////////////////////////////////////
  // Problem Setup Finalisation
  /////////////////////////////////////////////////////////////////////////////////////////////////


  /////////////////////////////////////////////////////////////////////////////////////////////////
  // Problem Solve
  /////////////////////////////////////////////////////////////////////////////////////////////////
  info_println!("Bitstream generation start");
  
  let blif_out = match (*MATCHES).value_of("output") {
    Some(blif) => blif.to_owned(), //test if it exists.
    None => String::from(&*MODULE_NAME.as_str()) + ".abc.dual.blif" // or dual.blif
  };
  
  // connect unconnected input ports
  for mut model in &mut blif{
    model.connect_dangling_input_ports();
  }

  //Output to blif file.
  println!("Outputting to blif file : {:?}",&blif_out);
  output_to_blif(&blif_out,&blif);


  // debug print
  if let Some(in_debug) = (*MATCHES).values_of("debug") {
    for debug_option in in_debug {
      match debug_option{
        "b" | "blif" if *INFO => {
          println!("\n\nDebug option enabled : {}", debug_option);
          println!("BLIF FILE DATA: ");
          for b in &blif {
            println!("{:#?}",b);
          }
        },
        _ => break

      }
    }
  }
}
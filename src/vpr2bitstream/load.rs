
use vpr_extra::global::*;
// use std::path::Path;
use std::process;
use std::path::{Path, PathBuf};

pub fn load_inputs(){
  // Gets a value for config if supplied by user, or defaults to "default.conf"
  println!("Loading Input Files\n");

  // let user_config = (*MATCHES).value_of("config").unwrap_or("default.conf");

  let vpr_name = match (*MATCHES).value_of("INPUT"){
    Some(matched) => matched.to_owned(),
    None          => String::from("none")
  };
  // thread::spawn(move || {

  let input_file = format!("{}", vpr_name.to_owned() + ".v");
  println!("Input File: {}", &input_file);

  let input_path = PathBuf::from(input_file);

  if !input_path.exists(){
    panic!("No Verilog input file found ({}).", vpr_name.to_owned() + ".v");
  }

  println!("parent : {:?}", Path::parent(&input_path));
  let output_path = match Path::parent(&input_path){
    Some(path) => PathBuf::from(path),
    None => panic!("could not parse ({:?}).",&input_path)
  };
  

  let blif_in = match (*MATCHES).value_of("blif"){
    Some(matched) => matched.to_owned(),
    None          => vpr_name.to_owned()
  };

  let route_in = match (*MATCHES).value_of("route"){
    Some(matched) => matched.to_owned(),
    None          => vpr_name.to_owned()
  };

  let place_in = match (*MATCHES).value_of("place"){
    Some(matched) => matched.to_owned(),
    None          => vpr_name.to_owned()
  };

  let bit_out = match (*MATCHES).value_of("bit_out"){
    Some(matched) => matched.to_owned(),
    None          => vpr_name.to_owned()
  };

  let mut blif_file   = format!("{}{}",&blif_in,".pre-vpr.blif");
  let mut place_file  = format!("{}{}",&place_in,".place");
  let mut route_file  = format!("{}{}",&route_in,".route");
  let mut bit_file    = format!("{}{}",&bit_out,".bs");



  thread::spawn(move || {
    let mut config : MutexGuard<Config> = GL_CONFIG.lock().unwrap();
    config.module_name  = vpr_name;
    config.output_path  = Some(output_path.to_owned());
    config.blif_file    = blif_file;
    config.place_file   = place_file;
    config.route_file   = route_file;
    config.bit_file     = bit_file;

  }).join().expect("thread::spawn failed");

}
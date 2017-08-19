
use vpr_extra::global::*;
// use std::path::Path;
use std::process;
use std::path::{Path, PathBuf};

pub fn parse_command_line(){
  // Gets a value for config if supplied by user, or defaults to "default.conf"
  info_println!("Loading Input Files\n");



  let mut config : MutexGuard<Config> = GL_CONFIG.lock().unwrap();  
  let matches = &*MATCHES;

  let vpr_name = match matches.value_of("INPUT"){
    Some(matched) => matched.to_owned(),
    None          => String::from("none")
  };

  let input_file = format!("{}", vpr_name.to_owned() + ".v");
  vv_info_println!("Input File: {}", &input_file);

  let input_path = PathBuf::from(&input_file);

  if !input_path.exists(){
    panic!("No Verilog input file found ({}).", vpr_name.to_owned() + ".v");
  }

  vv_info_println!("parent : {:?}", Path::parent(&input_path));
  
  let output_path = match Path::parent(&input_path){
    Some(path) => PathBuf::from(path),
    None => panic!("could not parse ({:?}).",&input_path)
  };
  

  let blif_in = match matches.value_of("blif"){
    Some(matched) => matched.to_owned(),
    None          => vpr_name.to_owned()
  };

  let route_in = match matches.value_of("route"){
    Some(matched) => matched.to_owned(),
    None          => vpr_name.to_owned()
  };

  let place_in = match matches.value_of("place"){
    Some(matched) => matched.to_owned(),
    None          => vpr_name.to_owned()
  };

  let bit_out = match matches.value_of("bit_out"){
    Some(matched) => matched.to_owned(),
    None          => vpr_name.to_owned()
  };

  if let Ok(channel_width) = value_t!(matches, "channel_width", u16){
    config.channel_width = channel_width;
  }


  if let Ok(lut_k) = value_t!(matches, "lut_k", u16){
    config.lut_k = lut_k;
  }else{
    println!("{}", "NO LUT SIZE SPECIFIED.");
    config.lut_k = 3; //set these from user config file
  }

  let mut blif_file   = format!("{}{}",&blif_in,".pre-vpr.blif");
  let mut place_file  = format!("{}{}",&place_in,".place");
  let mut route_file  = format!("{}{}",&route_in,".route");
  let mut bit_file    = format!("{}{}",&bit_out,".bs");

  // info_println!("Using Config file : {:?}", &user_config);
  info_println!("Using VPR project name: {:?}",  &vpr_name);
  info_println!("Using blif file : {:?}", &blif_file);
  info_println!("Using place file: {:?}", &place_file);
  info_println!("Using route file: {:?}", &route_file);
  info_println!("Using bitstream file: {:?}", &bit_file);

  config.module_name      = vpr_name;
  config.output_path      = Some(output_path.to_owned());
  config.blif_file        = blif_file;
  config.place_file       = place_file;
  config.route_file       = route_file;
  config.bit_file         = bit_file;

}
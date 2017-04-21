
use vpr_extra::global::*;

pub fn load_inputs(){

  // Gets a value for config if supplied by user, or defaults to "default.conf"
  let user_config = (*MATCHES).value_of("config").unwrap_or("default.conf");

  let vpr_name = match (*MATCHES).value_of("INPUT"){
    Some(matched) => matched.to_owned(),
    None          => String::from("unspecified")
  };

  let blif_file = match (*MATCHES).value_of("blif"){
    Some(matched) => matched.to_owned(),
    None          => vpr_name.to_owned() + ".abc.blif"
  };

  // let mut blif_file   = format!("{}{}",&blif_file,".blif");


  println!("VPR project name: {:?}", &vpr_name);
  println!("Config file : {:?}", &user_config);
  println!("Using blif file : {:?}", &blif_file);

  thread::spawn(move || {
    let mut config : MutexGuard<Config> = GL_CONFIG.lock().unwrap();
    config.module_name  = vpr_name;
    config.blif_file    = blif_file;
  }).join().expect("thread::spawn failed");

}
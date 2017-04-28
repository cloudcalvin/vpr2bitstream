
use vpr_extra::global::*;

pub fn load_inputs(){
  // Gets a value for config if supplied by user, or defaults to "default.conf"
  println!("Loading Input Files\n");

  // let user_config = (*MATCHES).value_of("config").unwrap_or("default.conf");

  let vpr_name = match (*MATCHES).value_of("INPUT"){
    Some(matched) => matched.to_owned(),
    None          => String::from("none")
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
    config.blif_file    = blif_file;
    config.place_file   = place_file;
    config.route_file   = route_file;
    config.bit_file     = bit_file;

  }).join().expect("thread::spawn failed");

}
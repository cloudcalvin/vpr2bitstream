use types::*;
//use load::*;

//use std::sync::Arc;

pub use std::thread;
pub use std::sync::{Mutex,MutexGuard};

use clap::{App, ArgMatches, YamlLoader};

use yaml_rust::yaml::Yaml;


#[derive(Default, Debug)]
pub struct Config {
  pub loglevel_info: bool,
  pub loglevel_log: bool,
  pub loglevel_warn: bool,
  pub loglevel_debug: bool,
  pub loglevel_blif: bool,
  pub loglevel_route: bool,
  pub loglevel_place: bool,
  pub loglevel_bitstream: bool,
  pub arg_matches: ArgMatches<'static>,
  pub blif_file: String,
  pub route_file: String,
  pub place_file: String,
  pub arch_file: String,
  pub bit_file: String,
  pub module_name: String,
  pub channel_width: u16,
  pub grid_width: u16,
  pub fpga_width: u16,
  pub n_rail: u16,
  pub k_lut: u16,
  pub ble_clk_en_size: u16,
  pub ble_local_clk_en_index : u16,
  pub ble_global_clk_en_index :u16,
  //  pub switchblock_type : Arc<Box<SwitchBlock + 'static>>
  //  pub switchblock_type : &'static SwitchBlock
}



// IMPORTANT THAT CONFIG VALUES ARE SET BEFORE USE, OR THE VALUE OF THE CONST WIL BE FIXED TO THE DEFAULT.
lazy_static! {
  pub static ref GL_CONFIG: Mutex<Config> =  Mutex::new(Config::default());

  pub static ref INFO                       : bool                = GL_CONFIG.lock().unwrap().loglevel_info;
  pub static ref LOG                        : bool                = GL_CONFIG.lock().unwrap().loglevel_log;
  pub static ref WARN                       : bool                = GL_CONFIG.lock().unwrap().loglevel_warn;
  pub static ref DEBUG                      : bool                = GL_CONFIG.lock().unwrap().loglevel_debug;
  pub static ref BLIF_DEBUG                 : bool                = GL_CONFIG.lock().unwrap().loglevel_blif;
  pub static ref ROUTE_DEBUG                : bool                = GL_CONFIG.lock().unwrap().loglevel_route;
  pub static ref PLACE_DEBUG                : bool                = GL_CONFIG.lock().unwrap().loglevel_place;
  pub static ref BITSTREAM                  : bool                = GL_CONFIG.lock().unwrap().loglevel_bitstream;


//  pub static ref YAML_CONTENT               : &'static str        = include_str!("cli.yml");
//  pub static ref YAML                       : Yaml                = YamlLoader::load_from_str(*YAML_CONTENT).unwrap()[0].clone();
  pub static ref MATCHES                    : ArgMatches<'static> = GL_CONFIG.lock().unwrap().arg_matches.clone();

  pub static ref MODULE_NAME                : String              = GL_CONFIG.lock().unwrap().module_name.clone();
  pub static ref BLIF_FILE                  : String              = GL_CONFIG.lock().unwrap().blif_file.clone();
  pub static ref ROUTE_FILE                 : String              = GL_CONFIG.lock().unwrap().route_file.clone();
  pub static ref PLACE_FILE                 : String              = GL_CONFIG.lock().unwrap().place_file.clone();
  pub static ref BIT_FILE                   : String              = GL_CONFIG.lock().unwrap().bit_file.clone();

  pub static ref N_RAIL                     : u16                 = GL_CONFIG.lock().unwrap().n_rail;
  pub static ref K_LUT                      : u16                 = GL_CONFIG.lock().unwrap().k_lut;
  pub static ref LUT_SIZE                   : u16                 = (2 as u16).pow(*K_LUT as u32);


  pub static ref N_TILES                    : u16                 = GL_CONFIG.lock().unwrap().fpga_width;
  pub static ref N_TRACKS                   : u16                 = GL_CONFIG.lock().unwrap().channel_width;
  pub static ref CH_WIDTH                   : u16                 = GL_CONFIG.lock().unwrap().channel_width;
  pub static ref CH_MAX                     : u16                 = (*CH_WIDTH) - 1;
  
  pub static ref CB_TOP_IN_IDX              : u16                 = (*CH_WIDTH) * 3;
  pub static ref CB_BOT_IN_IDX              : u16                 = (*CH_WIDTH) * 2;
  pub static ref CB_TOP_OUT_IDX             : u16                 = (*CH_WIDTH);
  pub static ref CB_BOT_OUT_IDX             : u16                 = 0;


  pub static ref SB_TOP_IDX                 : u16                 = (*CH_WIDTH) * 3;
  pub static ref SB_LEFT_IDX                : u16                 = (*CH_WIDTH) * 2;
  pub static ref SB_BOT_IDX                 : u16                 = (*CH_WIDTH);
  pub static ref SB_RIGHT_IDX               : u16                 = 0;

//  pub static ref SW_BLK_ENUM_TYPE           : SwitchBlockType = GL_CONFIG.lock().unwrap().switchblock_type;
//  pub static ref SW_BLK_TYPE                : SwitchBlockBitstream = GL_CONFIG.lock().unwrap().switchblock_type;
//  pub static ref SW_BLK_TYPE                : SwitchBlockBitstream = SwitchBlockType::get_struct((*SW_BLK_TYPE));
//  pub static ref SW_BLK_TYPE                : &'static SwitchBlockBitstream = &WiltonSwitchBlockBitstream;

  // pub static ref BLE_ADDR_SIZE              : u16                 = (2 as u16).pow(*K_LUT as u32);
  // pub static ref BLE_CLK_CTRL_SIZE          : u16                 = GL_CONFIG.lock().unwrap().ble_clk_en_size;

  // pub static ref BLE_CLK_OFFSET_LOCAL       : u16                 = GL_CONFIG.lock().unwrap().ble_local_clk_en_index;
  // pub static ref BLE_CLK_OFFSET_GLOBAL      : u16                 = GL_CONFIG.lock().unwrap().ble_global_clk_en_index;
}
lazy_static!{
  pub static ref BLE_ADDR_SIZE              : u16                 = 2u16.pow(*K_LUT as u32);
  pub static ref BLE_CLK_CTRL_SIZE          : u16                 = GL_CONFIG.lock().unwrap().ble_clk_en_size;

  pub static ref BLE_CLK_OFFSET_LOCAL       : u16                 = GL_CONFIG.lock().unwrap().ble_local_clk_en_index;
  pub static ref BLE_CLK_OFFSET_GLOBAL      : u16                 = GL_CONFIG.lock().unwrap().ble_global_clk_en_index;

}


pub fn setup_global_context(yaml : &'static Yaml) {
  debug_println!("content of file : >{:#?}<",yaml);

  let mut config : MutexGuard<Config> = GL_CONFIG.lock().unwrap();

  let matches : ArgMatches<'static> =  App::from_yaml(yaml).get_matches();
  config.arg_matches = matches;

}
pub fn init(yaml : &'static Yaml, load : fn() -> ()){
  debug_println!("create global context");
  setup_global_context(yaml);

  debug_println!("load input files");
  load();

  debug_println!("set verbose level");
  thread::spawn(move || {
    let mut config : MutexGuard<Config> = GL_CONFIG.lock().unwrap();
    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    debug_println!("occurences of v : {} ", (*MATCHES).occurrences_of("verbose").to_string());

    match (*MATCHES).occurrences_of("verbose") {
      0 => {
        println!("No verbose info")
      },
      1 => {
        config.loglevel_info = true;
        println!("Some verbose info")
      },
      2 => {
        config.loglevel_info = true;
        config.loglevel_debug = true;
        println!("Tons of verbose info")
      },
      3 | _ => println!("Don't be crazy"),
    }
  }).join().expect("thread::spawn failed");

}


//#[macro_export]
//macro_rules! debug_println {
//  () => (if *DEBUG {print!("\n")});
//  ($fmt:expr) => (if *DEBUG {print!(concat!($fmt, "\n"))});
//  ($fmt:expr, $($arg:tt)*) => (if *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
//}
//#[macro_export]
//macro_rules! info_println {
//  () => (if *INFO {print!("\n")});
//  ($fmt:expr) => (if *INFO {print!(concat!($fmt, "\n"))});
//  ($fmt:expr, $($arg:tt)*) => (if *INFO {print!(concat!($fmt, "\n"), $($arg)*)});
//}
//#[macro_export]
//macro_rules! bits_println {
//  () => (if *BITSTREAM {print!("\n")});
//  ($fmt:expr) => (if *BITSTREAM {print!(concat!($fmt, "\n"))});
//  ($fmt:expr, $($arg:tt)*) => (if *BITSTREAM {print!(concat!($fmt, "\n"), $($arg)*)});
//}
//

//
//pub struct Session<S> {
//  store: Arc<S>
//}
//
//impl<S: SwitchBlock> Session<S> {
//  fn new(store: S) -> Session<S> {
//    Session {
//      store: Arc::new(store)
//    }
//  }
//}
//
//trait SessionStore: Sync + Clone {
//  fn select_session(&mut self) -> Session<K, Self> {
//    Session::new(self.clone())
//  }
//}




//pub static GL_CONFIG: Arc<RefCell<types::Config> = RefCell::new(Config::default());

//static CLI_YAML: &'static Yaml = load_yaml!("cli.yml");
//'static CLI_YAML : YamlLoader = load_yaml!("cli.yml");
//static YAML_FILE: &'static str = "cli.yml";
//lazy_static!{

//  pub static ref YAML                       : &'static Yaml = load_yaml!("cli.yml");

//  pub static ref YAML_FILE                  : &'static str  = "cli.yml";
//  pub static ref YAML                       : Yaml = Yaml::Integer(0);

////  static ref YAML_FILE: &'static str = "cli.yml";
////  pub static ref YAML : &'static Yaml = load_yaml!(format!("{}", YAML_FILE));
//  pub static ref MATCHES : ArgMatches<'static> = |&'static Yaml| App::from_yaml(&yml).get_matches();
//}

//thread_local!(pub static GL_CONFIG: RefCell<Option<types::Config>> = RefCell::new(Some(types::Config::default())));
//thread_local!(pub static GL_CONFIG: Option<types::Config> = RefCell::new(Some(types::Config::default())));
//thread_local!(pub static CONFIG_MAP: HashMap<Config> = HashMap::new());




//  let gl = GL_CONFIG.map(||{
//
//  });
//  lazy_static!{
//    static ref (*CH_WIDTH ): u16 = gl.channel_width;
//    static ref FPGA_WIDTH : u16 = gl.fpga_width;

//  }
//static (*CH_WIDTH ): usize = GL_CONFIG.lock().unwrap().channel_width as usize;
//static RIGHT : &'static fn() -> usize = |(x,y)| match x {
//  (*CH_WIDTH )=> true,
//  _ => false
//};

//use std::sync::{Once, ONCE_INIT};
//
//static mut VAL: usize = 0;
//static INIT: Once = ONCE_INIT;
//
//// Accessing a `static mut` is unsafe much of the time, but if we do so
//// in a synchronized fashion (e.g. write once or read all) then we're
//// good to go!
////
//// This function will only call `expensive_computation` once, and will
//// otherwise always return the value returned from the first invocation.
//fn globals() -> usize {
//  unsafe {
//    INIT.call_once(|| {
//      let gl = GL_CONFIG.lock().unwrap();
//      (*CH_WIDTH       ): u16 = GL_CONFIG.lock().unwrap().channel_width;
//      CH_MAX         : u16 = gl.channel_width - 1;
//      CB_TOP_IN_IDX  : u16 = gl.channel_width * 3u16 ;
//      CB_BOT_IN_IDX  : u16 = gl.channel_width * 2u16 ;
//      CB_TOP_OUT_IDX : u16 = gl.channel_width;
//      CB_BOT_OUT_IDX : u16 = 0 ;
//    });
//    GL_CONFIG.lock().unwrap()
//  }
//}
//
//fn expensive_computation() -> usize {
//  // ...
//}
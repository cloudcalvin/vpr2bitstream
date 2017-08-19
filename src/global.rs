#[macro_use]
pub use logging::*;

use types::*;
//use load::*;

pub use std::thread;
pub use std::sync::{Mutex,MutexGuard};
//use std::sync::Arc;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub use ansi_term::Colour::*;

use clap::{App, ArgMatches, YamlLoader};

use yaml_rust::yaml::Yaml;


#[derive(Default, Debug)]
pub struct Context {
  // Config - config
  pub arg_matches: ArgMatches<'static>,
  
  // LOGGING
  pub loglevel_info: bool,
  pub loglevel_log: bool,
  pub loglevel_warn: bool,
  pub loglevel_debug: bool,
  pub loglevel_dump: bool,

  pub loglevel_blif: bool,
  pub loglevel_route: bool,
  pub loglevel_place: bool,
  pub loglevel_bitstream: bool,
  pub loglevel_timing: bool,

}
#[derive(Default, Debug)]
pub struct Config {

  // PROJECT CONFIG
  pub output_path: Option<PathBuf>,
  pub parameter_store: HashMap<String,String>,
  pub dual_rail_mode: bool,
  
  // >>> THESE SHOULD NOT BE LIBRARY GLOBALS?
  pub blif_file: String,
  pub route_file: String,
  pub place_file: String,
  pub arch_file: Option<String>,
  pub bit_file: String, // <<< ACTUALLY WANT INHERENTENCE TO ADD ONLY THESE..
  pub port_map_file: String, // <<<
  
  // FPGA ARCH CONFIG // <<< ACTUALLY WANT HASHMAPS OR SOMETHING FOR THESE.
  pub module_name: String,
  pub channel_width: u16,
  pub grid_width: u16,
  pub fpga_width: u16,
  pub n_rail: u16,
  pub lut_k: u16,
  pub ble_clk_en_size: u16,
  pub ble_local_clk_en_index : u16,
  pub ble_global_clk_en_index :u16,

  // FPGA TIMINGS
  pub fpga_delay_pad_exit: u32,      
  pub fpga_delay_ble_exit: u32,      
  pub fpga_delay_ble_enter: u32,     
  pub fpga_delay_pad_enter: u32,     
  pub fpga_delay_trackswitch: u32,

  //  pub switchblock_type : Arc<Box<SwitchBlock + 'static>>
  //  pub switchblock_type : &'static SwitchBlock
  // <<<
}

// IMPORTANT THAT CONFIG VALUES ARE SET BEFORE USE, OR THE VALUE OF THE CONST WIL BE FIXED TO THE DEFAULT.
// cloning these might seem like a bad idea, but actually it only clones them the first time they are used. (I THINK)
lazy_static! {
  pub static ref GL_CONFIG: Mutex<Config> =  Mutex::new(Config::default());
  pub static ref GL_CONTEXT: Mutex<Context> =  Mutex::new(Context::default());

  pub static ref MATCHES                    : ArgMatches<'static> = GL_CONTEXT.lock().unwrap().arg_matches.clone();
//  pub static ref YAML_CONTENT               : &'static str        = include_str!("cli.yml");
//  pub static ref YAML                       : Yaml                = YamlLoader::load_from_str(*YAML_CONTENT).unwrap()[0].clone();
}

lazy_static! {

  pub static ref INFO                       : bool                = GL_CONTEXT.lock().unwrap().loglevel_info;
  pub static ref LOG                        : bool                = GL_CONTEXT.lock().unwrap().loglevel_log;
  pub static ref WARN                       : bool                = GL_CONTEXT.lock().unwrap().loglevel_warn;
  pub static ref DEBUG                      : bool                = GL_CONTEXT.lock().unwrap().loglevel_debug;
  pub static ref BLIF_DEBUG                 : bool                = GL_CONTEXT.lock().unwrap().loglevel_blif;
  pub static ref ROUTE_DEBUG                : bool                = GL_CONTEXT.lock().unwrap().loglevel_route;
  pub static ref PLACE_DEBUG                : bool                = GL_CONTEXT.lock().unwrap().loglevel_place;
  pub static ref BITSTREAM                  : bool                = GL_CONTEXT.lock().unwrap().loglevel_bitstream;
  pub static ref TIMING_DEBUG               : bool                = GL_CONTEXT.lock().unwrap().loglevel_timing;

}
lazy_static! {
  pub static ref PARAMETERS                 : HashMap<String, String> = GL_CONFIG.lock().unwrap().parameter_store.clone();

  pub static ref MODULE_NAME                : String              = GL_CONFIG.lock().unwrap().module_name.clone();
  pub static ref OUTPUT_PATH                : PathBuf             = GL_CONFIG.lock().unwrap().output_path.clone().unwrap();
  pub static ref BLIF_FILE                  : String              = GL_CONFIG.lock().unwrap().blif_file.clone();
  pub static ref ROUTE_FILE                 : String              = GL_CONFIG.lock().unwrap().route_file.clone();
  pub static ref PLACE_FILE                 : String              = GL_CONFIG.lock().unwrap().place_file.clone();
  pub static ref BIT_FILE                   : String              = GL_CONFIG.lock().unwrap().bit_file.clone();
  
}
lazy_static!{

  pub static ref DUALRAIL_MODE              : bool                = GL_CONFIG.lock().unwrap().dual_rail_mode;

  pub static ref N_RAIL                     : u16                 = GL_CONFIG.lock().unwrap().n_rail;
  pub static ref LUT_K                      : u16                 = GL_CONFIG.lock().unwrap().lut_k;
  pub static ref LUT_SIZE                   : u16                 = (2 as u16).pow(*LUT_K as u32);

  pub static ref N_TILES                    : u16                 = GL_CONFIG.lock().unwrap().fpga_width;
  pub static ref N_TRACKS                   : u16                 = GL_CONFIG.lock().unwrap().channel_width;
  pub static ref CH_WIDTH                   : u16                 = GL_CONFIG.lock().unwrap().channel_width;
  pub static ref CH_MAX                     : u16                 = (*CH_WIDTH) - 1;

  pub static ref FPGA_TOP_IDX               : u16                 = (*N_TILES) * 3;
  pub static ref FPGA_LEFT_IDX              : u16                 = (*N_TILES) * 2;
  pub static ref FPGA_BOT_IDX               : u16                 = (*N_TILES);
  pub static ref FPGA_RIGHT_IDX             : u16                 = 0;

  pub static ref FPGA_DELAY_PAD_EXIT        : u32                 = GL_CONFIG.lock().unwrap().fpga_delay_pad_exit   ;
  pub static ref FPGA_DELAY_BLE_EXIT        : u32                 = GL_CONFIG.lock().unwrap().fpga_delay_ble_exit   ;
  pub static ref FPGA_DELAY_BLE_ENTER       : u32                 = GL_CONFIG.lock().unwrap().fpga_delay_ble_enter  ;
  pub static ref FPGA_DELAY_PAD_ENTER       : u32                 = GL_CONFIG.lock().unwrap().fpga_delay_pad_enter  ;
  pub static ref FPGA_DELAY_TRACKSWITCH     : u32                 = GL_CONFIG.lock().unwrap().fpga_delay_trackswitch;

  pub static ref BLE_ADDR_SIZE              : u16                 = 2u16.pow(*LUT_K as u32);
  pub static ref BLE_CLK_CTRL_SIZE          : u16                 = GL_CONFIG.lock().unwrap().ble_clk_en_size;

  pub static ref BLE_CLK_OFFSET_LOCAL       : u16                 = GL_CONFIG.lock().unwrap().ble_local_clk_en_index;
  pub static ref BLE_CLK_OFFSET_GLOBAL      : u16                 = GL_CONFIG.lock().unwrap().ble_global_clk_en_index;

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

  // pub static ref BLE_ADDR_SIZE              : u16                 = (2 as u16).pow(*LUT_K as u32);
  // pub static ref BLE_CLK_CTRL_SIZE          : u16                 = GL_CONFIG.lock().unwrap().ble_clk_en_size;

  // pub static ref BLE_CLK_OFFSET_LOCAL       : u16                 = GL_CONFIG.lock().unwrap().ble_local_clk_en_index;
  // pub static ref BLE_CLK_OFFSET_GLOBAL      : u16                 = GL_CONFIG.lock().unwrap().ble_global_clk_en_index;

}



pub fn setup_global_context(yaml : &'static Yaml) {
  thread::spawn(move || {
    
    let mut context : MutexGuard<Context> = GL_CONTEXT.lock().unwrap();

    let matches : ArgMatches<'static> =  App::from_yaml(yaml).get_matches();
    context.arg_matches = matches;

  }).join().expect("thread::spawn failed");

  //this is to reduce changes of deadlock;
  let matches = &*MATCHES;
  match matches.occurrences_of("verbose"){
    3 => {
      thread::spawn(move || {
        println!{"{:#?}",*MATCHES}
      }).join().expect("thread::spawn failed");
    },
    _ => ()
  }  
}
// pub fn init(yaml : &'static Yaml, load : fn() -> ()){
//   setup_global_context(yaml);
//   load();

//   // thread::spawn(move || {
//     let mut config : MutexGuard<Config> = GL_CONFIG.lock().unwrap();
//     // Vary the output based on how many times the user used the "verbose" flag
//     // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'

//   // }).join().expect("thread::spawn failed");

// }


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
use types::*;
use std::sync::Mutex;
//use types::Config;


#[derive(Default, Debug, Builder)]
pub struct Config{
  pub module_name: String,
  pub channel_width: u16,
  pub lut_k: u8,
  pub grid_width: u32,
  pub fpga_width : u32,
}


// IMPORTANT THAT CONFIG VALUES ARE SET BEFORE USE, OR THE VALUE OF THE CONST WIL BE FIXED TO THE DEFAULT.
lazy_static! {
  pub static ref GL_CONFIG: Mutex<Config> =  Mutex::new(Config::default());
  pub static ref FPGA_SW_BLK_TYPE : SwitchBlockType = SwitchBlockType::Wilton;


  pub static ref N_TILES        : u32 = GL_CONFIG.lock().unwrap().fpga_width; //todo make u16
  pub static ref N_TRACKS       : u16 = GL_CONFIG.lock().unwrap().channel_width;
  pub static ref CH_WIDTH       : u16 = GL_CONFIG.lock().unwrap().channel_width;
  pub static ref CH_MAX         : u16 = (*CH_WIDTH) - 1;
  pub static ref CB_TOP_IN_IDX  : u16 = (*CH_WIDTH) * 3;
  pub static ref CB_BOT_IN_IDX  : u16 = (*CH_WIDTH) * 2;
  pub static ref CB_TOP_OUT_IDX : u16 = (*CH_WIDTH);
  pub static ref CB_BOT_OUT_IDX : u16 = 0;


  pub static ref SB_TOP_IDX     : u16 = (*CH_WIDTH) * 3;
  pub static ref SB_LEFT_IDX    : u16 = (*CH_WIDTH) * 2;
  pub static ref SB_BOT_IDX     : u16 = (*CH_WIDTH);
  pub static ref SB_RIGHT_IDX   : u16 = 0;

}






//pub static GL_CONFIG: Arc<RefCell<types::Config> = RefCell::new(Config::default());

//static CLI_YAML: &'static Yaml = load_yaml!("cli.yml");
//'static CLI_YAML : YamlLoader = load_yaml!("cli.yml");
//static YAML_FILE: &'static str = "cli.yml";
//lazy_static!{
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
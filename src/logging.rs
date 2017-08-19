use global::*;
use errors::*;
use types::*;
use ::clap;
#[macro_export]
macro_rules! info_println {
    () => (if *INFO {print!("\n")});
    ($fmt:expr) => (if *INFO {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *INFO {print!(concat!($fmt, "\n"), $($arg)*)});
}

#[macro_export]
macro_rules! vv_info_println {
    () => (if *DEBUG {print!("\n")});
    ($fmt:expr) => (if *DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}

#[macro_export]
macro_rules! debug_println {
    () => (if *DEBUG {print!("\n")});
    ($fmt:expr) => (if *DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}

#[macro_export]
macro_rules! vv_debug_println {
    () => (if *DEBUG {print!("\n")});
    ($fmt:expr) => (if *DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}


#[macro_export]
macro_rules! bits_println {
    () => (if *BITSTREAM  {print!("\n")});
    ($fmt:expr) => (if *BITSTREAM {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *BITSTREAM {print!(concat!($fmt, "\n"), $($arg)*)});
}

#[macro_export]
macro_rules! vv_bits_println {
    () => (if *BITSTREAM & *DEBUG {print!("\n")});
    ($fmt:expr) => (if *BITSTREAM & *DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *BITSTREAM & *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}

#[macro_export]
macro_rules! blif_println {
    () => (if *BLIF_DEBUG  {print!("\n")});
    ($fmt:expr) => (if *BLIF_DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *BLIF_DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}
#[macro_export]
macro_rules! vv_blif_println {
    () => (if *BLIF_DEBUG & *DEBUG {print!("\n")});
    ($fmt:expr) => (if *BLIF_DEBUG & *DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *BLIF_DEBUG & *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}

#[macro_export]
macro_rules! route_println {
    () => (if *ROUTE_DEBUG {print!("\n")});
    ($fmt:expr) => (if *ROUTE_DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *ROUTE_DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}
#[macro_export]
macro_rules! vv_route_println {
    () => (if *ROUTE_DEBUG & *DEBUG {print!("\n")});
    ($fmt:expr) => (if *ROUTE_DEBUG & *DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *ROUTE_DEBUG & *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}

#[macro_export]
macro_rules! place_println {
    () => (if *PLACE_DEBUG {print!("\n")});
    ($fmt:expr) => (if *PLACE_DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *PLACE_DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}
#[macro_export]
macro_rules! vv_place_println {
    () => (if *PLACE_DEBUG & *DEBUG {print!("\n")});
    ($fmt:expr) => (if *PLACE_DEBUG & *DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *PLACE_DEBUG & *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}

#[macro_export]
macro_rules! timing_println {
    () => (if *TIMING_DEBUG  {print!("\n")});
    ($fmt:expr) => (if *TIMING_DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *TIMING_DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}
#[macro_export]
macro_rules! vv_timing_println {
    () => (if *TIMING_DEBUG & *DEBUG {print!("\n")});
    ($fmt:expr) => (if *TIMING_DEBUG & *DEBUG {print!(concat!($fmt, "\n"))});
    ($fmt:expr, $($arg:tt)*) => (if *TIMING_DEBUG & *DEBUG {print!(concat!($fmt, "\n"), $($arg)*)});
}


pub fn init_logging(){

  let mut context : MutexGuard<Context> = GL_CONTEXT.lock().unwrap();

  match context.arg_matches.occurrences_of("verbose").clone() {

    1 => {
      context.loglevel_info = true;
    },
    2 => {
      context.loglevel_info = true;
      context.loglevel_debug = true;
    },
    3 => {
      context.loglevel_info = true;
      context.loglevel_debug = true;
      context.loglevel_dump = true;
    }
    _ => ()
  }


  if context.loglevel_info {
    println!("Info logging enabled.");
  }

  if context.loglevel_debug {
    println!("Verbose logging enabled.");
  }

  if context.loglevel_dump {
    // println!("Data dump logging enabled.");
    println!("Very verbose logging enabled.");
  }

  // let debug_matches = c;
  use clap::{Values,ArgMatches};
  let matches = context.arg_matches.clone();
  if let Ok(debug_options) = values_t!(matches.values_of("debug"), String){
    for option in &debug_options {
      match option.as_str(){
        "c" | "config" => {
          println!("TODO : CONFIG FILE DATA: ");
        },
        "b" | "blif" => {
          println!("BLIF DEBUGGING ENABLED : ");
          context.loglevel_blif = true;
        },
        "p" | "place" => {
          context.loglevel_place = true;
        },
        "r" | "route" => {
          context.loglevel_route = true;
        },
        "s" | "stream" => {
          context.loglevel_bitstream = true;
        },
        "cp" | "crit_path" => {
          context.loglevel_timing = true;
        },
        _ => continue
      }   
    }
  }

  // thread::spawn(move || {
  //   let debug_flags = vec!{ 
  //     &*INFO
  //     &*LOG
  //     &*WARN
  //     &*DEBUG
  //     &*BLIF_DEBUG
  //     &*ROUTE_DEBUG
  //     &*PLACE_DEBUG
  //     &*BITSTREAM
  //     &*TIMING_DEBUG
  //   };

  //   // // let mut dummy = debug_flags.iter().fold(0 |sum, x| sum + x);
  //   // let mut dummy : u16 ;
  //   // for flag in debug_flags{
  //   //   dummy += 1;
  //   //   if dummy == _ {
  //   //     println!("Flag : {:#?}",flag)
  //   //   }else{
  //   //     ()
  //   //   }
  //   // }
  // }).join().expect("thread::spawn failed");
  
}

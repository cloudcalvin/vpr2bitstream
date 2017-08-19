
#[macro_use]
use global::*;
use errors::*;
use types::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::prelude::*;
use std::path::PathBuf;

// use std::path::Path;

// pub fn load_parameters(config : MutexGuard<Config>, matches){
pub fn load_parameters() -> Result<()>{
  let mut config : MutexGuard<Config> = GL_CONFIG.lock().expect("Config mutex lock could not be aquired.");
  // let ref matches = (*MATCHES);
  //TODO : url like parameter directories. with relative and absolute/global parameter(hierarchical)
  // value_t!(matches, "length", u32).unwrap_or_else(|e| e.exit());
    // If we specified the multiple() setting we can get all the values

  // config.fpga_delay_pad_exit            = value_t!(matches, "fpga_delay_pad_exit", u32)?;
  // config.fpga_delay_ble_exit            = value_t!(matches, "fpga_delay_ble_exit", u32)?;
  // config.fpga_delay_ble_enter           = value_t!(matches, "fpga_delay_ble_enter", u32)?;
  // config.fpga_delay_pad_enter           = value_t!(matches, "fpga_delay_pad_enter", u32)?;
  // config.fpga_delay_trackswitch         = value_t!(matches, "fpga_delay_trackswitch", u32)?;
  let mut parameters = HashMap::new();

  if let Some(param_argv) =  (*MATCHES).values_of("parameter") {

    for arg in param_argv {
      lazy_static!{
        static ref RE_PARAMETER:  Regex = Regex::new(r"(?:parameter\s+)?([[:graph:]]+)\s*?=\s*?([[:graph:]]+[^;])\s*?;?")
        // static ref RE_PARAMETER:  Regex = Regex::new(r"([[:graph:]]+)\s*?=\s*?([[:graph:]]+[^;])\s*?;?")
          .expect("Parameter extraction regex error");
      }

      let maybe_arg_path =  PathBuf::from(arg);
      debug_println!("Parsing parameter : '{:?}'",arg);

      if RE_PARAMETER.is_match(arg){
        println!("hello danie");
        let captured: Option<Captures> = RE_PARAMETER.captures(arg); //captures, executes the regex query defined in 'util.rs'
        println!("{}",Red.bold().paint(format!("{:#?}",captured)));
        match captured {
          
          Some(ref cap) if cap.len() == 3 => {
          println!("hello sanni");
            
            let name = try!(cap.get(1)
                .ok_or::<Error>("Regex Failure".into()))
                .as_str();
            let value = try!(cap.get(2)
                .ok_or::<Error>("Regex Failure".into()))
                .as_str();
            println!("Found parameter {:?} with value : {:?}.", &name, &value);
            parameters.insert(String::from(name),String::from(value));

          },
          _ => bail!("Malformed parameter file. ({:})",&arg)
        }
      }
      else
      if maybe_arg_path.exists(){
        
        //Read File into Buffer
        let fh = try!(File::open(maybe_arg_path));
        let mut content = BufReader::new(&fh);
        //  let mut lines_enumerated = file.lines().enumerate();
        let mut lines = content.lines();

        for line in lines{
          let line = line?;
          let captured: Option<Captures> = RE_PARAMETER.captures(&line); //captures, executes the regex query defined in 'util.rs'
          // println!("here : len : {:}",captured.as_ref().expect("no capture").len());
          vv_debug_println!("{:#?}",&captured);
          match &captured {
            &Some(ref cap) if cap.len() == 3 => {
              let name = try!(cap.get(1)
                  .ok_or::<Error>("Regex Failure".into()))
                  .as_str();

              let value = try!(cap.get(2)
                  .ok_or::<Error>("Regex Failure".into()))
                  .as_str();
              vv_debug_println!("Found parameter {:?} with value : {:?}.", &name, &value);          
              parameters.insert(String::from(name),String::from(value));
            },
            _ => bail!("Malformed parameter file. ({:})",&line)
          }
        }
      }else{
        //un recognised parameter definition format.
        bail!("Unable to read parameter or file from input: (\"{:}\")", arg)
      }
    }

  }
  config.parameter_store = parameters;
  Ok(())
}

pub fn write_parameters(file_path : PathBuf, local : Delay, global : Delay) -> Result<()>{


  // let mut file = File::create(&(*PARAMETER_OUT))?;
  let mut file = File::create(&file_path)?;
  let p = &*PARAMETERS;


  let global_param_name = match p.get("SYNC_DELAY_PARAM_NAME"){
    Some(x) => x,
    None => "GLOBAL_CRIT_PATH_DELAY"
  };
  let local_param_name = match p.get("ASYNC_DELAY_PARAM_NAME"){
    Some(x) => x,
    None => "LOCAL_CRIT_PATH_DELAY"
  };
  // let mut global_param_name = *SYNC_DELAY_PARAM_NAME;
  // let mut local_param_name = *ASYNC_DELAY_PARAM_NAME;

  // if global_param_name.is_empty() {
  //   global_param_name = "GLOBAL_CRIT_PATH_DELAY";
  // }
  
  // if local_param_name.is_empty() {
  //   local_param_name = "LOCAL_CRIT_PATH_DELAY";
  // }

  // let global_param_name = if p.get("ASYNC_DELAY_PARAM_NAME")

  let global_param_line = format!("parameter {:} = {:};\n",global_param_name,global);
  let local_param_line = format!("parameter {:} = {:};\n",local_param_name,local);

  let out = global_param_line + local_param_line.as_str();
  file.write_all(&out.as_str().as_bytes());

  Ok(())
}

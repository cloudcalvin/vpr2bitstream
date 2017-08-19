#[macro_use]
extern crate vpr_extra;

#[macro_use]
use vpr_extra::global::*;
use vpr_extra::errors::*;
use vpr_extra::types::*;
use vpr_extra::parameter::*;
use vpr_extra::parse::*;
use vpr_extra::bitstream::*;
use vpr_extra::timing::*;

mod input;
use input::*;

mod output;
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
  pub static ref YAML_CONTENT  : &'static str = include_str!("cli.yml");
  pub static ref YAML          : Yaml         = YamlLoader::load_from_str(*YAML_CONTENT).unwrap()[0].clone();
}



/////////////////////////////////////////////////////////////////////////////////////////////////
// Program Start
/////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {


  // Global Config Initialisation
  setup_global_context(&*YAML);

  // Set logging levels.
  vpr_extra::global::init_logging();

  // Store input files globally.
  parse_command_line();

  //Load the placement file into the Block matrix.
  let (N,netlist_file,arch_file,place,place_map) = match parse_place_file(Path::new((*PLACE_FILE).as_str())) {
    Ok(data) => {
      data
    },
    Err(e) => panic!("Could not load placement file : {}",e.to_string())
  };

  //Load the blif file into the block matrix
  let blif = match parse_blif_file(Path::new((*BLIF_FILE).as_str())){
    Ok(data) => data,
    Err(e) => panic!("Could not load blif file : {}",e.to_string())
  };

  //Load the routing file into the Nets matrix.
  let routing_content = parse_route_file(Path::new((*ROUTE_FILE).as_str()));

  let (N2,temp_place_file, rr_nets, gl_nets) : (u16, Option<String>, Vec<RouteNet>,Vec<GlobalNet>) = match routing_content {
    Ok(n) => n,
    Err(e) => panic!("Could not load route file : {}",e.to_string())
  };

  vv_blif_println!("BLIF FILE DATA: {:#?}",&blif);
  vv_place_println!("PLACE FILE DATA: {:#?}",&rr_nets);
  vv_route_println!("ROUTE FILE DATA: {:#?}",&place);



          // println!("BLIF FILE DATA: ");

          // for b in &blif {
          //   println!("{:#?}",b);
          // }
          //           for p in place.iter() {
          //   println!("{:?}",p); //dont pretty print this.
          // }
  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Problem Initialisation
  /////////////////////////////////////////////////////////////////////////////////////////////////

  info_println!("Netlist file used : {}", &netlist_file);

  if let Some(ref file) = arch_file{
    info_println!("Architecture file used : {}", file);
  }
  info_println!("LUT array width : {}", &N);

  let dual_rail_mode = match (*MATCHES).occurrences_of("dual_rail") {
    0 => { info_println!{"Single rail mode enabled"}; false },
    _ => { info_println!{"Dual rail mode enabled"}; true }
  };


  thread::spawn(move || {
    let mut config : MutexGuard<Config> = GL_CONFIG.lock().unwrap();

    config.dual_rail_mode                 = dual_rail_mode;
    config.arch_file                      = arch_file;
    config.fpga_width                     = N+1;
    // config.k_lut                          = 3; //set these from user config file
    config.grid_width                     = 4; //set these from user config file
    config.n_rail                         = if dual_rail_mode { 2 } else { 1 };
    config.ble_local_clk_en_index         = 1; //set these from user config file
    config.ble_global_clk_en_index        = 0; //set these from user config file
    config.ble_clk_en_size                = 2; //set these from user config file

  }).join().expect("thread::spawn failed");



  match load_parameters(){
    Ok(_) => info_println!("Parameters loaded successfully"),
    _ => panic!{"Could not load parameters."}
  }

  //from the config file, build the Tile Matrix, setting the correct types that wil determine
  // which mapping process' to follow.

  vv_info_println!("(*N_TILES as usize) = {}",(*N_TILES as usize)); //todo: make the N_TILES unlock and continue..

  use SwitchBlockType::*;
  let mut tiles : Vec<Vec<Tile>> = Vec::with_capacity(*N_TILES as usize);
  for y in 0..(*N_TILES as usize) {
    let mut y_row : Vec<Tile> = Vec::with_capacity(*N_TILES as usize);
    for x in 0..(*N_TILES as usize){
      let mut tile = Tile{
        xy : Point(x as u16,y as u16),
        sw_blk: WiltonSwitchBlockBitstream::new(),
        con_blk_top: Vec::new(),
        con_blk_right: Vec::new(),
        ble: Vec::new(),
      };
      tile.sw_blk.ref_mut().resize((3*4*(*CH_WIDTH)/2) as usize, false);
      tile.con_blk_top.resize((4*(*CH_WIDTH)*(*N_RAIL)) as usize, false);
      tile.con_blk_right.resize((4*(*CH_WIDTH)*(*N_RAIL)) as usize, false);
      tile.ble.resize( (*BLE_ADDR_SIZE + *BLE_CLK_CTRL_SIZE) as usize, false);
      tile.set_ble_clk_mode(ClockMode::Async);
      y_row.push(tile);

    }
    tiles.push(y_row);
  }
  vv_info_println!("finished setting up tile matrix");
  // log!("asdfa" if $verbose)


  match (*MATCHES).occurrences_of("verbose") {
    3 => {
      println!("DUMPING CONFIG: ");
      println!(">\n{:#?}\n<", *GL_CONFIG.lock().unwrap() ); //dont pretty print this.

    },
    _ => ()
  }
  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Generate the bitstream :
  /////////////////////////////////////////////////////////////////////////////////////////////////
  //todo : to pre-generate the tile matrix or not to pre-generate te tile matrix, that is the question.
  info_println!("\nGenerating Bitstream.");
  build_bitstream(&mut tiles,&rr_nets,&blif,&place);
  info_println!("finished generating bitstream");

  if let Some(in_debug) = (*MATCHES).values_of("debug") {
    for debug_option in in_debug {
      match debug_option{
        "a" | "all" => {
          println!("Debug option enabled : {}", debug_option);
          println!("TODO : ALL DATA: ");

        },
        "t" | "tile" => {
          println!("Debug option enabled : {}", debug_option);
          println!("TILE DATA: ");
          for t in &tiles {
            println!("{:#?}",t);
          }
        },
        _ => break

      }
    }
  }



  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Output the bitstream
  /////////////////////////////////////////////////////////////////////////////////////////////////
  info_println!("\nOutputting bitstream to file.");
  output_bitstream(&tiles).expect("Could not create bit stream file : {}");



  /////////////////////////////////////////////////////////////////////////////////////////////////
  //Output FPGA pad mapping (Actually pad mapping can be read by PERL and can find&replace template values.)
  /////////////////////////////////////////////////////////////////////////////////////////////////
  if let Some(mapping_file) = (*MATCHES).value_of("map") {
    info_println!("Port map files enabled.");
    // println!("Output bitstream");
    // output_bitstream(&tiles).expect("Could not create bit stream file : {}");
    output_port_map(&blif,&place).expect("Could not create port map file : {}")
  }

  /////////////////////////////////////////////////////////////////////////////////////////////////
  // Calculate Critical paths.
  /////////////////////////////////////////////////////////////////////////////////////////////////
  use std::fs;
  if (*MATCHES).is_present("timing") {
    // let output_path = (*OUTPUT_PATH);

    let timing_file = if let Some(timing_file) = (*MATCHES).value_of("timing") {
      let mut output_path = &(*OUTPUT_PATH);
      output_path.join(timing_file)
      // output_path
    }else{
      let mut output_path = &(*OUTPUT_PATH);
      output_path.join("parameter.tmp")
      // output_path
    };
    println!("writing prameters to : {}",timing_file.as_os_str().to_str().expect("Parameter file path contains illigal characters."));
    info_println!("Calculate critical paths enabled.");

    // let mut files = vec!{};

    // for file in timing_file {
    //   files.push(file);
    //   println!("DUMPING : {:#?}",file);

    //   // let s = value_t()
    //   // match file{
    //   //   s @ String =>  {
    //   //     files.push(s);
    //   //     println!("DUMPING : {:#?}",s);
    //   //   },
    //   //   _ => ()
    //   // }
    // }
    // let parameter_out = match files.get(1){
    //   Some(f) => String::from(*f),
    //   _ =>
    // };


    let delay_event_graph = build_delay_event_graph(&blif,&rr_nets,&place_map)
      .expect("Failed to create event graph : {}"); //to extract routing events. Could not

    let inter_ble_crit_path_delay = get_inter_ble_critical_path_delay(&delay_event_graph)
      .expect("Failed to determine critical paths");

    let inter_latch_crit_path_delay = match *DUALRAIL_MODE{
      true => {
          get_async_inter_latch_crit_path_delay(&delay_event_graph,&place_map)
            .expect("Failed to determine critical paths")
      },
      false => {
          get_sync_inter_latch_crit_path_delay(inter_ble_crit_path_delay,&delay_event_graph,&place_map)
            .expect("Failed to determine critical paths")
      }
    };
    // info_println!("delay_event_graph : {:#?}",delay_event_graph);
    // info_println!("delay_event_graph : {:#?}",delay_event_graph);
    let mut cp_count = 0;
    // netlist -> sfq-gate-synthesis -> netlist timing analysis -> calculate critical path delay for sfq pulses.
    if let Some(in_debug) = (*MATCHES).values_of("debug") {
      for debug_option in in_debug {
        match debug_option{
          "cp" => {
            cp_count += 1;
          },
          _ => break
        }
      }
    }
    if cp_count >= 2{
      println!("Post Critical Path Delay Calculations");
      debug_println!("delay_event_graph : {:#?}",delay_event_graph);
    }
    info_println!("Inter-BLE critical path delay : {}",inter_ble_crit_path_delay);
    info_println!("Inter-latch critical path delay : {}",inter_latch_crit_path_delay);

    write_parameters(timing_file,inter_ble_crit_path_delay,inter_latch_crit_path_delay).expect("Could not output timing parameters.");

  }


  info_println!("OK")
}

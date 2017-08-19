
#[macro_use]
use vpr_extra::global::*;
use vpr_extra::parse::*;
use vpr_extra::errors::*;
use vpr_extra::types::*;
use vpr_extra::types::PortFlow::*;
use vpr_extra::types::XY::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use vpr_extra::chrono::prelude::*;

/// # OUTPUT THE BITSTREAM TO FILE.
///
///
///
pub fn output_bitstream<'a>(tiles : &'a Vec<Vec<Tile>>) -> Result<()>{
  //TODO : generate the bitstream in the order that the chip is programmed.

  let mut file = File::create(&(*BIT_FILE))?;

  //  for i in tiles.len()..0 {
  //    for j in tiles.len()..0{
  for i in 0..tiles.len(){
    for j in 0..tiles.len(){
      let mut bitstream : String = tiles[j as usize][((*N_TILES - 1)-i as u16) as usize]
          .bitstream()
          .iter().cloned().rev()
          .map(|x| if x==true {"1"} else {"0"} )
          .fold(String::new(), |acc,b| acc + b);

      file.write_all(&bitstream.as_bytes()).unwrap();
      //      debug_println!("{:?}",bitstream);
      bits_println!("{:?} @ {:?}",bitstream,(((*N_TILES - 1)-i as u16),j));
    }
  }
  Ok(())
}

/// # OUTPUT PORTMAP FILE
///
///
pub fn output_port_map<'a>(models : &'a Vec<Model>, place : &Placement ) -> Result<()>{
  
  for model in models{
    let mut file_path = (*OUTPUT_PATH).clone();
    file_path.push(Path::new(&model.name));
    file_path.set_extension("pmf");
    
    // let file_path :Path  = (*OUTPUT_PATH).as_path().join(Path::new(&format!("{}",model.name.clone()+".pmf"))).as_path();
    println!("Creating port map file : {:?}",&file_path);
    let mut file = File::create(&file_path)?;
    let mut string = String::new();

    //input ports
    for port in &model.inputs {
      let point = place.get(port).expect(&format!("could not find port ({})",port));
      if let Some(edge_port_index) = TileGrid::try_get_port_index(&point){        
        string.push_str(format!("{},pad {}\n",Model::trim_port(port).unwrap(),edge_port_index).as_str());
      }
    }
    
    //output ports
    for port in &model.outputs{
      let point = place.get(&format!("out:{}",port)).expect(&format!("could not find port ({})",port));
      if let Some(edge_port_index) = TileGrid::try_get_port_index(&point){
        println!("Output port : {}",port);
        
        string.push_str(format!("pad {},{}\n",edge_port_index,Model::trim_port(port).unwrap()).as_str());
      }
    }

    //write to file
    match file.write_all(&string.as_bytes()){
      Ok(_) => println!("Successfull output port map file."),
      Err(e) => panic!("Error in writing to port map file: ({:?})",file_path)
    }

  }
  
  Ok(())
}
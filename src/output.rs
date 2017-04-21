
#[macro_use]
use global::*;
use parse::*;
use errors::*;
use types::*;
use types::PortFlow::*;
use types::XY::*;
use std::fs::File;
use std::io::prelude::*;

use chrono::prelude::*;

/// # OUTPUT MODEL TO BLIF FILE.
///
///
///
pub fn output_to_blif<'a>(file_name : &str, models : &'a Vec<Model>) -> Result<()>{


  let mut file = File::create(&file_name)?;
  let mut out : String = String::new();
  for m in models{
    // out.push_str(&format!("# Dual rail \"{}\" written by abc2dualrail on {}\n",m.name,time));
    out.push_str(&format!(".model {}\n",m.name));
    out.push_str(&format!(".inputs "));
    for input in &m.inputs{
      out.push_str(&format!("{} ",input));
    }
    out.push_str(&format!("\n"));

    out.push_str(&format!(".outputs "));
    for output in &m.outputs{
      out.push_str(&format!("{} ",output));
    }
    out.push_str(&format!("\n"));

    // for latch in latced {
    //   out.push_str(&format!(".latch "));
    //   out.push_str(&format!("{}",latch));
    // }
    // out.push_str(&format!("\n"));

    for name in &m.logic {
      out.push_str(&format!(".names {}",name.to_blif()));
    }
    // out.push_str(&format!("\n.end\n"));

    // let dt = Local::now();
    out.push_str(&format!(".end"));


//      let mut bitstream : String = tiles[j as usize][((*N_TILES - 1)-i as u16) as usize]
//          .bitstream()
//          .iter().cloned().rev()
//          .map(|x| if x==true {"1"} else {"0"} )
//          .fold(String::new(), |acc,b| acc + b);
//
     blif_println!("{:#?}",&out);
     try!(file.write_all(&out.as_bytes()));
//      println!("{:?} @ {:?}",bitstream,(((*N_TILES - 1)-i as u16),j));

  }




  Ok(())
}

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
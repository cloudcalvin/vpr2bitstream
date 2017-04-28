#[macro_use]
use global::*;

use std::sync::Mutex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::borrow::Cow;

pub use na::{Vector3, Rotation3};
pub use na::{Dynamic, MatrixArray, MatrixVec, DMatrix};

use self::PortFlow::*;
use types::XY::*;


//type BitArray = Vec<bool>;
//type BitMatrix = Vec<BitArray>;
//
//impl Index<u16> for BitArray{
//  type Output = bool;
//
//  fn index(&self, index: u16) -> &bool {
//    &self[index as usize]
//  }
//}
//
//impl Index<u16> for BitArray{
//  type Output = bool;
//
//  fn index(&self, index: u16) -> &bool {
//    &self[index as usize]
//  }
//}

//impl Index<u16> for BitArray {
//  type Output = bool;
//
//  fn index<'a>(&'a self, index: u16) -> &'a bool {
//    &self[index as usize]
//  }
//}
//
//impl IndexMut<u16> for BitArray {
//  fn index_mut<'a>(&'a mut self, index: Side) -> &'a mut bool {
//    &mut self[index as usize]
//  }
//}
//
//





#[derive(Default,Clone,Debug,PartialEq)]
pub struct Port(u16);

#[derive(Default,Clone,Debug,PartialEq)]
pub struct Point(pub u16,pub u16);


pub type MetaNumber = u16;
pub type ClassOrPad = MetaNumber;
pub type PinOrPad = MetaNumber;
//pub type Track = u32;
pub type Placement = HashMap<String, Point>;


#[derive(Debug)]
pub struct Source(pub Point,pub NodeMetaType, pub ClassOrPad,pub PinOrPad);


#[derive(Debug)]
pub struct Sink(pub Point, pub NodeMetaType, pub ClassOrPad, pub PinOrPad);




pub trait Positionable{

}
impl Positionable for Source{}
impl Positionable for Sink{}

// Channels instantiated during the read from placement file and used during bitstream generation..
#[derive(Debug)]
pub struct Track{
  pub id : u64,
  pub nr : MetaNumber,
  pub xy : Point,
  pub orientation : XY,
  //  pub track : u32,
}


#[derive(Debug)]
pub struct Route{
  pub tracks : Vec<Track>,
  pub sink : Option<Sink>,
  pub from_opin : bool,
}
//todo change the Route into a tuple called route because the data is actually also route.
//pub type Route = Vec<Track>;
//pub type RouteTree = (Source,Vec<(Route,Option<Sink>)>);

pub type RouteTree = Vec<Route>;

#[derive(Debug)]
pub enum Net{
  Global(NetGlobal),
  Local(NetLocal)
}

//pub trait Net{}
#[derive(Debug)]
pub struct NetLocal{
  pub name : String,
  pub src: Source,
//  pub net_type: NetType,
  pub route_tree : RouteTree, //each outer vec is a new sub source. each inner vec is a connection.
}
//impl Net for NetLocal{}
#[derive(Debug)]
pub struct NetGlobal;
//impl Net for NetGlobal{}

#[derive(Debug,PartialEq)]
pub enum XY {
  X,
  Y
}
#[derive(Debug,PartialEq)]
pub enum NodeType{
  Source,
  Sink,
  IPin,
  OPin,
  Chan(XY),
  Block
}
#[derive(Debug,PartialEq)]
pub enum NodeMetaType{
  Pin,
  Pad,
  Class,
  Track
}
#[derive(Debug)]
pub struct Node{
  pub node_nr : u64,
  pub node_type : NodeType,
  pub xy : Point,
  pub meta_type : NodeMetaType,
  pub meta_nr : u16,
}
#[derive(Debug)]
pub struct LogicBlock{ // aka : .names block
  pub inputs : Vec<String>,
  pub output : String,
//  pub latched : bool,
  pub truth_idxs : Vec<usize>,
  pub truth_table : Vec<bool>
}
impl LogicBlock{
  pub fn to_blif(&self) -> String{
    let mut blif = String::new();
    for input in &self.inputs {
      blif.push_str(&format!("{}{}",input," "));
    }
    blif.push_str(&format!("{}{}",self.output,"\n"));
    for i in &self.truth_idxs{
      let n = &format!("{:01$b}", i,*K_LUT as usize);
      blif.push_str(&format!("{}{}",n," 1\n"));
    }
    blif
  }
}

//#[derive(Default,Clone,Debug,PartialEq)]
//pub enum Logic{
//  True,
//  False,
//  DontCare
//}


#[derive(Debug)]
pub struct Model{
  pub name: String,
  pub inputs : Vec<String>,
  pub outputs : Vec<String>,
  pub latched : Vec<String>,
  pub logic : Vec<LogicBlock>,
}

// NOTE : the blocks are stored with xy coords as received from VTR.
#[derive(Builder, Default, Debug)]
pub struct Block{ //can be ble, or pad.
  pub id : u32,
  pub name : String,
  pub xy : Point,
  pub sub_blk : u8,
  pub input : Vec<Port>, //should be size k.
  pub output : Port, //can a block ever have more than one output port? maybe for AICs
  pub truth_table : Vec<bool> //index is the input to the lookup. Will need to bounds check the read+writes.
}
impl Block{
  pub fn switch(&self, a: Port, b:  Port){
    // rearrange the lut_table accordingly.
  }

//  pub fn try_get_edges(p : &Point) -> Cow<Vec<Option<Side>>> {

  pub fn try_get_edges(p : &Point) -> Vec<Option<Side>> {
    let &Point(x,y) = p;
    let mut sides = Vec::new();
    if x == (*N_TILES) {
      sides.push(Some(Side::Right));
    }
    if y == (*N_TILES){
      sides.push(Some(Side::Top));
    }
    if y == 0 {
      sides.push(Some(Side::Bottom));
    }
    if x == 0{
      sides.push(Some(Side::Left));
    }
//    sides.into_owned()
//    Cow::from(sides)
  sides
  }


}

//struct PortNr() //how would you change the bounds?
#[derive(Debug,PartialEq,Clone)]
pub enum Side{
  Right,
  Bottom,
  Left,
  Top
}
#[derive(Debug,PartialEq)]
pub enum PortFlow{
  RightIn,
  RightOut,
  BotIn,
  BotOut,
  LeftIn,
  LeftOut,
  TopIn,
  TopOut,
}
#[derive(Debug,PartialEq)]
pub enum FlowDir {
  In,
  Out
}

pub enum ClockMode{
  Async,
  Latched,
}


#[derive(Default, Debug, Builder)]
//#[builder(pattern="immutable")]
pub struct Tile{
  pub xy : Point,
//  pub sw_blk : Vec<bool>,         // 24'b111_111_111_111_111_111_111_111,
  pub sw_blk : SwitchBlockBitstream,         // 24'b111_111_111_111_111_111_111_111,
  pub con_blk_top: Vec<bool>,     // (4'b{top_in}, 4'b{bot_in}, 4'b{top_out}, 4'b{bot_out}) each bit x2 for dualrail.
  pub con_blk_right: Vec<bool>,   // (4'b{top_in}, 4'b{bot_in}, 4'b{top_out}, 4'b{bot_out}) each bit x2 for dualrail.
  pub ble : Vec<bool>,            // 2'b11   8'b0000_0000
}

impl Tile{ //todo : move tile impl away from the other types. Make this file declarative-only.. (maybe put this in the bitstream.rs?)
  pub fn bitstream(&self) -> Vec<bool> {
    let mut ret = Vec::new();
    ret.extend_from_slice(&self.ble);
    ret.extend_from_slice(&self.con_blk_top);
    ret.extend_from_slice(&self.sw_blk.get_bits());
    ret.extend_from_slice(&self.con_blk_right);
    return ret
  }

//  pub fn try_get_edges(&self) -> Vec<Option<Side>> {
//    let Point(x,y) = self.xy;
//
//    let sides = vec!{};
//    if x == (*N_TILES-1) {
//      sides.push(Some(Side::Right));
//    }
//    if y == (*N_TILES-1){
//      sides.push(Some(Side::Top));
//    }
//    if y == 0 {
//      sides.push(Some(Side::Bottom));
//    }
//    if x == 0{
//      sides.push(Some(Side::Left));
//    }
//    sides
//  }

  pub fn connect_track_to_sink(&self, track : &Track){

  }
  pub fn connect_source_to_track(&self, track : &Track){

  }

  pub fn connect_tracks(&mut self, in_track : &Track, out_track: &Track){
    //only sets to true. never to false?
    let (in_port,out_port) = (self.get_sw_blk_port(in_track),self.get_sw_blk_port(out_track)); // port mapping is a property of the tile.
//    self.sw_blk.0[*SW_BLK_TYPE::get_switchblock_bit_index(in_port, out_port)] = true;
//    let mut SwitchBlockBitstream(sw_blk_handle) = self.sw_blk;

    let ref mut sw_blk_handle = self.sw_blk.0;
    let sw_blk_index = WiltonSwitchBlockBitstream::get_connection_bit_index(in_port, out_port);
    route_println!("connecting port {} to port {} with SB index : {}",in_port,out_port,sw_blk_index);
    sw_blk_handle[sw_blk_index] = true;
  }

  pub fn set_ble_at(&mut self, index: usize){
    self.ble[index as usize] = true; //remember this is setting true at the index. thus 001 means set to true at idx 0.
  }

  pub fn set_ble_clk_mode(&mut self, mode : ClockMode){

    match mode{
      ClockMode::Async    => {
        self.set_ble_at((*BLE_ADDR_SIZE + *BLE_CLK_OFFSET_LOCAL) as usize)
        //should rather have it like this : 
        // &self.ble[get slice of entire clk switch], then mask the bits setting the local mode
      } ,
      ClockMode::Latched  => {
        self.set_ble_at((*BLE_ADDR_SIZE + *BLE_CLK_OFFSET_GLOBAL) as usize)        
      }
    }
  }

//  pub fn set_ble_binary(&mut self, value: usize){
//    self.ble = value; //first need to convert the value to true's.
//  }
  pub fn set_top_con_blk_at(&mut self, index: usize){
    match *N_RAIL {
      1 => self.con_blk_top[index as usize] = true,
      2 => {
        self.con_blk_top[index*2usize as usize] = true;
        self.con_blk_top[(index*2usize)+1usize as usize] = true;
      },
      _ => unreachable!()
    }

  }
  pub fn set_right_con_blk_at(&mut self, index: usize){
    match *N_RAIL {
      1 => self.con_blk_right[index as usize] = true,
      2 => {
        self.con_blk_right[index*2usize as usize] = true;
        self.con_blk_right[(index*2usize)+1usize as usize] = true;
      },
      _ => unreachable!()

    }
  }
  pub fn get_con_blk_section_mut(&mut self, side : PortFlow) -> &mut[bool]{

    use self::PortFlow::*;
    match side { //TODO : this is specific to tiles with 2 con_blk + sw_blk. what happens when i move away from single tile type?
      PortFlow::RightIn  => &mut self.con_blk_right[(*CB_TOP_IN_IDX as usize  )..(*CB_TOP_IN_IDX as usize)],    // todo : check upper index
      PortFlow::RightOut => &mut self.con_blk_right[(*CB_BOT_IN_IDX as usize  )..(*CB_TOP_IN_IDX as usize)],    // todo : check upper index
      PortFlow::LeftIn   => &mut self.con_blk_right[(*CB_TOP_OUT_IDX as usize )..(*CB_BOT_IN_IDX as usize)],   // todo : check upper index
      PortFlow::LeftOut  => &mut self.con_blk_right[(*CB_BOT_OUT_IDX as usize )..(*CB_TOP_OUT_IDX as usize)],  // todo : check upper index

      PortFlow::TopIn    => &mut self.con_blk_top[  (*CB_TOP_IN_IDX as usize  )..(*CB_TOP_IN_IDX as usize)],      // todo : check upper index
      PortFlow::TopOut   => &mut self.con_blk_top[  (*CB_BOT_IN_IDX as usize  )..(*CB_TOP_IN_IDX as usize)],      // todo : check upper index
      PortFlow::BotIn    => &mut self.con_blk_top[  (*CB_TOP_OUT_IDX as usize )..(*CB_BOT_IN_IDX as usize)],     // todo : check upper index
      PortFlow::BotOut   => &mut self.con_blk_top[  (*CB_BOT_OUT_IDX as usize )..(*CB_TOP_OUT_IDX as usize)],    // todo : check upper index
    }
  }
  pub fn get_con_blk_section(&self, side : PortFlow) -> &[bool]{

    use self::PortFlow::*;
    match side { //TODO : this is specific to tiles with 2 con_blk + sw_blk. what happens when i move away from single tile type?
      PortFlow::RightIn  => &self.con_blk_right[(*CB_TOP_IN_IDX as usize)..(*CB_TOP_IN_IDX as usize)],        // todo : check upper index
      PortFlow::RightOut => &self.con_blk_right[(*CB_BOT_IN_IDX as usize)..(*CB_TOP_IN_IDX as usize)],        // todo : check upper index
      PortFlow::LeftIn   => &self.con_blk_right[(*CB_TOP_OUT_IDX as usize)..(*CB_BOT_IN_IDX as usize)],       // todo : check upper index
      PortFlow::LeftOut  => &self.con_blk_right[(*CB_BOT_OUT_IDX as usize)..(*CB_TOP_OUT_IDX as usize)],      // todo : check upper index

      PortFlow::TopIn    => &self.con_blk_top[(*CB_TOP_IN_IDX as usize)..(*CB_TOP_IN_IDX as usize)],          // todo : check upper index
      PortFlow::TopOut   => &self.con_blk_top[(*CB_BOT_IN_IDX as usize)..(*CB_TOP_IN_IDX as usize)],          // todo : check upper index
      PortFlow::BotIn    => &self.con_blk_top[(*CB_TOP_OUT_IDX as usize)..(*CB_BOT_IN_IDX as usize)],         // todo : check upper index
      PortFlow::BotOut   => &self.con_blk_top[(*CB_BOT_OUT_IDX as usize)..(*CB_TOP_OUT_IDX as usize)],        // todo : check upper index
    }
  }

  fn get_sw_blk_port(&self, track : &Track) -> u16 {
    match track.orientation {
      X => {
        if track.xy == self.xy {
          //side 2
          (*SB_LEFT_IDX) + track.nr
        } else {
          //side 0
          (*SB_RIGHT_IDX) + ((*CH_MAX) - track.nr)
        }
      },
      Y => {
        if track.xy == self.xy {
          //side 1
          (*SB_BOT_IDX) + ((*CH_MAX) - track.nr)
        } else {
          //side 3
          (*SB_TOP_IDX) + track.nr
        }
      }
    }
  }
}

#[derive(Debug,PartialEq)]
pub enum ConnectionBlockType {
  SingleInputSide,
  DoubleInputSide
}
#[derive(Debug,PartialEq,Clone,Default)]
pub struct ConnectionBlockBitstream(pub Vec<bool>);


#[derive(Debug,PartialEq)]
pub enum SwitchBlockType {
  Wilton
}
//impl SwitchBlockType{
//  fn get_struct(&self) {
//   match self{
//     Wilton => WiltonSwitchBlockBitstream,
//   }
//  }
//}

#[derive(Debug,PartialEq,Clone,Default)]
pub struct SwitchBlockBitstream(pub Vec<bool>);

impl SwitchBlockBitstream{
  pub fn new() -> Self{
    Self::default()
  }
  pub fn ref_mut(&mut self) -> &mut Vec<bool>{
    &mut self.0
  }
  pub fn get_bits(&self) -> Vec<bool>{
//    self.0.as_slice()
    self.0.to_owned()
  }
  //  fn get_port_index(&self, track: Track) -> u16;
  pub fn as_slice(&self) -> &[bool]{
//    self.0.as_slice()
    self.0.as_slice()
  }
  pub fn as_mut_slice(&mut self) -> &[bool]{
    //    self.0.as_slice()
    self.0.as_mut_slice()
  }
}

pub type WiltonSwitchBlockBitstream = SwitchBlockBitstream;
//impl WiltonSwitchBlockBitstream{
//  pub fn new() -> SwitchBlockBitstream{
//    Self(vec![false;4*(*CH_WIDTH)])
//  }
//}

///    Given the nature of the unidirectional wilton SW block, only half of the port on each side is an input port..
///      every input does:
///        a right turn that goes from the current input-only(half-side/unidirectional-pair) index to the output port 6 unidirectional-pairs later
///        a left turn that goes out 2 pair's later.
///        a straight connection, that goes out based on the side that you are on..
///
///
///        for each of the output ports it does not really matter which way around the uni-pair is,
///        as the programming bits aren't affected..
///
///        Thus if CHX goes to CHY and CHY orientation is up, it must have gone out at the top of the SW-blk.
///        which means we set the uni-pair index you find with CHX port nr (left or right wil correspond to a unique uni-pair index)
///        and this is the position the true (or true,true for dualrail) needs to be put.
///
///
///
///             pair-index     port idx
///              ___6___7_
///      11     \         \0       0
///      10    5\         \        1
///      9      \         \1       2
///      8     4\_________\        3
///               3   2             ...
///               7 6 5 4
///
///
/// //todo : give example
///
///  // given the out port, which selects the in port switch you can figure out from the reference point how many bits to skip.
///  // if considering the uni-directional wilton, only considering input ports the reference port (the 0 port) is the right top
///  // port, and you gou around the block clockwise with fc==3, it means every port you skip is a 3 added to the switch index.
//pub trait SwitchBlock: Sync + Clone + Send where Self: ?Sized{
  pub trait SwitchBlock{

//  type SwitchType;
//  type TransferSchema; //or Rail
//  type TileType;

//  fn set_connect(&mut self, input_track : &Track, output_track : &Track);

  fn get_connection_bit_index(in_port: u16, out_port: u16) -> usize;

}


impl SwitchBlock for WiltonSwitchBlockBitstream{

//  type SwitchType = SwitchBlockType::Wilton;// cant store a type variant..
  //Todo :  a WiltonSwitch trait impl for a SwitchBlock struct..

//  fn set_connect(&mut self, input_track: &Track, output_track: &Track) {
//    self.sw_blk[Self::get_connection_bit_index(in_port, out_port)] = true;
//  }


  //This is a property of the wilton connection structure. (3bits per input for Fc==3)
  fn get_connection_bit_index(in_port: u16, out_port: u16) -> usize{

    let output_side = out_port/(*N_TRACKS) as u16; // should be rounded down(test that it does). side 0 is the rhs, and increases clockwise.
    let input_side = in_port/(*N_TRACKS) as u16; // should be rounded down(test that it does). side 0 is the rhs, and increases clockwise.
    debug_println!("in_port : {}",&in_port);
    debug_println!("out_port : {}",&out_port);
    debug_println!("input side : {}",input_side);
    debug_println!("output side : {}",output_side);
    //    let input_side = in_port/(*N_TRACKS) as u16;
    let bit =  if in_port > out_port{
      (input_side-output_side)-1
    }else{
      3-(output_side-input_side)
    };
    debug_println!("bit to write : {}",bit);
    // based on input track and output size, determine switch.
    let input_index = in_port / 2 as u16; //must round down..
    let bit_idx =  input_index*3 + bit;
    bit_idx as usize
  }

}



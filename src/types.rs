use std::cell::RefCell;


pub use na::{Vector3, Rotation3};
pub use na::{Dynamic, MatrixArray, MatrixVec, DMatrix};

use std::sync::Mutex;

lazy_static! {
//    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
    pub static ref GL_CONFIG: Mutex<Config> =  Mutex::new(Config::default());
}

//use ::GL_CONFIG;

#[derive(Default, Debug, Builder)]
pub struct Config{
  pub module_name: String,
  pub channel_width: u16,
  pub lut_k: u8,
  pub grid_width: u16,
  pub fpga_size : u16,
}

#[derive(Default,Clone,Debug)]
pub struct Port(u32);

#[derive(Default,Clone,Debug)]
pub struct Point(pub u32,pub u32);

pub type MetaNumber = u16;
pub type Class = MetaNumber;
pub type Pin = MetaNumber;
//pub type Track = u32;
pub type Placement = (String, Point);

#[derive(Debug)]
pub struct Source(pub Point,pub Class,pub Pin);

#[derive(Debug)]
pub struct Sink(pub Point,pub Class,pub Pin);

// Channels instantiated during the read from placement file and used during bitstream generation..
#[derive(Debug)]
pub struct Track{
  pub id : u32,
  pub nr : MetaNumber,
  pub xy : Point,
  pub orientation : XY,
  //  pub track : u32,
}


#[derive(Debug)]
pub struct Route{
  pub tracks : Vec<Track>,
  pub sink : Option<Sink>,
}
//todo change the Route into a tuple called route because the data is actually also route.
//pub type Route = Vec<Track>;
//pub type RouteTree = (Source,Vec<(Route,Option<Sink>)>);

pub type RouteTree = Vec<Route>;

#[derive(Debug)]
pub struct Net{
  pub name : String,
  pub src: Source,
  pub route_tree : RouteTree, //each outer vec is a new sub source. each inner vec is a connection.
}
#[derive(Debug,PartialEq)]
pub enum XY {
  X,
  Y
}
#[derive(Debug)]
pub enum NodeType{
  Source,
  Sink,
  IPin,
  OPin,
  Chan(XY),
}

#[derive(Debug)]
pub struct Node{
  pub node_nr : u32,
  pub node_type : NodeType,
  pub xy : Point,
  pub meta_nr : u16,
}
#[derive(Debug)]
pub struct LogicBlock{ // aka : .names block
  pub inputs : Vec<String>,
  pub output : String,
  pub truth_table : Vec<bool>
}

#[derive(Debug)]
pub struct Model{
  pub name: String,
  pub inputs : Vec<String>,
  pub outputs : Vec<String>,
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
}

//struct PortNr() //how would you change the bounds?

#[derive(Default, Debug, Builder)]
//#[builder(pattern="immutable")]
pub struct Tile{
//  pub conf : &mut Config;
  pub xy : Point,
  pub sw_blk : Vec<bool>,
  pub con_bkl_top : Vec<bool>,
  pub con_bkl_right : Vec<bool>,
  pub ble : Vec<bool>,
}

impl Tile{
  pub fn bitstream(&self) -> Vec<bool> {
    let mut ret = Vec::new();
    ret.extend_from_slice(&self.ble);
    ret.extend_from_slice(&self.con_bkl_top);
    ret.extend_from_slice(&self.sw_blk);
    ret.extend_from_slice(&self.con_bkl_right);
    return ret
  }
  pub fn set_sw_b_bits(&mut self, in_port: u16, out_port: u16){
    //only sets to true. never to false?
    self.sw_blk[Tile::get_switchblock_path_index(in_port, out_port)] = true;
  }

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
  ///
  ///
  ///  // given the out port, which selects the in port switch you can figure out from the reference point how many bits to skip.
  ///  // if considering the uni-directional wilton, only considering input ports the reference port (the 0 port) is the right top
  ///  // port, and you gou around the block clockwise with fc==3, it means every port you skip is a 3 added to the switch index.

  fn get_switchblock_path_index( in_port: u16, out_port: u16) -> usize{

      let side = out_port/GL_CONFIG.lock().unwrap().channel_width; // should be rounded down(test that it does). side 0 is the rhs, and increases clockwise.
      // based on input track and output size, determine switch.
      let input_index = in_port / 2 ;
      let bit_idx = input_index *side;
      bit_idx as usize
  }
}
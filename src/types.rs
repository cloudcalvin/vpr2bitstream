

pub use na::{Vector3, Rotation3};
pub use na::{Dynamic, MatrixArray, MatrixVec, DMatrix};


#[derive(Default,Clone,Debug)]
pub struct Port(u32);

#[derive(Default,Clone,Debug)]
pub struct Point(pub u32,pub u32);

pub type Class = u32;
pub type Pin = u32;
pub type Track = u32;
pub type Placement = (String, Point);

pub struct Source(pub Point,pub Class,pub Pin);

pub struct Sink(pub Point,pub Class,pub Pin);

pub struct Route{
  pub channels : Vec<Channel>,
  pub sink : Option<Sink>,
}

pub type RouteTree = Vec<Route>;

pub struct Net{
  pub name : String,
  pub src: Source,
  pub route_tree : RouteTree, //each outer vec is a new sub source. each inner vec is a connection.
}

pub enum XY {
  X,
  Y
}
pub enum NodeType{
  Source,
  Sink,
  IPin,
  OPin,
  Chan(XY),
}

pub struct Node{
  pub node_nr : u32,
  pub node_type : NodeType,
  pub xy : Point,
  pub meta_nr : u32,
}

//pub enum NodeAttr{
// Class,
// Pin,
// Track
//}


pub struct Model{
  pub name: String,
  pub inputs : Vec<String>,
  pub outputs : Vec<String>,
  pub logic : Vec<(Vec<String>, String)>,
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
  pub lut_table : Vec<bool> //index is the input to the lookup. Will need to bounds check the read+writes.
}
impl Block{
  pub fn switch(&self, a: Port, b:  Port){
    // rearrange the lut_table accordingly.
  }
}

// Channels instantiated during the read from placement file and used during bitstream generation..
pub struct Channel{
  pub id : u32,
  pub xy : Point,
  pub orientation : XY, //true is vertical. false is horizontal.
  pub track : u32,
}

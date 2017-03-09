
//#[derive(clone)]
#[derive(Default,Clone,Debug)]
pub struct Port(u32);
#[derive(Default,Clone,Debug)]
pub struct Point(pub u32,pub u32);

pub type Class = u32;
pub type Pin = u32;

pub struct Source(Point,Class,Pin);
pub struct Sink(Point,Class,Pin);




//Nets are populated from the .route file.
pub struct Net{
  src_loc : Source,
  src_pin : u8,
  route_tree : Vec<(Vec<(Point,Point)>,Sink)>, //each outer vec is a new sub source. each inner vec is a connection.
}

pub struct Model{
  name: String,
  inputs : Vec<String>,
  outputs : Vec<String>,
  logic : Vec<(Vec<String>, String)>,
}

/*
  why use a builder : because the block requires both blif and placement to be fully instantiated.
*/
// NOTE : the blocks are stored with xy coords as received from VTR.
#[derive(Builder, Default, Debug)]
//#[builder(pattern="owned")]
pub struct Block{ //can be ble, or pad.
  id : u32,
  name : String,
  xy : Point,
  sub_blk : u8,
  input : Vec<Port>, //should be size k.
  output : Port, //can a block ever have more than one output port? maybe for AICs
  //  lut_table : HashMap<u8, bool> //assuming luts never have more than 256 inputs.
  lut_table : Vec<bool> //index is the input to the lookup. Will need to bounds check the read+writes.
}
impl Block{
  fn switch(&self, a: Port, b:  Port){
    // rearrange the lut_table accordingly.
  }
}

// Channels instantiated during the read from placement file and used during bitstream generation..
pub struct Channel{
  id : u32,
  name : String,
  xy : Point,
  orientation : bool, //true is vertical. false is horizontal.
}
#[derive(Default, Debug)]
//#[builder(pattern="immutable")]
pub struct Tile{
  xy : Point,
  sw_blk : Vec<bool>,
  con_bkl_top : Vec<bool>,
  con_bkl_right : Vec<bool>,
  ble : Vec<bool>,
}

impl Tile{
  fn bitstream(&self) -> Vec<bool> {
    let mut ret = Vec::new();
    ret.extend_from_slice(&self.ble);
    ret.extend_from_slice(&self.con_bkl_top);
    ret.extend_from_slice(&self.sw_blk);
    ret.extend_from_slice(&self.con_bkl_right);
    return ret
  }
}

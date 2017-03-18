


/*
The reason this bitstream generation is done in its own :
  The idea is that we eventually want a verilog file or something that describes the memory layout of the
  fpga so that the stream generation can happen automatically from hardware(memory) description.
    (could also be an extended form of arch.xml (prog_frame.xml))

  The ideal workflow would be to design a synthesisable verilog design of the FPGA (maybe even with myHDL that generates V*).
  Then to isolate the programming frame and map the bits from the frame to the features described in the arch.xml
  file so that a prog memory model can be generated for which the v*hdl user-code can then be generated (using this tool).

  Question. How do you model the different ways the programming could work?
  It is probably infeasable to think you know can allow every type of prog frame topology.
  is that why you create a mapping model? how would that work?

  fpga_prog_frame(data_input[N:0], p_clk_in){

    Then in here the *way* in which the dev describes the mapping of data_input to the memory
    topology can help us to create the bitstream.

  }

*/

use types::*;
use parse::*;
use errors::*;


////*
///
///
///    Start with the routing. for each net, for each route, for each track(node)(This can be done extremely parallel)
///        {keeping the previous track/source around for reference}:
///        look up the channel_X[x][y]\channel_Y[x][y]up based on orientation Y or X.
///        Then set the appropriate bits  based on the side the reference source comes from(oor a look-ahead method), and if a sink, 'connect' the track to the pad/pin.
///        for every track-track connection a SW-block is also at play, thus calculate which SW, do a lookup[][] and
///        using the track-track mask (todo : set up boolean masks),
///        change the appropriate sw values.
///
///
///   Then the logic blocks.
///    SO you have the blocks, where they are; the nets, and their routing. so you can
///     so you need to map the x,y of the tile to the VPR x,y coordinates.. Then as you create each tile
///     you do a lookup for that tile's ChannelX[x][y] and ChannelY[x][y]. you would need to look at the
///
///
///*/
///
///
///
/*

  If this new channel you are in is a higher channel (more right or more up if you stay in the same orientation) or
  has changed orientation but stayed level with this channel, then the Swblock of the reference was the path you had to take..
  Thus you need to access the tile at ref_coord.


  if you are in a channel moving away from the reference channel in an increasing x(for x ref) or an increasing y (for y ref)

  if the ref channel is moving up:
    find the sw_blk port_nr for ref_track_nr and based on the output side of the SW-blk you have one of the

*/
/*
  Connection Block:
  PROG: {BOT_out, TOP_out, TOP_in}
  // prog_data = 16'b1101100111101111;

  prog_data = 16'b1000_0110_1010;

  Wilton Switch Block:
  PROG : {BOT_out, TOP_out, TOP_in}

*/
//pub fn build_bitstream<'a>(nets: &'a Vec<Net>, blocks : &'a Vec<Vec<Block>>, tiles : &'a mut Vec<Vec<TileBuilder>>) -> Result<usize>{
  pub fn build_bitstream<'a>(nets: &'a Vec<Net>, blocks : &'a Vec<Vec<Block>>, tiles : &'a mut Vec<Vec<Tile>>) {

  let CH_WIDTH = GL_CONFIG.lock().unwrap().channel_width;

  for net in nets {
    let branches : &RouteTree = net.route_tree.as_ref();

    //set source and then follow branches

    for route in branches {
      let route : &Route = route;
      let tracks : &Vec<Track> = route.tracks.as_ref();

      //set sink and then set track->wilton_switches.

      for (i,track) in tracks.into_iter().enumerate() { // each track maps to either an Channel_X or a Channel_y
        //set channel at tile[x][y] (probably one of 4 variations.) (maybe no variation and only sink varies depending on position of sink wrt fpga borders.

        let this_track : &Track = track;
        if let Some(prev_track) = tracks.get(i-1){
          let Point(sb_x,sb_y) = if this_track.nr%2 == 0{ //up and right are true for even numbers.
            prev_track.xy.clone()
          }else{
            this_track.xy.clone()
          };

          // Switch Block port calculation.
          let (mut in_port,mut out_port) = (0,0);
          if this_track.nr%2 == 0{ //up and/or right are true for even numbers.
            if prev_track.orientation == XY::X{
              in_port = 2*CH_WIDTH + prev_track.nr;   // in = right
            }else{
              in_port = 2*CH_WIDTH - prev_track.nr;   // in = up
            }
            if this_track.orientation == XY::X{
              out_port = CH_WIDTH - this_track.nr;   // out = right
            }else{
              out_port = 3*CH_WIDTH + this_track.nr; // out = up
            }
          }else{ // directed down/left
            if prev_track.orientation == XY::X{ //
              in_port = CH_WIDTH -  prev_track.nr;
            }else{
              in_port = 2*CH_WIDTH -  prev_track.nr;
            }
            if this_track.orientation == XY::X{
              out_port = 2*CH_WIDTH + this_track.nr;
            }else{
              out_port = 3*CH_WIDTH + this_track.nr;
            }
          }
          tiles[sb_x as usize][sb_y as usize].set_sw_b_bits(in_port,out_port);
        }




      }
      //also there is a sink.
      //{}
    }
  }


}
//struct BitStreamBuilder{
//  tile_matrix : Vec<Vec<Tile>>,
//  bitstream : Vec<bool>,
//}
//impl BitStreamBuilder{
//  fn new(n : u16) -> BitStreamBuilder{
//    let mut tiles : Vec<Vec<Tile>> = Vec::capacity(N);
//    tiles = for y in N {
//      let y_row : Vec<Tile> = Vec::capacity(N);
//      for x in N{
//        y_row.push(TileBuilder::default().xy(x,y))
//      }
//      y_row
//    }
//  }
//  fn flatten(&self){
//    //flatten in the correct way for easy output.
//  }
//  fn get_bitstream(&self) -> Vec<u8>{
//
//  }
//}
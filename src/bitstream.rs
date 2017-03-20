


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
use types::NodeMetaType;

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
///
///
///  If this new channel you are in is a higher channel (more right or more up if you stay in the same orientation) or
///  has changed orientation but stayed level with this channel, then the Swblock of the reference was the path you had to take..
///  Thus you need to access the tile at ref_coord.
///
///
///  if you are in a channel moving away from the reference channel in an increasing x(for x ref) or an increasing y (for y ref)
///
///  if the ref channel is moving up:
///    find the sw_blk port_nr for ref_track_nr and based on the output side of the SW-blk you have one of the
///
///  Connection Block:
///  PROG: {BOT_out, TOP_out, TOP_in}
///  // prog_data = 16'b1101100111101111;
///
///  prog_data = 16'b1000_0110_1010;
///
///  Wilton Switch Block:
///  PROG : {BOT_out, TOP_out, TOP_in}
///
///pub fn build_bitstream<'a>(nets: &'a Vec<Net>, blocks : &'a Vec<Vec<Block>>, tiles : &'a mut Vec<Vec<TileBuilder>>) -> Result<usize>{
  pub fn build_bitstream<'a>(nets: &'a Vec<NetLocal>, blocks : &'a Vec<Vec<Block>>, tiles : &'a mut Vec<Vec<Tile>>) {

//  let gl = GL_CONFIG.map(||{
//
//  });
//  lazy_static!{
//    static ref CH_WIDTH : u16 = gl.channel_width;
//    static ref FPGA_WIDTH : u16 = gl.fpga_width;
//  }

  for net in nets {
    let branches : &Vec<Route> = net.route_tree.as_ref();
    //
    //set source and then follow branches
    //
//    build_bitstream(blocks,tiles)



    //
    //
    // MAP TRACKS TO CHANNELS IN THE TILES ARRAY FOR EACH TRACK IN THE BRANCH.
    //
    build_bitstream_routing_branches(branches,tiles);
  }


}

fn build_bitstream_routing_branches<'a>(branches :&Vec<Route>, tiles : &'a mut Vec<Vec<Tile>>) {
  let gl = GL_CONFIG.lock().unwrap();

  let CH_WIDTH = gl.channel_width;
  let FPGA_WIDTH = gl.fpga_width;

  for route in branches {
    let route : &Route = route;
    let tracks : &Vec<Track> = route.tracks.as_ref();
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
        tiles[sb_y as usize][sb_x as usize].set_sw_b_bits(in_port,out_port); // todo : make sure about tiles[][] indices vs vpr coordinates. [row][col]?
      }
    }
    //
    //
    // SET THE SINK FOR EACH BRANCH OF THE ROUTING TREE
    //
    //  Depending on the edge you are at, the sink should be handled differently
    //
    //   pad sink or lut sink?
    //
    //  top-pad, right-pad, bot-pad, left-pad.
    //
    //
    //
    //set sink and then set track->wilton_switches.
    let sink :&Sink = route.sink.as_ref().unwrap();
    let &Sink(Point(x,y),ref meta_type, class, pin) = sink; // todo : errorh; must have a sink tho.. maybe redundant.
    let prev_track : &Track = tracks.last().unwrap();
    //      let Point()

    /*
    if the last routing track lies within the sink tile, it must be a connection to BLE from right/top,
    or a connection to a pad.

    */
    //
    //
    // FOR SINK OF TYPE PAD.
    //
    if meta_type == &NodeMetaType::Pad{
      let mut tile = &mut tiles[y as usize][x as usize];

      if x==FPGA_WIDTH{ // RIGHT SIDE PADs
        // from the CB through the SW output port.

        // if it goes up
        // must be ch_y from tile(x-1,y) that needs to go to port 2*SIZE-t_nr of SW_B at (x-1,y) with output at (TBA) on rhs.

        // if it goes down, you must find the output port of the SB, trace it to the input and replicate the connection to the rhs.\
        let port_nr = 2*CH_WIDTH-prev_track.nr;
        match prev_track.nr%2 {
          up @ 0 => {
            let in_port = port_nr;
            let out_port = port_nr-4;
            ///no matter the track, if  it goes up, it has to turn right.. this means stil that you need to know the port nr?
            tile.set_sw_b_bits(in_port,out_port);
          },
          down @ 1 => {
            //if down, go check bits at port_nr + 4 and port_nr + 8;
            //  if bit at '+4 is xx1 change it to x11,
            // if bit at '+8 is x1x change it to 11x; actually not 8 but 7 or 11 pins later. (3 or 5 uni-pairs later)
            // note that there is no need for circular buffer as we only consider outputs going out of side 1. Thus only input from side 2 and 3.
            let bit_from_left = tile.sw_blk[(4+(3*port_nr/2)) as usize]; //can only be port 4 and 6. -> 3*2 and 3*3 the starting idx's respectivly.
            tile.sw_blk[(4+(3*port_nr/2)+1) as usize] = bit_from_left;

            if (port_nr - 2)/CH_WIDTH == port_nr/CH_WIDTH { //if they are on the same side, then port nr is the higher port.
              let bit_from_top = tile.sw_blk[(7+(3*port_nr/2)+1) as usize];
              tile.sw_blk[(7+(3*port_nr/2)+2) as usize] = bit_from_top;
            }else{
              let bit_from_top = tile.sw_blk[(11+(3*port_nr/2)+1) as usize];
              tile.sw_blk[(11+(3*port_nr/2)+2) as usize] = bit_from_top;
            }

          },
          _ => unreachable!()
        }


      }else if y == 0{ // BOTTOM PADS
        let port_nr = 2*CH_WIDTH+prev_track.nr;

        match prev_track.nr%2 {
          right @ 0 => {
            let in_port = port_nr;
            let out_port = port_nr - 4;
            tile.set_sw_b_bits(in_port, out_port);
          },
          left @ 1 => {
            // if bits at p'+4 are xx1 change it to 11x, {LSB == right-out}{MSB == left-out}
            // if bit at p'-8 is x1x change it to x11; if straight change to straight and right {}{s}{r} -> x11;
            // TODO: Document these better. especially pictures that wil help you reproduce in future.
            let bit_from_top   = tile.sw_blk[((3 * (port_nr + 4) / 2) + 0) as usize]; //LSB at input port 4 ports higher than current. LSB
            tile.sw_blk[((3 * port_nr + 4 / 2) + 1) as usize]  = bit_from_top; //should then also go straight

            if (port_nr - 2)/CH_WIDTH == port_nr/CH_WIDTH {  // if port_nr is at higher index:
              let bit_from_right = tile.sw_blk[((3 * (port_nr - 11) / 2) + 1) as usize ]; //mid bit at input port 8 ports higher/lower than current. ( 6 uni-pairs earlier..)
              tile.sw_blk[((3 * port_nr - 11 / 2) + 2) as usize]  = bit_from_right; // should also turn left.
            }else{
              let bit_from_right = tile.sw_blk[((3 * (port_nr - 7) / 2) + 1) as usize ]; //mid bit at input port 8 ports higher/lower than current. ( 6 uni-pairs earlier..)
              tile.sw_blk[((3 * port_nr - 7 / 2) + 2) as usize]  = bit_from_right; // should also turn left.
            }
          },
          _ => unreachable!()
        }
      }else if x == 0{ // left out
        let port_nr = 2*CH_WIDTH+prev_track.nr;

        match prev_track.nr%2 {
          up @ 0 => {
            let in_port = port_nr;
            let out_port = port_nr + 4;
            tile.set_sw_b_bits(in_port, out_port);
          },
          down @ 1 => {
            let bit_from_right   = tile.sw_blk[(3 * (port_nr - 4)/ 2 + 2) as usize];
            tile.sw_blk[((3 * port_nr - 4 / 2) + 1) as usize]  = bit_from_right;

            if (port_nr - 2)/CH_WIDTH == port_nr/CH_WIDTH { // if port_nr is at higher index:
              let bit_from_top = tile.sw_blk[(((3 * port_nr + 7) / 2) + 1) as usize ];
              tile.sw_blk[((3 * port_nr + 7 / 2) + 0) as usize]  = bit_from_top;
            }else{
              let bit_from_top = tile.sw_blk[(((3 * port_nr + 11) / 2) + 1) as usize ];
              tile.sw_blk[((3 * port_nr + 11 / 2) + 0) as usize]  = bit_from_top;
            }
          },
          _ => unreachable!()
        }
      }else if y == FPGA_WIDTH{ //top out
        let port_nr = 2*CH_WIDTH+prev_track.nr;

        match prev_track.nr%2 {
          right @ 0 => {
            let in_port = port_nr;
            let out_port = port_nr + 4;
            tile.set_sw_b_bits(in_port, out_port);
          },
          left @ 1 => {

            //  if bit at '-4 is 1xx change it to 11x, {LSB == right-out}{MSB == left-out}
            // if bit at '-8 is x1x change it to x11; if straight change to straight and right {}{s}{r} -> x11;
            // TODO: Document these better. especially pictures that wil help you reproduce in future.

            let bit_from_bot   = tile.sw_blk[(3 * (port_nr - 4)/ 2 + 2) as usize];
            tile.sw_blk[((3 * port_nr - 4 / 2) + 1) as usize]  = bit_from_bot;

            if (port_nr - 2)/CH_WIDTH == port_nr/CH_WIDTH { // if port_nr is at higher index:
              let bit_from_right = tile.sw_blk[(((3 * port_nr - 11) / 2) + 1) as usize ];
              tile.sw_blk[((3 * port_nr - 11 / 2) + 0) as usize]  = bit_from_right;
            }else{
              let bit_from_right = tile.sw_blk[(((3 * port_nr - 7) / 2) + 1) as usize ];
              tile.sw_blk[((3 * port_nr - 7 / 2) + 0) as usize]  = bit_from_right;
            }


          },
          _ => unreachable!()
        }
      }else{
        panic!("error")
      }
      ////
      ////
      //// FOR SINK OF TYPE PIN.
      ////
      //// Connection blocks : PROG: {BOT_out, TOP_out, TOP_in}
      ////
      //// Connection block tracks are numbered from top to bottom in hardware..
      //// thus the MSB of each output/input section of the programming is the the LSB according to
      //// VPR naming..
      ////
      //// Thus if track_nr == 1, for output from CB to bottom, then program con_bkl_top{(CH_WIDTH-1) - track_nr),..}
      ////
      //// note: Connection block 'TOP' for vertical channels is on the left side if the CB
      ////       which means the vpr vs. real indices line up. (unlike the horizontal CBs)
      ////
      //// BIT STRUCTURE :  {bot_out*ch_width, top_out*ch_width, bot_in*ch_width}
      ////

    }else{ // BLE input.

      let CH_MAX = CH_WIDTH - 1;

      let &Sink( ref sink_xy, ref meta_type, class, pin ) = sink;
      let &Point(sink_x,sink_y) = sink_xy;
      let Point(track_x,track_y) = prev_track.xy;
      let track_nr = prev_track.nr;

      let tile : &mut Tile =&mut tiles[track_y as usize][track_x as usize];       // track x,y and sink x,y are not
      // necessarily the same but the CB always
      // lies in the same tile as the track.
      // thus lies @ tiles[track_y][track_x] (aka. here)


      if prev_track.xy == *sink_xy{                              // SINK IS IN THIS TILE
        if prev_track.orientation == XY::X{
          tile.con_blk_top[((2*CH_WIDTH)+(CH_MAX-track_nr)) as usize] = true; //  - downward from CH_X (bot_out)
        }else{
          tile.con_blk_right[(CH_WIDTH+(CH_WIDTH+track_nr)) as usize] = true; //  - to the left from CH_Y(top_out)
        }
      }else if prev_track.orientation == XY::X{                 // SINK IS ABOVE THIS TILE
        tile.con_blk_top[(CH_WIDTH+(CH_MAX-track_nr)) as usize] = true;       //  - connect upward from CH_X(top_out)
        //
      }else{                                                    // SINK IS TO THE RIGHT OF THIS TILE(bot)
        tile.con_blk_right[(2*CH_WIDTH+track_nr) as usize] = true;            //  - connect to the right from CH_Y(bot_out)
      }
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



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
use global::*;
use parse::*;
use errors::*;
use types::*;
use types::PortFlow::*;
use types::XY::*;




/// # FPGA BITSTREAM STRUCTURE
///
/// The FPGA bitstream structure is generated from the VPR mappings to fpga architecture resources such as
///  routing resources (channels, switchblocks pads)
///  logic resources (ble, latches, hardblocks (memory, adders, multipliers etc))
///
/// The representation of VTR abstract fpga architecture in is described in their documentation[todo : link to arch description].
///
/// Firstly, note the FPGA architecture is based on a island style mesh of logic blocks and routing framework.
/// The VTR representation of the abstract architecture is laid out on a grid with cartesian coordinates
/// denoting the location of each resource on the grid.
///
/// The(an) actual FPGA target does not necessarily use the same coordinates for block position, nor any
/// of the conventions for numbering as used in VTR. (As an example, the track numbering in a channel)
///
/// For the mapping of the blif,route and place files to a numbering agnostic bitstream representation of the
/// FPGA the numbering conventions have to be seen as fpga/config specific.
///
/// In the first use case of this program, the conventions are hard coded,( conventions can be found in
/// the hardware description documentation of the FPGA implementation itself(SFPGA design documentation)).
///
/// In this first configuration, all track numbering coincides with VTR track numbering; but
/// port numbering does not match up. blk location is differently numbered but this is inconsequential.
///
/// Switch-block port numbers for example are defined from the right top corner, starting at port 0,
/// going around the sw_blk in a clockwise manner.
///
/// This leads to inconsistency since the direction of increase of the ports differ depending on the
/// side of the sw_blk you are on.
///
/// sw_blk bottom ports are numbered inversely wrt to the track numbering.
/// sw_blk top port are numbered in the same direction.
///
///
///
///
///


/// # BITSTREAM GENERATION
///
///    Start with the routing. for each net, for each route, for each track(node)
///           (This can be done extremely parallel)
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
///  # Connection Block:
///  PROG: {BOT_out, TOP_out, TOP_in}
///  // prog_data = 16'b1101100111101111;
///
///  ```prog_data = 16'b1000_0110_1010;```
///
///  Wilton Switch Block:
///  PROG : {BOT_out, TOP_out, TOP_in}
///
///pub fn build_bitstream<'a>(nets: &'a Vec<Net>, blocks : &'a Vec<Vec<Block>>, tiles : &'a mut Vec<Vec<TileBuilder>>) -> Result<usize>{
pub fn build_bitstream<'a>(nets: &'a Vec<NetLocal>, blocks : &'a Vec<Vec<Block>>, tiles : &'a mut Vec<Vec<Tile>>, places : &'a Vec<Placement>) {
  //todo: go through all the models, and map the model logic to the placement data and store them together in block matrix or hashmap
  for net in nets {
    let branches : &Vec<Route> = net.route_tree.as_ref();
    let first_track = &branches[0].tracks[0];
    //
    //  MAP SOURCE CONNECTION TO CHANNELS FOR EACH NET.
    //
    build_bitstream_source_connections(tiles,&net.src,first_track);
    //
    //  MAP TRACKS TO CHANNELS IN THE TILES ARRAY FOR EACH TRACK IN THE BRANCH.
    //
    build_bitstream_routing_branches(tiles,branches);
  }
}
/// # SOURCE BITSTREAM GENERATION
///
///  every net in the file has a source.
///  the source x,y is the location of the ble or the pad.
///
///  if the source is a pad, connect the correct SW-Block
///
///  if the source is a ble, find the CB for the corresponding pin and connect the cb to the ble.
///
fn build_bitstream_source_connections<'a>(tiles : &'a mut Vec<Vec<Tile>>, source: &Source, output_track: &Track) {

  let &Source(_,ref src_type,_,_) = source;                                                         //todo : change to named struct, so this line can be removed match directly on source.ty

  match src_type{
    &NodeMetaType::Pin => {
      build_bitstream_source_ble(tiles,source,output_track);
    },
    &NodeMetaType::Pad => {
      build_bitstream_source_pad(tiles,source,output_track);
    },
    _ => panic!("Unsupported node type used for Source node.")
  }
}


/// # ROUTING BITSTREAM GENERATION
///
///
///
fn build_bitstream_routing_branches<'a>(tiles : &'a mut Vec<Vec<Tile>>, branches :&Vec<Route>) {


  for route in branches {
    let route : &Route = route;
    let tracks : &Vec<Track> = route.tracks.as_ref();
    for (i,track) in tracks.into_iter().enumerate() { // each track maps to either an Channel_X or a Channel_y
      if i > 0{
        build_bitstream_routing_switch(tiles, &track, &tracks.get(i-1).unwrap());             // todo : what happens if there is none? or i==0?
      }                                                                                              // todo, better provision the iteration to be done in pairs.
    }

    let prev_track : &Track = tracks.last().unwrap();
    let sink :&Sink = route.sink.as_ref().unwrap();

    build_bitstream_sink_connections(tiles,sink,prev_track);
  }
}
/// # Set Bits in Switch Block for CHANNEL -> CHANNEL connection.
///
/// 'in_port' and 'out_port' in this context refers to the port number of the switch block as defined
/// in clockwise manner.
///
///
fn build_bitstream_routing_switch<'a>(tiles : &'a mut Vec<Vec<Tile>>, this_track : &Track,  prev_track : &Track){


  let Point(sb_x,sb_y) = if this_track.nr%2 == 0{ //up and right are true for even numbers.
    prev_track.xy.clone()
  }else{
    this_track.xy.clone()
  };
  tiles[sb_y as usize][sb_x as usize].sw_blk.set_connect(prev_track,this_track);

  //determine ports that correspond to tracks.
//  let (in_port,out_port) = if this_track.nr%2 == 0{ //up and/or right are true for even numbers.
//    let in_port = match prev_track.orientation {
//      X => 2*(*CH_WIDTH)+ prev_track.nr,    // in = right,
//      Y => 2*(*CH_WIDTH)- prev_track.nr     // in = up
//    };
//    let out_port = match this_track.orientation {
//      X => (*CH_WIDTH)- this_track.nr,    // out = right
//      Y => 3*(*CH_WIDTH)+ this_track.nr     // out = up
//    };
//    (in_port,out_port)
//  }else{ // directed down/left
//    let in_port = match prev_track.orientation{
//      X => (*CH_WIDTH)- prev_track.nr,
//      Y => 2*(*CH_WIDTH)- prev_track.nr
//    };
//    let out_port = match this_track.orientation{
//      X => 2*(*CH_WIDTH)+ this_track.nr,
//      Y => 3*(*CH_WIDTH)+ this_track.nr
//    };
//    (in_port,out_port)
//  };


//  let bit_idx = SwitchBlock::get_bit_index(prev_track,this_track);                                   // todo : this can be put in a seperate function.
//  tiles[sb_y as usize][sb_x as usize].set_sw_b_bits(in_port,out_port);    // todo : make sure about tiles[][] indices vs vpr coordinates. [row][col]?
//  tiles[sb_y as usize][sb_x as usize].set_sw_blk_connection(this_track)
}


/// # Connect source of type BLE to routing resources.
/// Can actually abstract this into fn track_to_ble(tiles: VecVec<Tile>, dir :TOP_IN/BOT_OUT/etc, ble_xy, track)
///
///  Then the connection from or to ble is encapsulated in the 'dir'.
///
/// Connection block tracks are numbered from top to bottom in hardware..
/// thus the MSB of each output/input section of the programming is the the LSB according to
/// VPR naming..
///
/// Thus if track_nr == 1, for output from CB to bottom, then program con_bkl_top{((*CH_WIDTH)-1) - track_nr),..}
///
/// note: Connection block 'TOP' for vertical channels is on the left side if the CB
///       which means the vpr vs. real indices line up. (unlike the horizontal CBs)
///
/// BIT STRUCTURE :  {top_in*CH_width, bot_in*CH_width, top_out*CH_width, bot_out*CH_width}
///
/// //NOTE : The algorithm is the same as for sink to ble connection
/// //NOTE : change the SOURCE to only allow SOURCE to CH_Y (to the left of ble) (NOW I DONT NEED TO KNOW TRACK XY, ALWAYS TO THE LEFT OF src xy.)
///
///```tile.con_blk_right[(choose_bit_section("wilton",2,track_nr)) as usize] = true; // 'top_out' (right)
///   tile.con_blk_right[(choose_bit_section("wilton",2,track_nr)) as usize] = true; // 'top_out' (right)
///   tile.con_blk_right.set_conn(gl.wilton, track_nr);
///   tile.set_ch(IntoBLE(track_nr))
///```
///
fn build_bitstream_source_ble<'a>(tiles : &'a mut Vec<Vec<Tile>>, source: &Source, out_track : &Track){

  let &Source(Point(src_x,src_y), _, _, _) = source;
  let Point(tr_x,tr_y) = out_track.xy;

  let tile : &mut Tile = &mut tiles[tr_y as usize][tr_x as usize];   // track x,y and sink x,y are not the same.

  match out_track.orientation {
    X => {
      if tr_y == src_y{
        tile.set_top_con_blk_at(((*CB_BOT_IN_IDX) +  out_track.nr) as usize); // 'bot'
      }else{
        tile.set_top_con_blk_at(((*CB_TOP_IN_IDX) +  out_track.nr) as usize); // 'top'
      }
    },
    Y => {
      if tr_x == src_x{
        tile.set_right_con_blk_at(((*CB_BOT_IN_IDX) +  out_track.nr) as usize); // 'left'
      }else{
        //*THIS IS THE ONLY CONNECTION POSSIBLE AT THE MOMENT
        tile.set_right_con_blk_at(((*CB_TOP_IN_IDX) +  out_track.nr) as usize); // 'right'

      }
    }
  }



}

/// # Connect source of type PAD to routing resources.
///
///
///
fn build_bitstream_source_pad<'a>(tiles : &'a mut Vec<Vec<Tile>>,source: &Source, track : &Track){

  let &Source(Point(src_x,src_y), _, _, _) = source;
  let track_nr = track.nr;
  let Point(tr_x,tr_y) = track.xy;

  let tile : &mut Tile = &mut tiles[tr_y as usize][tr_x as usize];   // track x,y and sink x,y are not the same.

  match tile.try_get_fpga_edge().unwrap(){
    Side::Right => {
      tile.set_right_con_blk_at(((*CB_TOP_IN_IDX) + track_nr) as usize);
    },
    Side::Bottom => {
      tile.set_top_con_blk_at(((*CB_BOT_IN_IDX) + track_nr) as usize);  // 'bot_in' (bot)
      tile.set_ble_at(2 as usize);
    },
    Side::Left => {
      tile.set_right_con_blk_at(((*CB_BOT_IN_IDX) + track_nr) as usize);  // 'bot_in' (left)
      tile.set_ble_at(4 as usize);
    },
    Side::Top => {
      tile.set_top_con_blk_at(((*CB_TOP_IN_IDX) + track_nr) as usize); // 'top_in' (top)
    },
    _ => unreachable!()
  }
}

/// # SINK BITSTREAM GENERATION
///
///  every net in the file has a source.
///  the source x,y is the location of the ble or the pad.
///
///  if the source is a pad, connect the correct SW-Block
///
///  if the source is a ble, find the CB for the corresponding pin and connect the cb to the ble.
///
fn build_bitstream_sink_connections<'a>(tiles : &'a mut Vec<Vec<Tile>>, sink: &Sink, input_track: &Track) {

  let &Sink(_,ref sink_type,_,_) = sink;
  //if the last routing track lies within the sink tile, it must be a connection to BLE from right/top,
  //or a connection to a pad.
  match *sink_type{
    NodeMetaType::Pin => {
      build_bitstream_sink_ble(tiles,sink,input_track);
    },
    NodeMetaType::Pad => {
      build_bitstream_sink_pad(tiles,sink,input_track);
    },
    _ => panic!("Unsupported node type used for Source node.")
  }
}


fn build_bitstream_sink_pad<'a>(tiles : &'a mut Vec<Vec<Tile>>, sink: &Sink, prev_track : &Track){

  let &Sink(Point(x,y),ref meta_type, class, pin) = sink;
  let mut tile : &mut Tile = &mut tiles[y as usize][x as usize];

  match tile.try_get_fpga_edge().unwrap(){
    Side::Right => {
      let port_nr = 2*(*CH_WIDTH)-prev_track.nr;
      match prev_track.nr%2 {
        UpOrRight @ 0 => {
          let in_port = port_nr;
          let out_port = port_nr-4;
          ///no matter the track, if  it goes up, it has to turn right.. this means stil that you need to know the port nr?
          tile.set_sw_b_bits(in_port,out_port);
        },
        DownOrLeft @ 1 => {
          //if down, go check bits at port_nr + 4 and port_nr + 8;
          //  if bit at '+4 is xx1 change it to x11,
          // if bit at '+8 is x1x change it to 11x; actually not 8 but 7 or 11 pins later. (3 or 5 uni-pairs later)
          // note that there is no need for circular buffer as we only consider outputs going out of side 1. Thus only input from side 2 and 3.
          let bit_from_left = tile.sw_blk[(4+(3*port_nr/2)) as usize]; //can only be port 4 and 6. -> 3*2 and 3*3 the starting idx's respectivly.
          tile.sw_blk[(4+(3*port_nr/2)+1) as usize] = bit_from_left;

          if (port_nr - 2)/(*CH_WIDTH)== port_nr/(*CH_WIDTH){ //if they are on the same side, then port nr is the higher port.
            let bit_from_top = tile.sw_blk[(7+(3*port_nr/2)+1) as usize];
            tile.sw_blk[(7+(3*port_nr/2)+2) as usize] = bit_from_top;
          }else{
            let bit_from_top = tile.sw_blk[(11+(3*port_nr/2)+1) as usize];
            tile.sw_blk[(11+(3*port_nr/2)+2) as usize] = bit_from_top;
          }

        },
        _ => unreachable!()
      }
    },
    Side::Bottom => {
      let port_nr = 2*(*CH_WIDTH)+prev_track.nr;

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

          if (port_nr - 2)/(*CH_WIDTH)== port_nr/(*CH_WIDTH){  // if port_nr is at higher index:
            let bit_from_right = tile.sw_blk[((3 * (port_nr - 11) / 2) + 1) as usize ]; //mid bit at input port 8 ports higher/lower than current. ( 6 uni-pairs earlier..)
            tile.sw_blk[((3 * port_nr - 11 / 2) + 2) as usize]  = bit_from_right; // should also turn left.
          }else{
            let bit_from_right = tile.sw_blk[((3 * (port_nr - 7) / 2) + 1) as usize ]; //mid bit at input port 8 ports higher/lower than current. ( 6 uni-pairs earlier..)
            tile.sw_blk[((3 * port_nr - 7 / 2) + 2) as usize]  = bit_from_right; // should also turn left.
          }
        },
        _ => unreachable!()
      }
    },
    Side::Left => {
      let port_nr = 2*(*CH_WIDTH)+prev_track.nr;

      match prev_track.nr%2 {
        up @ 0 => {
          let in_port = port_nr;
          let out_port = port_nr + 4;
          tile.set_sw_b_bits(in_port, out_port);
        },
        down @ 1 => {
          let bit_from_right   = tile.sw_blk[(3 * (port_nr - 4)/ 2 + 2) as usize];
          tile.sw_blk[((3 * port_nr - 4 / 2) + 1) as usize]  = bit_from_right;

          if (port_nr - 2)/(*CH_WIDTH)== port_nr/(*CH_WIDTH){ // if port_nr is at higher index:
            let bit_from_top = tile.sw_blk[(((3 * port_nr + 7) / 2) + 1) as usize ];
            tile.sw_blk[((3 * port_nr + 7 / 2) + 0) as usize]  = bit_from_top;
          }else{
            let bit_from_top = tile.sw_blk[(((3 * port_nr + 11) / 2) + 1) as usize ];
            tile.sw_blk[((3 * port_nr + 11 / 2) + 0) as usize]  = bit_from_top;
          }
        },
        _ => unreachable!()
      }
    },
    Side::Top => {
      let port_nr = 2*(*CH_WIDTH)+prev_track.nr;

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

          if (port_nr - 2)/(*CH_WIDTH)== port_nr/(*CH_WIDTH){ // if port_nr is at higher index:
            let bit_from_right = tile.sw_blk[(((3 * port_nr - 11) / 2) + 1) as usize ];
            tile.sw_blk[((3 * port_nr - 11 / 2) + 0) as usize]  = bit_from_right;
          }else{
            let bit_from_right = tile.sw_blk[(((3 * port_nr - 7) / 2) + 1) as usize ];
            tile.sw_blk[((3 * port_nr - 7 / 2) + 0) as usize]  = bit_from_right;
          }
        },
        _ => unreachable!()
      }
    },
    _ => unreachable!()
  }


}

///
///
/// FOR SINK OF TYPE PIN.
///
/// Connection blocks : PROG: {BOT_out, TOP_out, TOP_in}
///
/// Connection block tracks are numbered from top to bottom in hardware..
/// thus the MSB of each output/input section of the programming is the the LSB according to
/// VPR naming..
///
/// Thus if track_nr == 1, for output from CB to bottom, then program con_bkl_top{((*CH_WIDTH)-1) - track_nr),..}
///
/// note: Connection block 'TOP' for vertical channels is on the left side if the CB
///       which means the vpr vs. real indices line up. (unlike the horizontal CBs)
///
/// BIT STRUCTURE :  {bot_out*ch_width, top_out*ch_width, bot_in*ch_width}
///
fn build_bitstream_sink_ble<'a>(tiles : &'a mut Vec<Vec<Tile>>, sink: &Sink, track : &Track){

  let &Sink( ref sink_xy, _, _, _, ) = sink;
  let Point(track_x,track_y) = track.xy;
  let track_nr = track.nr;

  let tile : &mut Tile =&mut tiles[track_y as usize][track_x as usize];       // track x,y and sink x,y are not
  // necessarily the same but the CB always
  // lies in the same tile as the track.
  // thus lies @ tiles[track_y][track_x] (aka. here)


  if track.xy == *sink_xy{                              // SINK IS IN THIS TILE
    if track.orientation == XY::X{
      tile.con_blk_top[((2*(*CH_WIDTH))+((*CH_MAX)-track.nr)) as usize] = true; //  - downward from CH_X (bot_out)
    }else{
      tile.con_blk_right[((*CH_WIDTH)+((*CH_WIDTH)+track.nr)) as usize] = true; //  - to the left from CH_Y(top_out)
    }
  }else if track.orientation == XY::X{                 // SINK IS ABOVE THIS TILE
    tile.con_blk_top[((*CH_WIDTH)+((*CH_MAX)-track.nr)) as usize] = true;       //  - connect upward from CH_X(top_out)
    //
  }else{                                                    // SINK IS TO THE RIGHT OF THIS TILE(bot)
    tile.con_blk_right[(2*(*CH_WIDTH)+track.nr) as usize] = true;            //  - connect to the right from CH_Y(bot_out)
  }
}


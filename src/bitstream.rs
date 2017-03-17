


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
///
//pub fn build_bitstream<'a>(nets: &'a Vec<Net>, blocks : &'a Vec<Vec<Block>>, tiles : &'a mut Vec<Vec<TileBuilder>>) -> Result<usize>{
  pub fn build_bitstream<'a>(nets: &'a Vec<Net>, blocks : &'a Vec<Vec<Block>>, tiles : &'a mut Vec<Vec<Tile>>) {

//  lazy_static!{
//    right_turn(x,y) -> (x,y)
//    left_turn(x,y) -> (x,y)
//    up(x,y) -> (x,y)
//    down(x,y) -> (x,y)
//  }


  /*
    Connection Block:
    PROG: {BOT_out, TOP_out, TOP_in}
    // prog_data = 16'b1101100111101111;

    prog_data = 16'b1000_0110_1010;

    Wilton Switch Block:
    PROG : {BOT_out, TOP_out, TOP_in}

  */


  let SIZE = 5 as usize;
  let mut ref_track_nr = 0 as u16; //these can actually be u16..
  let mut ref_dir = XY::X;
  let (mut ref_x, mut ref_y) = (0 as u16,0 as u16); //these can actually be u16..
  let mut ref_coord = 0; //these can actually be u16..
  let mut sw_b_loc = Point(0,0);


  for net in nets {
    let branches : &RouteTree = net.route_tree.as_ref();

    //set source and then follow branches

    for route in branches {
      let route : &Route = route;
      let tracks : &Vec<Track> = route.tracks.as_ref();

      //set sink and then set track->wilton_switches.

      for track in tracks { // each track maps to either an Channel_X or a Channel_y
        //set channel at tile[x][y] (probably one of 4 variations.) (maybe no variation and only sink varies depending on position of sink wrt fpga borders.

        let t : &Track = track;

        let (sb_x,sb_y) = if t.nr%2 == 0{ //up and right are true for even numbers.
          ref_coords
        }else{
          (t.xy.1 ,t.xy.0)
        };

        //todo : find inport and outport
        /*
          if you are at tile xy going up/right your input could be from side 1 or side 2.. we dont know.

            your track number is 4*2-ref_track_nr if up and   (aka Y channel)
                              4*2+ref_track_nr if going right. (aka X channel)

          if you are going down the til
        */


        //set the correct switch for the connection to be possible.
        tiles[sb_x as usize][sb_y as usize].set_sw_b_bits(in_port,out_port);

        ref_track_nr = t.nr;
//        if t.orientation == X{
//          xy = x;
//        }else{
//          xy = y;
//        }
//        if up_right == true :
//          then if CH == X
//


        /*

          If this new channel you are in is a higher channel (more right or more up if you stay in the same orientation) or
          has changed orientation but stayed level with this channel, then the Swblock of the reference was the path you had to take..
          Thus you need to access the tile at ref_coord.


          if you are in a channel moving away from the reference channel in an increasing x(for x ref) or an increasing y (for y ref)

          if the ref channel is moving up:
            find the sw_blk port_nr for ref_track_nr and based on the output side of the SW-blk you have one of the

        */


//        if up_right { //then right left straight is only choices for a reference to have gone..
//          if ref_coord > xy{
//            this.sw_blk
//          }else if ref_coord == xy{
//            still this. but now it turned;
//          }else if ref_coord < xy{
//            then down
//          }
//        }


        //the question is : where did we come from?



//        tile.xy = Point(0,0)



        /*
          channel_x.get((x,y)) always lies in tile (x,y) -> channel_x((1,0) lies in tile @ point (1,0)
          channel_x(x+1,y) aka next channel lies at tile ((x+1,y)).
          channel_y.get((x,y)) also lies at tile (x,y) -> channel_y((1,0))(not that it exists(wil be used for pad 0,0(which doesnt exist) goes to tile 1,0.

          F(x,y,O) -> F(x+1,y,O)
          F(x,y,O) -> F(x-1,y,O)
          F(x,y,O) -> F(x+1,y+1,O)
          F(x,y,O) -> F(x-1,y-1,O)
        */
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
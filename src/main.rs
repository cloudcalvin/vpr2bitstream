fn main() {
  println!("Hello, world!");


  //read the .place file, and generate the placement bitstream based on input parameters and mapping file.
  //read the .route file and generate the routing bitstream based on input parameters and mapping file.

  /*

  settings (coordinate system (vpr-style == cartesian))
  read file ( .place )
    place file consists of:
      file name 
      array size 
      space
      header
      placement-data 

  read file ( .route )

    route file consists of:
      array size 
      \n
      'Routing'
      \n      
      routing-data 


  read file (.pre-vpr.blif)
    1) model name
    2) inputs port names
    3) output port names
    4) .names : 
      .name nInputs output
    5) .end

  */

    // single double-node fpga tree? maybe routing tree and placement tree. 

    /*
     1 ) read blif. build out configurations.
     2 ) get placement details. 
     3 ) read .route
     4 ) mangle routing

    */

    let fn get_place : Map< (x,y) , PlaceTree > {
      (net_file,arch_file,N,header,data) = load_placement(name + ".place");
      // data format  : (space+tab delimeted)
      //        block-name, x, y, subblk, blk number (commented with #)

      //for each line in data:
      data.map(|line| { 
        let tokenised = line.split(" ");
        let id = tokenised[0];
        let (x,y) = (tokenised[1],tokenised[2])
        map.insert((x,y), id)
          //for each line look at (x,y). If within array LUT array area, assign 
      })
    } 
    let fn get_routes : Map< (x,y) , PlaceTree > {
      /*
        Each net starts at a source and ends at a sink.. 
        'nets' is a collection of lines that have a header (net nr and port name).
        lines following the net header describe the connections of the net.

      */
      let (N,n_nets,nets) = load_routing(name + ".route") //contains SOURCE -> SINK description
      for net in nets {
        //populate the routing entries in the bitstream?
        //NO_FLIP_METHOD :  if read ch_x(x,y) then map to tile[row=(N-1)-y][col=x] where N = n_tiles. (2x2luts -> 3)
        //FLIP_method : ch_x(x,y) maps to tile[x,y] but requires that all blocks be treated as if they were flipped. ch_x() ch_y() y's are flipped, as well as ch_y(lanes 1->2 2->1 3->4 4->3) 

        //1) flip the mapping so that indices alighn
        //2) dont flip, but map the tiles into different programming order?
      }


    }
    let blif = load_blif(name + ".pre-vpr.blif") //contains '.names' id's and lut content. content mapped to input ports.. 


    /// if 2x2. Then there VPR coords work on a grid of 4x4. Thus 3:0. Where we are mapping it to an array of tiles of 3x3. 
    //// Thus N = 2; n_luts = N*N. lut[N-1:0][N-1:0] and N+1 == n_tiles == n_vpr - 1 thus n_vpr == N+2;




    let bitstream = {
      //scan van 0,0 na N,N en access die place/route/lut-data trees/maps soos nodig om die single/multiple bitstream te generate

    }

}

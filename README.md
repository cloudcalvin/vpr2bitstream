
settings (coordinate system (vpr-style == cartesian))
read file (``` .place ```)
  place file consists of:
    file name
    array size
    space
    header
    placement-data

read file (``` .route ```)

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
    ```.name nInputs output```
  5) .end


  1) Want to be able to match the BLE placement(in .place) with the blif data for that BLE(in .pre-vpr.blif).
  2) want to know the output pins for a BLE and the routing ch_X or ch_y that it goes to as well as whether it connects above or below.



 single double-node fpga tree? maybe routing tree and placement tree.


 1 ) read blif. build out configurations.
 2 ) get placement details.
 3 ) read ```.route```
 4 ) mangle routing



VPR coordinates with which it labels blks and routing : 
```
  y
  ^
  |
  |
  |
  |
  o ---------->x
(0,0)       (1,0)
```
MY SFPGA is defined array based and thus 
```
(0,0)  -> (0,1)
 |
\|/
(1,0)  -> (1,1)

```


0) read placement and create blocks and channels and tiles with proper 'array size'.
1) read placement to allocate .names to blocks. keep reverse lookup (hashmap (name, pos), hashmap (name
2) read routing. Connect the blocks and nets in the tiles.
3) add the blif data to the right tiles.
4) give each tile an x,y and a row columm. Create the bitstream from the correct connections and blif data.
5) rearrange the tiles according to row,col and flatten row major.
```
 let ch : ChannelEx = ChannelExBuilder::default()
     .special_info(42u8)
     .token(19124)
     .build()
     .unwrap();
```
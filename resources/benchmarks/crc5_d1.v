///////////////////////////////////////////////////////////////////////
// File:  CRC5_D1.v                             
// Date:  Wed May  4 22:47:04 2005                                                      
//                                                                     
// Copyright (C) 1999-2003 Easics NV.                 
// This source file may be used and distributed without restriction    
// provided that this copyright statement is not removed from the file 
// and that any derivative work contains the original copyright notice
// and the associated disclaimer.
//
// THIS SOURCE FILE IS PROVIDED "AS IS" AND WITHOUT ANY EXPRESS
// OR IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
// WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
//
// Purpose: Verilog module containing a synthesizable CRC function
//   * polynomial: (0 2 5)
//   * data width: 1
//                                                                     
// Info: tools@easics.be
//       http://www.easics.com                                  
///////////////////////////////////////////////////////////////////////


module crc5_d1;

  // polynomial: (0 2 5)
  // data width: 1
  function [4:0] nextCRC5_D1;

    input Data;
    input [4:0] CRC;

    reg [0:0] D;
    reg [4:0] C;
    reg [4:0] NewCRC;

  begin

    D[0] = Data;
    C = CRC;

    NewCRC[0] = D[0] ^ C[4];
    NewCRC[1] = C[0];
    NewCRC[2] = D[0] ^ C[1] ^ C[4];
    NewCRC[3] = C[2];
    NewCRC[4] = C[3];

    nextCRC5_D1 = NewCRC;

  end

  endfunction

endmodule

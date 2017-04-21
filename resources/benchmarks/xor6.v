module xor6 (i , o);
input [5:0] i;
output  o;

wire  [5:0] i;
wire  o;

assign o = ^i[5]^i[4]^i[3]^i[2]^i[1]^i[0];
endmodule


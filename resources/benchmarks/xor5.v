module xor5 (i , o);
input [4:0] i;
output  o;

wire  [4:0] i;
wire  o;

assign o = i[4]^i[3]^i[2]^i[1]^i[0];
endmodule


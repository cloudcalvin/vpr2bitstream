// Slides ECE 4514

module lfsr4_man(q, clk, rst, seed, load);
output q;
input [3:0] seed;
input load;
input rst;
wire [3:0] state_out;
wire [3:0] state_in;

genvar i;
generate
for (i  = 0; i < 4; i++) begin
	flipflop F(state_out, clk, rst, state_in);
	mux M1 (state_in, load, seed, {state_out[2],state_out[1],state_out[0],nextbit});
end
endgenerate
xor G1(nextbit, state_out[2], state_out[3]);
assign q = nextbit;

endmodule


module flipflop(q, clk, rst, d);
input clk;
input rst;
input d;
output q;
reg q;
always @(posedge clk or posedge rst)
begin
if (rst)
	q = 0;
else
	q = d;
end
endmodule


module mux(q, control, a, b);
output q;
reg q;
input control, a, b;
wire notcontrol;
always @(control or notcontrol or a or b)
	q = (control & a) |(notcontrol & b);
not (notcontrol, control);
endmodule




module lfsr4(out, clk, rst);
output [3:0] out;
input rst;
input clk;
wire [3:0] out;
wire rst;
wire clk;
reg [3:0]q;
assign out=q;
always @ (posedge clk)
begin
 if(rst)
 begin
  q<=4'h1;
 end 
 else
 begin
  q[3]<=q[2];
  q[2]<=q[1];
  q[1]<=q[0];
  q[0]<=q[2]^q[3];
 end
end

endmodule


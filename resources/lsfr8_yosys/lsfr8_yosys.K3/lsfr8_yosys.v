(*top*)
module lfsr8(out, clk, rst);
output [7:0] out;
input rst;
input clk;
wire [7:0] out;
wire rst;
wire clk;
reg [7:0]q;
assign out=q;
always @ (posedge clk)
begin
 if(rst)
 begin
  q<=8'h1;
 end 
 else
 begin
  q[7]<=q[6];
  q[6]<=q[5];	
  q[5]<=q[4];
  q[4]<=q[3];
  q[3]<=q[2];
  q[2]<=q[1];
  q[1]<=q[0];
  q[0]<=q[7]^q[5]^q[4]^q[3];
 end
end

endmodule


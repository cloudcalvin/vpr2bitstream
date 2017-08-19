module Div20z (rst, clk, cet, cep, count, tc);//src = wikipedia
// TITLE 'Divide-by-20 Counter with enables'
// enable CEP is a clock enable only
// enable CET is a clock enable and
// enables the TC output
// a counter using the Verilog language

parameter  size = 5;
parameter  length = 20;

input rst;
input clk;
input cet;
input cep;

output [size-1:0] count;
output tc;

reg [size-1:0] count;


wire tc; //Other signals are of type wire



always @ (posedge clk or posedge rst) begin
  if (rst) begin
    count <= {size{1'b0}};
  else
  if (cet && cep) begin
    if (count == length -1) begin
      count <= {size{1'b0}};
    else
      count <= count + 1'b1;
    end
  end
  end
end


endmodule // Div20z

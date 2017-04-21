//`ifndef _add8
//`define _add8
//`timescale 1ps/1ps
//
module add8(a, b, cin, sum, cout);
input [7:0] a;
input [7:0] b;
input cin;

reg [7:0]tsum;
reg tcout;



output [7:0]sum;
output cout;


assign sum=tsum;
assign cout=tcout;

always @ (a or b or cin)
begin
{tcout,tsum}=a+b+cin;
end 

endmodule
//`endif




//`ifndef _add_tb
//`define _add_tb
//`timescale 1ps/1ps
//
//module add8_tb();
//  reg [7:0] a;
//  reg [7:0] b;
//  reg cin;
//
//  reg [7:0]tsum;
//  reg tcout;
//
//  wire [7:0]sum;
//  wire cout;
//
//endmodule //c_element_adp2
//`endif

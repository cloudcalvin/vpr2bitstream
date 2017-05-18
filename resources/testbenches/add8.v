`ifndef _add8
`define _add8
`timescale 1ps/1ps

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
`endif




`ifndef _add8_tb
`define _add8_tb
`timescale 1ps/1ps

module add8_tb();
 reg [7:0] a;
 reg [7:0] b;
 reg cin;

 reg [7:0]tsum;
 reg tcout;

 wire [7:0]sum;
 wire cout;

parameter runtime = 100;


add8 #(

  )UT(
    .a(a),
    .b(b),
    .cin(cin),
    .sum(sum),
    .cout(cout)
  );


initial begin
  $dumpfile($sformatf("%m.vcd")); 
  $dumpvars();

  a = 0;
  b = 0;
  cin = 0;

  #2

  a = 8'b1111_1111;
  b = 8'b0000_0000;
  
  // #(runtime*3)

  $stop;
end



endmodule //c_element_adp2
`endif

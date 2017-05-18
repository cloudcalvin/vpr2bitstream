`ifndef _add8_tb
`define _add8_tb
`timescale 1ps/1ps

`include "add8.v"

module add8_tb();
 reg [7:0] a;
 reg [7:0] b;
 reg cin;

 wire [7:0]sum;
 wire cout;

 parameter test_duration = 0;
// DUT initialisation
// all ports must be assigned reg and wires of identical naming;
add8 DUT(
    .a(a),
    .b(b),
    .cin(cin),
    .sum(sum),
    .cout(cout)
  );

//what about parameters?

//testbench init
initial begin
  $dumpfile($sformatf("%m.vcd"));
  $dumpvars();
  wait(runtime == $stime)
  $display("\nSimulation completed with %d errors\n", errors);    
  $stop;
end

//test_iter
always @ (posedge next_test) begin
  $write("test nr %d done\n",test_nr);
  next_test = 0; 
  test_nr = test_nr + 1;
  last_test = $stime;
end
//end tests condition
always @ (posedge clk) begin
  if(($stime - last_test) > (end_buffer)) begin
    test_over = 1;
  end
end


// test 0
initial begin
  a = 0;
  b = 0;
end

// test 1
initial begin
  wait(test == 1);

  a = 8'b11111111;
  b = 8'b00000000;
  
  #test_duration;
  if (sum == 8'b11111111) begin
    pass[0] = 1;
  end
  next_test = 1;
end



endmodule 
`endif
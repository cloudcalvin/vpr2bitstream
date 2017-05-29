`ifndef _add8_tb
`define _add8_tb
`timescale 1ps/1ps

`include "resources/benchmarks/add8.v"

//(* testbench *)
module add8_tb();
reg [7:0] a;
reg [7:0] b;
reg cin;

wire [7:0]sum;
wire cout;

//paramters
parameter test_delay = 0;

// DUT initialisation
//  all ports must be assigned reg and wires of identical naming;
add8 DUT(
    .a(a),
    .b(b),
    .cin(cin),
    .sum(sum),
    .cout(cout)
  );


//testbench init
initial begin
  $dumpfile($sformatf("%m.vcd"));
  $dumpvars();
  wait(runtime == $stime)
  $display("\nSimulation completed with %d errors\n", errors);    
  $stop;
end

// # Mutliple tests require a test iterator to be defined.
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
  wait(test_nr === 1);
  $display("TEST %d\n",test_nr); 

  a = 8'b11111111;
  b = 8'b00000000;
  
  #test_delay;
  if (sum == 8'b11111111) begin
    $display("OK\n"); 
    // pass[0] = 1;
  end
  else begin 
   $display("FAIL\n"); 
end
  next_test = 1;
end



endmodule 
`endif
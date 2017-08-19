module smux(a, b, sel, clk, out);
input a, b;
input sel, clk;
output out;
reg out;
always @ (posedge clk) begin
  if (sel) out <= a;
  else out <= b;
end    
endmodule


module smux_tb();
// input clk;
  // parameter off_time = 50;
  // parameter pulse_width = 50;
  reg a, b;
  reg sel, clk;
  wire out;


  // // GLOBAL CLK
  // always begin
  //   clk <= 1'b0;
  //   #(off_time);
  //   clk <= 1'b1;
  //   #(pulse_width);
  // end
  

  //////////////////////////////////////////////////////////
  // TESTS 
  //////////////////////////////////////////////////////////
  initial begin
    $dumpfile($sformatf("%m.vcd"));
    $dumpvars;

    a = 0;
    b = 0;
    sel = 0;
    clk = 0;
    #(period*3);
    $finish;
  end

  initial begin
    a = 1;
    sel = 1;
    clk = 1;
    # 20;
    clk = 0;
  end

endmodule
module mux2 (select,data,enable,out);

input [1:0]select;
input [3:0]data;
input enable;       
output out;

wire [1:0]select;
wire [3:0]data;
wire enable;       
wire out;

reg out_t;
assign out=out_t;
// Odin doesn't allow data[select]
// Therefore, a more manual definition is required

always @ (enable or data or select)
begin
  if (enable) begin
    case (select)
      2'h0 : out_t = data[0];
      2'h1 : out_t = data[1];
      2'h2 : out_t = data[2];
      2'h3 : out_t = data[3];
    endcase
  end
  else
  begin
	out_t = 0;
  end 
end

endmodule



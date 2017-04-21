module mux3 (select,data,enable,out);

input [2:0]select;
input [7:0]data;
input enable;       
output out;

wire [2:0]select;
wire [7:0]data;
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
      3'h0 : out_t = data[0];
      3'h1 : out_t = data[1];
      3'h2 : out_t = data[2];
      3'h3 : out_t = data[3];
      3'h4 : out_t = data[4];
      3'h5 : out_t = data[5];
      3'h6 : out_t = data[6];
      3'h7 : out_t = data[7];
    endcase
  end
  else
  begin
	out_t = 0;
  end 
end

endmodule



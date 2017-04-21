
module decoder3 (in, out, enable);
input [2:0] in;
input  enable;
output [7:0] out;

wire [2:0] in;
wire  enable;
wire [7:0] out;

reg [7:0] out_t;
assign out=out_t;

always @ (enable or in)
begin
  if (enable) begin
    case (in)
      3'h0 : out_t = 8'h01;
      3'h1 : out_t = 8'h02;
      3'h2 : out_t = 8'h04;
      3'h3 : out_t = 8'h08;
      3'h4 : out_t = 8'h10;
      3'h5 : out_t = 8'h20;
      3'h6 : out_t = 8'h40;
      3'h7 : out_t = 8'h80;
    endcase
  end
  else
  begin
	out_t = 0;
  end 
end

endmodule


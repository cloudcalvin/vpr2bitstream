
module mutli(a, b, pro, cout);
input [2:0] a;
input [2:0] b;
input cin;

reg [2:0]tpro;
reg tcout;



output [5:0]pro;
output cout;


assign pro=tpro;
assign cout=tcout;

always @ (a or b or cin)
begin
{tcout,tpro}=a*b;
end 

endmodule


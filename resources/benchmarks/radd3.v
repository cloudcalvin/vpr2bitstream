// One of the first benchmarks obtained from:
//http://gajjarpremal.blogspot.co.za/2013/12/verilog-code-for-8-bit-ripple-carry.html
//One of the advantages of manually defining an adder in this fashion
//is that the VPR names during placement offer a greater degree of clarity

module radd3(a, b, cin, sum, cout);
input [2:0] a;
input [2:0] b;
input cin;
output [2:0]sum;
output cout;
wire[1:0] c;
fulladd a1(a[0],b[0],cin,sum[0],c[0]);
fulladd a2(a[1],b[1],c[0],sum[1],c[1]);
fulladd a3(a[2],b[2],c[1],sum[2],cout);
endmodule

module fulladd(a, b, cin, sum, cout);
input a;
input b;
input cin;
output sum;
output cout;
assign sum=(a^b^cin);
assign cout=((a&b)|(b&cin)|(a&cin));
endmodule

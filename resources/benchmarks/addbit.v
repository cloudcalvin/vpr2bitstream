module
 addbit(a , b , ci , sum ,co );
//Input declaration
input  a;
input  b;
input  ci;
//Ouput declaration
output  sum;
output  co;
//Port Data types
wire  a;
wire  b;
wire  ci;
wire  sum;
wire  co;
//Code starts here
assign {co,sum}  =  a + b + ci;
endmodule
// End of Module addbi

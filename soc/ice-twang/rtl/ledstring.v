/*
 * ledstring.v
 * 
 * Copyright (C) 2021 Piotr Esden-Tempski <piotr@esden.net>
 * SPDX-License-Identfier: CERN-OHL-P-2.0
 */

`default_nettype 

module ledstring (
    // LED String IO
    output wire led_clk,
    output wire led_data,

    // Data
    input  wire framing,  // 0= data, 1= framing
    input  wire se_frame, // 0= start frame, 1= end frame
    input  wire [4:0] dat_glo,
    input  wire [7:0] dat_red,
    input  wire [7:0] dat_grn,
    input  wire [7:0] dat_blu,
    input  wire valid,
    output reg  ack,

    // Clock / Reset
    input  wire clk,
    input  wire rst
);

    // Signals
    wire go, done;//, ce;
    reg  active;
    reg [31:0] shift;
    reg [5:0] bit_cnt;

    // Control
    assign go = valid & ~active;
    assign done = bit_cnt[5];

    always @(posedge clk or posedge rst)
        if (rst)
            active <= 1'b0;
        else
            active <= (active & ~done) | go;

    // Bit counter
    always @(posedge clk)
        if (~active)
            bit_cnt <=6'h1E;
        else
            bit_cnt <= bit_cnt - 1;


    // Shift register
    always @(posedge clk or posedge rst)
        if (rst)
            shift <= 32'h00000000;
        else if(go & framing)
            shift <= { 32{se_frame} };
        else if(go & ~framing)
            shift <= { 3'b111, dat_glo, dat_blu, dat_grn, dat_red };
        else if(active)
            shift <= { shift[30:0], 1'b1 };

    // Outputs
    always @(posedge clk)
        ack <= go;

    assign led_clk  = active & ~clk;
    assign led_data = shift[31];


endmodule

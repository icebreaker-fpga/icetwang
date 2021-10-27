/*
 * ledstring.v
 * 
 * Copyright (C) 2021 Piotr Esden-Tempski <piotr@esden.net>
 * SPDX-License-Identfier: CERN-OHL-P-2.0
 */

`default_nettype none

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


    SB_IO #(
        .PIN_TYPE(6'b0100_00)
    ) led_clk_pin (
        .PACKAGE_PIN(led_clk),
        .OUTPUT_CLK(clk),
        .D_OUT_0(1'b0),
        .D_OUT_1(active)
    );

    assign led_data = shift[31];


endmodule

// A small test module to generate a static 16 led long data stream
module ls_tst_runner (
    output wire led_clk,
    output wire led_data,

    input  wire clk,
    input  wire rst
);

    wire [23:0] pixl_data [0:15];

    assign pixl_data[0]  = 24'hAA55EE;
    assign pixl_data[1]  = 24'h9944DD;
    assign pixl_data[2]  = 24'h8833CC;
    assign pixl_data[3]  = 24'h7722BB;
    assign pixl_data[4]  = 24'h6611AA;
    assign pixl_data[5]  = 24'h550099;
    assign pixl_data[6]  = 24'h44F188;
    assign pixl_data[7]  = 24'h33E277;

    assign pixl_data[8]  = 24'h22D366;
    assign pixl_data[9]  = 24'h11C455;
    assign pixl_data[10] = 24'h00B544;
    assign pixl_data[11] = 24'h11A633;
    assign pixl_data[12] = 24'h229722;
    assign pixl_data[13] = 24'h338811;
    assign pixl_data[14] = 24'h447900;
    assign pixl_data[15] = 24'h556A1F;


    reg  framing;  // 0= data, 1= framing
    reg  se_frame; // 0= start frame, 1= end frame
    reg  [7:0] dat_red;
    reg  [7:0] dat_grn;
    reg  [7:0] dat_blu;
    reg  valid;
    wire ack;

    parameter IDLE=0, FS=1, FS_ACK=2, DAT=3, DAT_ACK=4, FE=5, FE_ACK=6;
    reg [2:0] state;
    reg [4:0] dat_cnt;

    always @(posedge clk)
        if (rst) begin
            state    <= IDLE;
            dat_cnt  <= 5'h0F;

            framing  <= 1'b1;
            se_frame <= 1'b0;
            dat_red  <= 8'h00;
            dat_grn  <= 8'h00;
            dat_blu  <= 8'h00;
            valid    <= 1'b0;
        end else case (state)
            IDLE:
                state <= FS;
            FS: begin
                framing <= 1'b1;  // framing mode
                se_frame <= 1'b0; // start frame
                valid <= 1'b1;    // indicate that data is valid
                state <= FS_ACK;  // wait for ack
            end
            FS_ACK:
                if (ack) begin
                   valid <= 1'b0;
                   dat_cnt <= 5'h00; // start from the first pixel
                   state <= DAT;
                end
            DAT: begin
                framing <= 1'b0;  // data mode
                {dat_red, dat_grn, dat_blu} = pixl_data[dat_cnt];
                valid <= 1'b1;
                state <= DAT_ACK;
            end
            DAT_ACK:
                if (ack) begin
                    valid <= 1'b0;
                    if (dat_cnt == 5'h0F)
                        state <= FE; // sent all data, send end frame
                    else begin
                        dat_cnt <= dat_cnt + 1;
                        state <= DAT; // send next data frame
                    end
                end
            FE: begin
                framing <= 1'b1;  // framing mode
                se_frame <= 1'b1; // end frame
                valid <= 1'b1;    // indicate that data is valid
                state <= FE_ACK;
            end
            FE_ACK:
                if (ack) begin
                    valid <= 1'b0;
                    state <= IDLE; // we are done!
                end

        endcase

    ledstring ls1 (
        .led_clk(led_clk),
        .led_data(led_data),

        .framing(framing),
        .se_frame(se_frame),
        .dat_glo(5'h1F), // Full global brightness
        .dat_red(dat_red),
        .dat_grn(dat_grn),
        .dat_blu(dat_blu),
        .valid(valid),
        .ack(ack),

        .clk(clk),
        .rst(rst)
    );

endmodule
/*
 * ledstring_tb.v
 *
 * vim: ts=4 sw=4
 *
 * Copyright (C) 2019  Sylvain Munaut <tnt@246tNt.com>
 * All rights reserved.
 *
 * BSD 3-clause, see LICENSE.bsd
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *     * Neither the name of the <organization> nor the
 *       names of its contributors may be used to endorse or promote products
 *       derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL <COPYRIGHT HOLDER> BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

`default_nettype none

module ledstring_tb;

    // Signals
    // -------
    wire led_clk;
    wire led_data;
    //reg framing = 1'b0;
    //reg se_frame = 1'b0;
    //reg [4:0] dat_glo = 5'b01010;
    //reg [7:0] dat_red = 8'hAA;
    //reg [7:0] dat_grn = 8'h55;
    //reg [7:0] dat_blu = 8'h5A;
    //reg valid = 1'b0;
    //wire ack;
    reg clk = 1'b0;
    reg rst = 1'b1;

    // Setup recording
    // ---------------

    initial begin
        $dumpfile("ledstring_tb.vcd");
        $dumpvars(0,ledstring_tb);
        #20000 $finish;
    end

    always #10 clk <= !clk;

    // initial begin
    //     #20 rst = 0;
    //     #20 framing = 1;
    //     #20 valid = 1; // send start frame
    //     #20 valid = 0;
    //     #1000 framing = 0; // wait for the send to finish
    //     #20 valid = 1; // send data
    //     #20 valid = 0;
    //     #1000 framing = 1; se_frame = 1;
    //     #20 valid = 1; // send end frame
    //     #20 valid = 0;
    // end
    initial begin
        #20 rst = 0;
    end

    // DUT
    // ---

    // ledstring dut_I (
    //     .led_clk(led_clk),
    //     .led_data(led_data),
        
    //     .framing(framing),
    //     .se_frame(se_frame),
    //     .dat_glo(dat_glo),
    //     .dat_red(dat_red),
    //     .dat_grn(dat_grn),
    //     .dat_blu(dat_blu),
    //     .valid(valid),
    //     .ack(ack),

    //     .clk(clk),
    //     .rst(rst)
    // );

    ls_runner dut_I (
        .led_clk(led_clk),
        .led_data(led_data),
        .clk(clk),
        .rst(rst)
    );

endmodule // ledstring_tb

module ls_runner (
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
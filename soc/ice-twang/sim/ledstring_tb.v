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

    ls_tst_runner dut_I (
        .led_clk(led_clk),
        .led_data(led_data),
        .clk(clk),
        .rst(rst)
    );

endmodule // ledstring_tb
/*
 * timer_tb.v
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

module timer_tb;

    // Signals
    // -------
    reg [1:0] wb_addr;
    wire [31:0] wb_rdata;
    reg [31:0] wb_wdata;
    reg wb_we;
    reg wb_cyc;
    wire wb_ack;
    reg clk = 1'b0;
    reg rst = 1'b1;

    reg [127:0] annotation;

    // Tasks
    task wb_write;
		input [ 4:0] addr;
		input [31:0] data;
		begin
			wb_addr  <= addr;
			wb_wdata <= data;
			wb_we    <= 1'b1;
			wb_cyc   <= 1'b1;

			while (~wb_ack)
				@(posedge clk);

			wb_addr  <= 4'hx;
			wb_wdata <= 32'hxxxxxxxx;
			wb_we    <= 1'bx;
			wb_cyc   <= 1'b0;

			@(posedge clk);
		end
    endtask


    // Setup recording
    // ---------------

    initial begin
        $dumpfile("timer_tb.vcd");
        $dumpvars(0,timer_tb);
        #20000 $finish;
    end

    always #10 clk <= !clk;

    initial begin
        #20 rst = 0;
    end

    initial begin
		// Defaults
		wb_addr  <= 4'hx;
		wb_wdata <= 32'hxxxxxxxx;
		wb_we    <= 1'bx;
		wb_cyc   <= 1'b0;
        annotation <= 0;

        @(negedge rst);
		@(posedge clk);

        annotation <= "rtim";
		wb_write(5'h01, 32'h00000002); // set LOAD
		wb_write(5'h02, 32'h00000000); // set RELOAD
		wb_write(5'h00, 32'h00000001); // set CSR - Enable Timer

        annotation <= 0;
        repeat (20) @(posedge clk);

        annotation <= "stop";
		wb_write(5'h00, 32'h00000000); // set CSR - Disable Timer

        annotation <= 0;
        repeat (20) @(posedge clk);

        annotation <= "evtim";
		wb_write(5'h00, 32'h00000009); // set CSR - Enable Timer with EV enabled

        annotation <= 0;
        repeat (20) @(posedge clk);

        annotation <= "rstev";
        wb_write(5'h00, 32'h00000009); // reset CSR

        repeat (20) @(posedge clk);

        annotation <= "stop";
		wb_write(5'h00, 32'h00000000); // set CSR - Disable Timer

        repeat (20) @(posedge clk);

        annotation <= "evtim";
		wb_write(5'h00, 32'h00000009); // set CSR - Enable Timer with EV enabled

        annotation <= 0;
        repeat (20) @(posedge clk);


    end

    // DUT
    // ---

    timer_wb #(
        .DIV(0)
    ) dut_I (
        .wb_addr(wb_addr),
        .wb_rdata(wb_rdata),
        .wb_wdata(wb_wdata),
        .wb_we(wb_we),
        .wb_cyc(wb_cyc),
        .wb_ack(wb_ack),

        .clk(clk),
        .rst(rst)
    );

endmodule // timer_tb
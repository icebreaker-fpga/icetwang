/*
 * ledstring.v
 *
 * Copyright (C) 2021 Piotr Esden-Tempski <piotr@esden.net>
 * SPDX-License-Identfier: CERN-OHL-P-2.0
 */
`default_nettype none

module joy_wb (
    // Buttons
    input wire joy_up,
    input wire joy_down,
    input wire joy_left,
    input wire joy_right,

    // Bus interface
    //input  wire [0] wb_addr, // No address needed
    output reg  [31:0] wb_rdata,
    //input  wire [31:0] wb_wdata, // You can only read
    input  wire        wb_we,
    input  wire        wb_cyc,
    output wire        wb_ack,

    // Clock / Reset
    input  wire clk,
    input  wire rst
);

    // Wishbone
    reg b_ack;
    wire b_rd_rst;

    // Wishbone interface
    // ------------------

    // Ack
    always @(posedge clk)
        b_ack <= wb_cyc & ~b_ack;

    assign wb_ack = b_ack;

    // Write
    // Nothing to do here

    // Read
    assign b_rd_rst = ~wb_cyc | b_ack;

    always @(posedge clk)
        if (b_rd_rst)
            wb_rdata <= 32'h00000000;
        else
            wb_rdata <= {28'h000000, fjoy_right, fjoy_left, fjoy_down, fjoy_up};

    // Signals

    // Filtered (Deglitched) buttons
    wire fjoy_right;
    wire fjoy_left;
    wire fjoy_up;
    wire fjoy_down;

    // IOB outputs
    wire bjoy_right;
    wire bjoy_left;
    wire bjoy_up;
    wire bjoy_down;

    // IOB instantiation
    SB_IO #(
		.PIN_TYPE(6'b000000),	// Reg input, no output
		.PULLUP(1'b1),
	    .IO_STANDARD("SB_LVCMOS")
	) btn_iob_I[3:0] (
		.PACKAGE_PIN({joy_up, joy_down, joy_left, joy_right}),
		.INPUT_CLK  (clk),
		.D_IN_0     ({bjoy_up, bjoy_down, bjoy_left, bjoy_right})
	);

    // Deglitch filters
    glitch_filter #(
		.L(4),
		.RST_VAL(1'b1), // Invert
		.WITH_SYNCHRONIZER(1),
		.WITH_SAMP_COND(0)
	) joy_up_flt_I (
		.in       (~bjoy_up), // Invert
		.samp_cond(),
		.val      (fjoy_up),
		.rise     (),
		.fall     (),
		.clk      (clk),
`ifdef SIM
		.rst      (rst)
`else
		// Don't reset so we let the filter settle before
		// the rest of the logic engages
		.rst      (1'b0)
`endif
	);

    glitch_filter #(
		.L(4),
		.RST_VAL(1'b1), // Invert
		.WITH_SYNCHRONIZER(1),
		.WITH_SAMP_COND(0)
	) joy_down_flt_I (
		.in       (~bjoy_down),
		.samp_cond(),
		.val      (fjoy_down),
		.rise     (),
		.fall     (),
		.clk      (clk),
`ifdef SIM
		.rst      (rst)
`else
		// Don't reset so we let the filter settle before
		// the rest of the logic engages
		.rst      (1'b0)
`endif
	);

    glitch_filter #(
		.L(4),
		.RST_VAL(1'b1), // Invert
		.WITH_SYNCHRONIZER(1),
		.WITH_SAMP_COND(0)
	) joy_left_flt_I (
		.in       (~bjoy_left),
		.samp_cond(),
		.val      (fjoy_left),
		.rise     (),
		.fall     (),
		.clk      (clk),
`ifdef SIM
		.rst      (rst)
`else
		// Don't reset so we let the filter settle before
		// the rest of the logic engages
		.rst      (1'b0)
`endif
	);

    glitch_filter #(
		.L(4),
		.RST_VAL(1'b1), // Invert
		.WITH_SYNCHRONIZER(1),
		.WITH_SAMP_COND(0)
	) joy_right_flt_I (
		.in       (~bjoy_right),
		.samp_cond(),
		.val      (fjoy_right),
		.rise     (),
		.fall     (),
		.clk      (clk),
`ifdef SIM
		.rst      (rst)
`else
		// Don't reset so we let the filter settle before
		// the rest of the logic engages
		.rst      (1'b0)
`endif
	);

endmodule
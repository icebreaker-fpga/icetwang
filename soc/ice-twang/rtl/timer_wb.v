/*
 * ledstring.v
 *
 * Copyright (C) 2021 Piotr Esden-Tempski <piotr@esden.net>
 * SPDX-License-Identfier: CERN-OHL-P-2.0
 */
`default_nettype none

module timer_wb (
    // Bus interface

    // Bus interface
    input  wire [1:0] wb_addr,
    output reg  [31:0] wb_rdata,
    input  wire [31:0] wb_wdata,
    input  wire        wb_we,
    input  wire        wb_cyc,
    output wire        wb_ack,

    // Clock / Reset
    input  wire clk,
    input  wire rst
);

    // Signals
    // -------

    // Wishbone
    reg b_ack;
    reg b_we_csr;
    reg b_we_load;
    reg b_we_reload;
    wire b_rd_rst;

    // CSR
    //        [31:4] [3]    [2] [1] [0]
    // write: rsv    ev en  ev  0   en
    // read:  0      ev en  ev  st  en
    reg ev_en   = 1'b0; // set to 1 to ennable events
    reg ev      = 1'b0; // is set to 1 when ev_en is 1 and counter reaches 0, can be reset by writing 0
    wire st;            // 1 when the counter is running aka. != 0
    reg en      = 1'b0; // set to 1 to enable counter, transition 0->1 loads `load`

    // LOAD
    // The initial value to be loaded when enable is set
    reg [31:0] load = 32'h00000000;

    // RELOAD
    // The value to be loaded when counter reaches 0
    reg [31:0] reload = 32'h00000000;

    // COUNTER
    // The timer counter
    reg [31:0] counter = 32'h00000000;

    // Wishbone interface
    // ------------------

    // Ack
    always @(posedge clk)
        b_ack <= wb_cyc & ~b_ack;

    assign wb_ack = b_ack;

    // Write
    always @(posedge clk)
        if (b_ack) begin
            b_we_csr    <= 1'b0;
            b_we_load   <= 1'b0;
            b_we_reload <= 1'b0;
        end else begin
            b_we_csr    <= wb_cyc & wb_we & (wb_addr == 2'b00);
            b_we_load   <= wb_cyc & wb_we & (wb_addr == 2'b01);
            b_we_reload <= wb_cyc & wb_we & (wb_addr == 2'b10);
        end

    always @(posedge clk)
        if (rst) begin
            ev_en <= 1'b0;
            ev    <= 1'b0;
            en    <= 1'b0;
        end else if (b_we_csr) begin
            ev_en <= wb_wdata[3];
            ev    <= wb_wdata[2];
            en    <= wb_wdata[0];
        end else if (counter_zero_strb)
            ev    <= ev_en;

    always @(posedge clk)
        if (rst) begin
            load <= 32'h00000000;
        end else if (b_we_load) begin
            load <= wb_wdata;
        end

    always @(posedge clk)
        if (rst) begin
            reload <= 32'h00000000;
        end else if (b_we_reload) begin
            reload <= wb_wdata;
        end

    // Read
    assign b_rd_rst = ~wb_cyc | b_ack;

    always @(posedge clk)
        if (b_rd_rst)
            wb_rdata <= 32'h00000000;
        else
            casez (wb_addr)
                2'b00: wb_rdata <= {24'h000000, ev_en, ev, st, en};
                2'b01: wb_rdata <= load;
                2'b10: wb_rdata <= reload;
                2'b11: wb_rdata <= counter;
                default: wb_rdata <= 32'hxxxxxxxx;
            endcase

    // Signals
    localparam DIV = 23; // 24MHz clock -> us counter
    localparam DIVN = $clog2(DIV);
    initial begin
        $display("timer_wb:divider width: %d", DIVN);
    end
    reg [DIVN:0] divider = DIV;
    wire ce;
    reg prev_en = 1'b0;
    wire en_strb;
    wire counter_zero;
    reg prev_counter_zero = 1'b0;
    wire counter_zero_strb;

    // Divider
    always @(posedge clk)
        if (~en | divider[DIVN])
            divider <= DIV;
        else
            divider <= divider - 1;

    assign ce = divider[DIVN];

    // Enable strobe
    always @(posedge clk)
        if (rst)
            prev_en <= 1'b0;
        else
            if (ce)
                prev_en <= en;

    assign en_strb = (~prev_en & en);

    // Counter
    always @(posedge clk) begin
        if (rst) begin
            counter <= 32'h00000000;
        end else if (ce) begin
            if (en_strb)
                counter <= load;
            else if (counter_zero) begin
                counter <= reload;
            end else
                counter <= counter - 1;
        end
    end

    assign counter_zero = (counter == 32'b00000000);
    assign st = ~counter_zero;

    // Counter zero strobe
    always @(posedge clk)
        if (rst)
            prev_counter_zero <= 1'b0;
        else
            prev_counter_zero <= counter_zero;

    assign counter_zero_strb = (~prev_counter_zero & counter_zero);

endmodule
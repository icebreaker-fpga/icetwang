/*
 * ledstring.v
 *
 * Copyright (C) 2021 Piotr Esden-Tempski <piotr@esden.net>
 * SPDX-License-Identfier: CERN-OHL-P-2.0
 */
`default_nettype none

module ledstring_wb (
    // LED String IO
    output wire led_clk,
    output wire led_data,

    // Bus interface
    input  wire [9:0] wb_addr,
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
    reg  b_ack;
    reg  b_we_csr;
    reg  b_we_glob;
    reg  b_we_ldat;
    wire b_rd_rst;

    // CSR
    //  [31] [30] [29:25] [24:16] [15:0]
    //  strt 0    res     len     div
    //  0    bsy  res     len     div
    //
    //  div = clock divider 0= sys_clk (Not implemented, always sys_clk)
    //  len = LED string length 0..512
    //  strt = write 1 to start transfer of vram to led string
    //  bsy = reads 1 when transfer from vram to led string in progress
    reg strt = 0;
    reg bsy = 0;
    reg [8:0] len = 0; // 0..512 LED string length
    reg [15:0] div = 0; // Clock divider (Not implemented, fixed to 0)
    // GLOB
    //  [31:5] [4:0]
    //  res    glob
    //
    //  glob = global LED brightness (defaults to highest)
    //  Note: The APA102 use a lower frequency modulation to implement
    //  global brightness setting. Using a global brightness value can
    //  result in LED flicker.
    reg [4:0] glob = 5'b11111; // Global brightness

    // LED data buffer
    // Note: This module uses 3 ICESTORM_RAM blocks.
    reg [23:0] ldat [(1<<9)-1:0];

    // Wishbone interface
    // ------------------

    // Ack
    always @(posedge clk)
        b_ack <= wb_cyc & ~b_ack;

    assign wb_ack = b_ack;

    // Write
    always @(posedge clk) begin
        if (b_ack) begin
            b_we_csr    <= 1'b0;
            b_we_glob   <= 1'b0;
            b_we_ldat   <= 1'b0;
        end else begin
            b_we_csr    <= wb_cyc & wb_we & ~wb_addr[9] & ~wb_addr[0];
            b_we_glob   <= wb_cyc & wb_we & ~wb_addr[9] &  wb_addr[0];
            b_we_ldat   <= wb_cyc & wb_we & wb_addr[9];
        end
    end

    always @(posedge clk) begin
        if (rst) begin
            strt <= 0;
            len <= 0;
            div <= 0;
        end else if (b_we_csr) begin
            strt <= wb_wdata[31];
            len  <= wb_wdata[24:16];
            div  <= wb_wdata[15:0];
        end
    end

    always @(posedge clk) begin
        if (rst) begin
            glob <= 5'b11111;
        end else if (b_we_glob) begin
            glob <= wb_wdata[4:0];
        end
    end

    always @(posedge clk) begin
        if (b_we_ldat) begin
            ldat[wb_addr[8:0]] <= wb_wdata[23:0];
        end
    end

    // Read
    assign b_rd_rst = ~wb_cyc | b_ack;

    always @(posedge clk) begin
        if (b_rd_rst)
            wb_rdata <= 32'h00000000;
        else
            casez (wb_addr)
                10'b0z_zzzz_zzz0: wb_rdata <= {1'b0, bsy, 5'b00000, len, div};
                10'b0z_zzzz_zzz1: wb_rdata <= {27'h0000000, glob};
                10'b1z_zzzz_zzzz: wb_rdata <= ldat[wb_addr[8:0]];
                default: wb_rdata <= 32'hxxxxxxxx;
            endcase
    end

endmodule // ledstring_wb
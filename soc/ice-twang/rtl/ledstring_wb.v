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
    reg strt = 1'b0;
    reg bsy = 1'b0;
    reg [8:0] len = 9'h000; // 0..512 LED string length
    reg [15:0] div = 16'h0000; // Clock divider (Not implemented, fixed to 0)
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
            strt <= 1'b0;
            len <= 9'b0;
            div <= 16'b0;
        end else if (b_we_csr) begin
            strt <= wb_wdata[31];
            len  <= wb_wdata[24:16];
            div  <= wb_wdata[15:0];
        end else if (bsy) begin
            strt <= 1'b0;
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
                10'b0z_zzzz_zzz0: wb_rdata <= {strt, bsy, 5'b00000, len, div};
                10'b0z_zzzz_zzz1: wb_rdata <= {27'h0000000, glob};
                //10'b1z_zzzz_zzzz: wb_rdata <= ldat[wb_addr[8:0]]; // Disabled for now
                10'b1z_zzzz_zzzz: wb_rdata <= 32'h00000000;
                default: wb_rdata <= 32'hxxxxxxxx;
            endcase
    end

    // Signals
    reg framing;
    reg se_frame;
    reg [8:0] led_ptr;
    reg valid;
    wire ack;

    // State Machine
    parameter IDLE=0, FS=1, DAT=2, FE=3;
    reg [1:0] state;

    always @(posedge clk)
        if (rst) begin
            state    <= IDLE;
            framing  <= 1'b1;
            se_frame <= 1'b0;
            led_ptr  <= 9'h00;
            valid    <= 1'b0;
        end else case (state)
            IDLE:
                if (strt) begin       // Send Start Frame
                    bsy <= 1'b1;      // Indicate busy
                    framing <= 1'b1;  // framing mode (Start)
                    se_frame <= 1'b0; // start frame
                    led_ptr <= {1'b0, len}; // prime LED data pointer
                    valid <= 1'b1;    // indicate that data is valid
                    state <= FS;
                end
            FS:
                if (ack) begin
                    framing <= 1'b0;      // data mode
                    state <= DAT;
                end
            DAT:
                if (ack) begin
                    led_ptr <= led_ptr - 1; // Decrement LED Pointer
                    if (led_ptr == 9'h00) begin // Send End Frame
                        framing <= 1'b1;  // framing mode
                        se_frame <= 1'b1; // end frame
                        state <= FE;
                    end
                end
            FE:
                if (ack) begin
                    bsy   <= 1'b0;
                    valid <= 1'b0;
                    state <= IDLE; // we are done!
                end
        endcase

    wire [7:0] dat_red;
    wire [7:0] dat_grn;
    wire [7:0] dat_blu;

    assign {dat_red, dat_grn, dat_blu} = ldat[led_ptr];

    ledstring ls1 (
        .led_clk(led_clk),
        .led_data(led_data),

        .framing(framing),
        .se_frame(se_frame),
        .dat_glo(glob), // Full global brightness
        .dat_red(dat_red),
        .dat_grn(dat_grn),
        .dat_blu(dat_blu),
        .valid(valid),
        .ack(ack),

        .clk(clk),
        .rst(rst)
    );

endmodule // ledstring_wb

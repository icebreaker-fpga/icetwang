RISC-V core for 1D LED games (iCETWANG)
=========================================

For the iCEBreaker-bitsy, the hardware connections are :
  * `42`: USB DP
  * `38`: USB DN
  * `37`: Pull up. Resistor of 1.5 kOhm to USB DP 

To run :
  * Build and flash the bitstream
      * `make dfuprog`
      * This will build `fw/boot.hex` and include it as the BRAM initial data

  * You can flash the test c application code in SPI at offset 1M
      * `make -C fw dfuprog`
  
  * This project's main application can be found in `../../firmware/`
    directory. Follow the instructions there.

  * Connect to the iCEBreaker-bitsy uart console (P0, P1) with a 1M baudrate
      * and then at the `Command>` prompt, press `r` for 'run'. This will
        start the USB detection and device should enumerate

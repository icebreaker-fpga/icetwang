# PAC for the iCE-TWANG SOC peripherals

SVD is generated from the soc generator script located in `../../soc/icetwang/svd`.

To generate the SVD file run the gensvd.sh script.

All you need to do to regenerate the files is replace `icetwang-soc.svd`.
The `build.rs` will see this file has changed and then regenerate `src/pac.rs`.

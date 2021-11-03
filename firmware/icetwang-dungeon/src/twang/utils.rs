/*
 * Copyright (c) 2020-2021, Piotr Esden-Tempski <piotr@esden.net>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

use core::ops::*;
use core::cmp::PartialOrd;

pub fn range_map<T>(i: T, imin: T, imax: T, omin: T, omax: T) -> T
where
    T: Add<Output = T> + Sub<Output = T> +
        Mul<Output = T> + Div<Output = T> +
        PartialOrd + Copy
{
    let mut invert = false;
    let ispan: T;
    let ospan: T;
    if omax < omin {
        ospan = omin - omax;
        invert = !invert;
    } else  {
        ospan = omax - omin;
    }
    if imax < imin {
        ispan = imin - imax;
        invert = !invert;
    } else  {
        ispan = imax - imin;
    }
    if invert {
        return omin - (((i - imin) * ospan) / ispan);
    } else {
        return (((i - imin) * ospan) / ispan) + omin;
    }
}
const SIN_LUT_SIZE: usize = 64;
const SIN_LUT: [u8; SIN_LUT_SIZE as usize] = [
    0,0,0,0,1,1,1,2,
    2,3,4,5,5,6,7,9,
    10,11,12,14,15,17,18,20,
    21,23,25,27,29,31,33,35,
    37,40,42,44,47,49,52,54,
    57,59,62,65,67,70,73,76,
    79,82,85,88,90,93,97,100,
    103,106,109,112,115,118,121,124
];

pub fn sinu8(n: u8) -> u8 {
    if n < SIN_LUT_SIZE as u8 { SIN_LUT[n as usize] }
    else if n < (SIN_LUT_SIZE * 2) as u8 { 255 - SIN_LUT[(SIN_LUT_SIZE * 2 - n as usize - 1)] }
    else if n < (SIN_LUT_SIZE * 3) as u8 { 255 - SIN_LUT[(n as usize - (SIN_LUT_SIZE * 2))] }
    else { SIN_LUT[((SIN_LUT_SIZE * 4 - 1) - n as usize)] }
}

pub fn sini8(n: i8) -> i8 {
    let nu = ((n as i32) + 128) as u8;
    let su8 = sinu8(nu);
    ((su8 as i32) - 128) as i8
}

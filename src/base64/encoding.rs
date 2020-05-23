// MIT License
//
// Copyright (c) 2020 Alexander Seifarth
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub fn encode_internal(bytes : &[u8], encoding_table: &[u8]) -> Vec<u8> {
    let size0 = bytes.len() / 3;
    let mut result = Vec::with_capacity(4 * size0 + 3);
    for idx in 0 .. size0 {
        let ridx = 3 * idx;
        let val = bytes_to_u32(0, bytes[ridx], bytes[ridx+1], bytes[ridx+2]);
        encode_triplet(val, &mut result, 3, &encoding_table);
    }
    match bytes.len() % 3 {
        0 => {},
        1 => {
            let val = bytes_to_u32(0, bytes[3 * size0], 0, 0);
            encode_triplet(val, &mut result, 1, &encoding_table);
            result.push(0x3d);
            result.push(0x3d);
        },
        2 => {
            let val = bytes_to_u32(0, bytes[3 * size0], bytes[3 * size0 +1], 0);
            encode_triplet(val, &mut result, 2, &encoding_table);
            result.push(0x3d);
        },
        _ => {panic!("mod 3 yields result >= 3: NOT WITH ME!")},
    }
    result
}

fn encode_triplet(value: u32, output: &mut Vec<u8>, count_to_encode: u32, encoding_table: &[u8]) -> () {
    for run in 0..(count_to_encode + 1) {
        output.push(encoding_table[((value >> (6 * (3-run))) & 0x3f) as usize]);
    }
}

fn bytes_to_u32(b3: u8, b2: u8, b1: u8, b0: u8) -> u32 {
    ((b3 as u32) << 24) + ((b2 as u32) << 16) + ((b1 as u32) << 8) + (b0 as u32)
}

pub const ENCODING_TABLE: &'static [u8] = &[
    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50,
    0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
    0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76,
    0x77, 0x78, 0x79, 0x7a, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x2b, 0x2f];

pub const ENCODING_TABLE_FILENAME_SAVE: &'static [u8] = &[
    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50,
    0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
    0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76,
    0x77, 0x78, 0x79, 0x7a, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x2d, 0x5f];

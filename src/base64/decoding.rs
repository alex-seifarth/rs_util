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

#[derive(Debug, PartialEq)]
pub enum DecodingError {

    /// invalid input character - associated data is the value that is invalid and its position in the input byte array
    InvalidChar{value: u8, index: usize},

    /// invalid number of input characters
    InvalidSize,

}

pub fn decode_internal(input: &[u8]) -> Result<Vec<u8>, DecodingError> {
    if input.is_empty() {
        return Ok(vec!{});
    }

    let (size0, missing_pad) = match input.len() % 4 {
        0 => (input.len()/4 -1, 0),
        x => (input.len()/4, 4 - x),
    };

    let mut result = Vec::with_capacity(input.len());

    for idx in 0..size0 {
        let ridx = 4 * idx;
        let r = decode_quadruplet_inner(input[ridx], input[ridx + 1],
                                        input[ridx + 2], input[ridx + 3],ridx, &mut result);
        match r {
            Ok(()) => {},
            Err(err) => return Err(err),
        }
    }

    let idx0 = 4 * size0;
    let (c0, c1, c2, c3) = match missing_pad {
        0 => (input[idx0], input[idx0 + 1], input[idx0 + 2], input[idx0 + 3]),
        1 => (input[idx0], input[idx0 + 1], input[idx0 + 2], 0x3d),
        2 => (input[idx0], input[idx0 + 1], 0x3d, 0x3d),
        3 => return Err(DecodingError::InvalidSize),
        _ => {panic!("mod 4 yields result >=4 : NOT WITH ME!")},
    };
    return match decode_quadruplet_last(c0, c1, c2, c3,idx0, &mut result) {
        Ok(()) => Ok(result),
        Err(err) => return Err(err),
    }
}

fn decode_quadruplet_inner(c0: u8, c1: u8, c2: u8, c3: u8, pos0: usize, output: &mut Vec<u8>) -> Result<(), DecodingError> {

    let mut value : u32;
    // if c0 == 0x3d || c1 == 0x3d {
    //     return Err(DecodingError::InvalidSize);
    // }
    // assert!((c2 == 0x3d && c3 == 0x3d) || (c2 != 0x3d), "quadruplet with '=' at [2] but not at [3]");
    match decode_singlet(c0, pos0) {
        Ok(d0) => {value = (d0 as u32) << 18},
        Err(err) => return Err(err),
    }
    match decode_singlet(c1, pos0 + 1) {
        Ok(d0) => {value = value + ((d0 as u32) << 12)},
        Err(err) => return Err(err),
    }
    match decode_singlet(c2, pos0 + 2) {
        Ok(d0) => {value = value + ((d0 as u32) << 6)},
        Err(err) => return Err(err),
    }
    match decode_singlet(c3, pos0 + 3) {
        Ok(d0) => {value = value + ((d0 as u32) << 0)},
        Err(err) => return Err(err),
    }
    output.push(((value >> 16) & 0xff) as u8);
    output.push(((value >> 8) & 0xff) as u8);
    output.push(((value >> 0) & 0xff) as u8);
    Ok(())
}

fn decode_quadruplet_last(c0: u8, c1: u8, c2: u8, c3: u8, pos0: usize, output: &mut Vec<u8>) -> Result<(), DecodingError> {

    let mut value : u32;
    if c0 == 0x3d || c1 == 0x3d {
         return Err(DecodingError::InvalidSize);
    }
    assert!((c2 == 0x3d && c3 == 0x3d) || (c2 != 0x3d), "quadruplet with '=' at [2] but not at [3]");
    match decode_singlet(c0, pos0) {
        Ok(d0) => {value = (d0 as u32) << 18},
        Err(err) => return Err(err),
    }
    match decode_singlet(c1, pos0 + 1) {
        Ok(d0) => {value = value + ((d0 as u32) << 12)},
        Err(err) => return Err(err),
    }
    match decode_singlet(c2, pos0 + 2) {
        Ok(d0) => {value = value + ((d0 as u32) << 6)},
        Err(DecodingError::InvalidChar{value:0x3d, index:_}) => {},
        Err(err) => return Err(err),
    }
    match decode_singlet(c3, pos0 + 3) {
        Ok(d0) => {value = value + ((d0 as u32) << 0)},
        Err(DecodingError::InvalidChar{value:0x3d, index:_}) => {},
        Err(err) => return Err(err),

    }
    output.push(((value >> 16) & 0xff) as u8);
    if c2 != 0x3d {
        output.push(((value >> 8) & 0xff) as u8);
    }
    if c3 != 0x3d {
        output.push(((value >> 0) & 0xff) as u8);
    }
    Ok(())
}

fn decode_singlet(ch: u8, position: usize) -> Result<u8, DecodingError> {
    match ch {
        0x41..=0x5a => Ok(ch - 0x41),           // A..Z
        0x61..=0x7a => Ok(ch - 0x61 + 0x1a),    // a..z
        0x30..=0x39 => Ok(ch - 0x30 + 0x34),    // 0..9
        0x2b => Ok(0x3e),                       // +
        0x2f => Ok(0x3f),                       // \
        0x2d => Ok(0x3e),                       // -
        0x5f => Ok(0x3f),                       // _
        _ => Err(DecodingError::InvalidChar {value:ch, index: position})
    }
}
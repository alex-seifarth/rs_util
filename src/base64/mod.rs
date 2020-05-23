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

use base64::decoding::DecodingError;

mod encoding;
mod decoding;

/// Encodes the bytes into canonical Base64 according RFC 4648
/// # Arguments
/// * 'bytes'   - Data to be encoded as Base64. Must be referencable as &[u8].
pub fn encode<T>(bytes: T) -> Vec<u8>
    where T: AsRef<[u8]>
{
    encoding::encode_internal(bytes.as_ref(), encoding::ENCODING_TABLE)
}

/// Encodes the byte array bytes into filename save Base64 according RFC 4648
/// # Notes
/// * Filename save Base64 avoids usage of the characters + and / and uses - and _ instead.
/// # Arguments
/// * 'bytes'   - Data to be encoded as Base64. Must be referencable as &[u8].
pub fn encode_fns<T>(bytes: T) -> Vec<u8>
     where T: AsRef<[u8]>
{
    encoding::encode_internal(bytes.as_ref(), encoding::ENCODING_TABLE_FILENAME_SAVE)
}

/// Decodes the input data as Base64 (canonical or filename safe) encoded data.
/// The method can handle missing padding characters.
/// The method cannot automatically remove new lines and interprets them as input data error.
pub fn decode<T>(bytes: T) -> Result<Vec<u8>, DecodingError>
    where T: AsRef<[u8]>
{
    decoding::decode_internal(bytes.as_ref())
}


#[cfg(test)]
mod tests {
    extern crate rand;

    use crate::base64;
    use self::rand::Rng;
    use base64::decoding::DecodingError;

    #[test]
    fn test_encode_rfc4648_test_vectors() {
        let rfc4648_test_vectors = vec![
            ("", ""), ("f", "Zg=="), ("fo", "Zm8="), ("foo", "Zm9v"), ("foob", "Zm9vYg=="), ("foobar", "Zm9vYmFy")
        ];

        for data in rfc4648_test_vectors.iter() {
            assert_eq!(std::str::from_utf8(base64::encode(data.0.as_bytes()).as_slice()).unwrap(), data.1, "input vector {}", data.0);
        }
    }

    #[test]
    fn test_decode_empty() {
        let r0 = base64::decode(b"");
        assert_eq!(r0, Ok(vec!{}));
    }

    #[test]
    fn test_decode_rfc4648_test_vectors() {
        let rfc4648_test_vectors = vec![
            ("", ""), ("f", "Zg=="), ("fo", "Zm8="), ("foo", "Zm9v"), ("foob", "Zm9vYg=="), ("foobar", "Zm9vYmFy")
        ];

        for data in rfc4648_test_vectors.iter() {
            assert_eq!(std::str::from_utf8(base64::decode(data.1.as_bytes()).unwrap().as_slice()).unwrap(), data.0, "input vector {}", data.1);
        }
    }

    #[test]
    fn test_decode_incomplete_padding() {
        assert_eq!(base64::decode(b"Zg="), Ok(b"f".to_vec()));
        assert_eq!(base64::decode(b"Zg"), Ok(b"f".to_vec()));

        assert_eq!(base64::decode(b"Zm9vYg="), Ok(b"foob".to_vec()));
        assert_eq!(base64::decode(b"Zm9vYg"), Ok(b"foob".to_vec()));
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let data = vec!["", "Alexander Seifarth", "ioa\ndkdi", "Enjoy the Silence", "09a 9./9d+"];

        for d in data.iter() {
            let enc = base64::encode(d.as_bytes());
            let dec = base64::decode(enc.as_slice());
            assert_eq!(dec, Ok(d.as_bytes().to_vec()));
        }
    }

    #[test]
    fn test_encode_decode_roundtrip_random_binary() {
        let side = self::rand::distributions::Uniform::new_inclusive(0, 255);

        let rng = rand::thread_rng();
        let n1 : Vec<u8> = rng.sample_iter(&side).take(500).collect();
        let enc = base64::encode(&n1);
        let dec = base64::decode(enc.as_slice());
        assert_eq!(dec, Ok(n1));
    }

    #[test]
    fn test_decode_invalid_input() {
        let input : Vec<u8> = vec![0x41, 0x42, 0x43, 0x99, 0x3d];
        assert_eq!(base64::decode(input.as_slice()), Err(DecodingError::InvalidChar{value: 0x99, index: 3}));

        assert_eq!(base64::decode(b"Zg==="), Err(DecodingError::InvalidChar{value: 0x3d, index: 2}));
    }
}

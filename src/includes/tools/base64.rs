use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Base64Error(String);

impl fmt::Display for Base64Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for Base64Error {}

const BASE64_CHARS_STANDARD: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                       abcdefghijklmnopqrstuvwxyz\
                                       0123456789+/";

const BASE64_CHARS_URL: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                  abcdefghijklmnopqrstuvwxyz\
                                  0123456789-_";

fn pos_of_char(chr: u8) -> Result<u32, Base64Error> {
    match chr {
        b'A'..=b'Z' => Ok((chr - b'A') as u32),
        b'a'..=b'z' => Ok((chr - b'a' + 26) as u32),
        b'0'..=b'9' => Ok((chr - b'0' + 52) as u32),
        b'+' | b'-' => Ok(62),
        b'/' | b'_' => Ok(63),
        _ => Err(Base64Error("Input is not valid base64-encoded data.".to_string())),
    }
}

fn insert_linebreaks(s: String, distance: usize) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut result = String::with_capacity(s.len() + s.len() / distance);
    let mut pos = 0;

    for chunk in s.as_bytes().chunks(distance) {
        if pos > 0 {
            result.push('\n');
        }
        result.push_str(std::str::from_utf8(chunk).unwrap());
        pos += distance;
    }

    result
}

pub fn base64_encode(bytes_to_encode: &[u8], url: bool) -> String {
    let len_encoded = (bytes_to_encode.len() + 2) / 3 * 4;
    let trailing_char = if url { b'.' } else { b'=' };
    let base64_chars = if url { BASE64_CHARS_URL } else { BASE64_CHARS_STANDARD };

    let mut ret = String::with_capacity(len_encoded);
    let in_len = bytes_to_encode.len();
    let mut pos = 0;

    while pos < in_len {
        ret.push(base64_chars[((bytes_to_encode[pos] & 0xfc) >> 2) as usize] as char);

        if pos + 1 < in_len {
            ret.push(
                base64_chars[(((bytes_to_encode[pos] & 0x03) << 4)
                    + ((bytes_to_encode[pos + 1] & 0xf0) >> 4)) as usize] as char,
            );

            if pos + 2 < in_len {
                ret.push(
                    base64_chars[(((bytes_to_encode[pos + 1] & 0x0f) << 2)
                        + ((bytes_to_encode[pos + 2] & 0xc0) >> 6)) as usize] as char,
                );
                ret.push(base64_chars[(bytes_to_encode[pos + 2] & 0x3f) as usize] as char);
            } else {
                ret.push(base64_chars[((bytes_to_encode[pos + 1] & 0x0f) << 2) as usize] as char);
                ret.push(trailing_char as char);
            }
        } else {
            ret.push(base64_chars[((bytes_to_encode[pos] & 0x03) << 4) as usize] as char);
            ret.push(trailing_char as char);
            ret.push(trailing_char as char);
        }

        pos += 3;
    }

    ret
}

pub fn base64_decode(encoded_string: &str, remove_linebreaks: bool) -> Result<String, Base64Error> {
    if encoded_string.is_empty() {
        return Ok(String::new());
    }

    let encoded_string = if remove_linebreaks {
        encoded_string.replace('\n', "")
    } else {
        encoded_string.to_string()
    };

    let length_of_string = encoded_string.len();
    let mut pos = 0;

    let approx_length_of_decoded_string = length_of_string / 4 * 3;
    let mut ret = Vec::with_capacity(approx_length_of_decoded_string);

    while pos < length_of_string {
        let bytes = encoded_string.as_bytes();
        
        let pos_of_char_1 = pos_of_char(bytes[pos + 1])?;

        ret.push((((pos_of_char(bytes[pos])?) << 2) + ((pos_of_char_1 & 0x30) >> 4)) as u8);

        if pos + 2 < length_of_string
            && bytes[pos + 2] != b'='
            && bytes[pos + 2] != b'.'
        {
            let pos_of_char_2 = pos_of_char(bytes[pos + 2])?;
            ret.push((((pos_of_char_1 & 0x0f) << 4) + ((pos_of_char_2 & 0x3c) >> 2)) as u8);

            if pos + 3 < length_of_string
                && bytes[pos + 3] != b'='
                && bytes[pos + 3] != b'.'
            {
                ret.push((((pos_of_char_2 & 0x03) << 6) + pos_of_char(bytes[pos + 3])?) as u8);
            }
        }

        pos += 4;
    }

    String::from_utf8(ret).map_err(|e| Base64Error(format!("UTF-8 conversion error: {}", e)))
}

pub fn base64_encode_string(s: &str, url: bool) -> String {
    base64_encode(s.as_bytes(), url)
}

pub fn base64_encode_pem(s: &str) -> String {
    insert_linebreaks(base64_encode_string(s, false), 64)
}

pub fn base64_encode_mime(s: &str) -> String {
    insert_linebreaks(base64_encode_string(s, false), 76)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let original = "Hello, World!";
        let encoded = base64_encode_string(original, false);
        let decoded = base64_decode(&encoded, false).unwrap();
        assert_eq!(original, decoded);
    }
}
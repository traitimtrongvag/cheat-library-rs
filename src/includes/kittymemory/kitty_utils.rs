use std::fmt::Write;
use std::str;


pub fn trim_string(s: &mut String) {
    s.retain(|c| {
        !(c == ' ' || c == '\n' || c == '\r' || c == '\t' || c == '\x0b' || c == '\x0c')
    });
}


pub fn validate_hex_string(hex: &mut String) -> bool {
    if hex.is_empty() {
        return false;
    }

  
    if hex.starts_with("0x") {
        hex.replace_range(0..2, "");
    }
    trim_string(hex);

    if hex.len() < 2 || hex.len() % 2 != 0 {
        return false;
    }

    hex.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn to_hex(data: &[u8]) -> String {
    let mut out = String::new();
    for b in data {
        write!(&mut out, "{:02x}", b).unwrap();
    }
    out
}

pub fn from_hex(hex: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(hex.len() / 2);

    let bytes = hex.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if i + 1 >= bytes.len() {
            break;
        }

        let h = bytes[i] as char;
        let l = bytes[i + 1] as char;

        let byte_str = format!("{}{}", h, l);
        let value = u8::from_str_radix(&byte_str, 16).unwrap_or(0);

        result.push(value);
        i += 2;
    }

    result
}

pub fn hex_dump(data: &[u8], row_size: usize, show_ascii: bool) -> String {
    if data.is_empty() || row_size == 0 {
        return "".to_string();
    }

    let mut out = String::new();
    let len = data.len();

    for i in (0..len).step_by(row_size) {
        write!(&mut out, "{:08X}: ", i).unwrap();

        for j in 0..row_size {
            if i + j < len {
                write!(&mut out, "{:02X} ", data[i + j]).unwrap();
            } else {
                out.push_str("   ");
            }
        }

        if show_ascii {
            out.push(' ');
            for j in 0..row_size {
                if i + j < len {
                    let c = data[i + j];
                    if c.is_ascii_graphic() || c == b' ' {
                        out.push(c as char);
                    } else {
                        out.push('.');
                    }
                }
            }
        }

        out.push('\n');
    }

    out
}
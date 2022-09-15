use std::collections::HashMap;

fn pad_zeroes(mut input: String) -> String {
    while input.len() % 6 != 0 {
        input.push('0');
    }
    input
}

fn base64_padding(mut input: String) -> String {
    let mut base64_len = input.len() * 6;
    while base64_len % 8 != 0 {
        base64_len += 6;
        input.push_str("=");
    }
    input
}

fn binary_string_to_u8(input: String) -> u8 {
    let mut result = 0;
    let mut factor = 1;
    for ch in input.chars().rev() {
        result += if ch == '1' { factor } else { 0 };
        if factor < 128 {
            factor *= 2;
        }
    }
    result
}

fn char_to_u8(ch: char) -> u8 {
    match ch {
        '0'..='9' => ((ch as i8) - 48) as u8,
        'a' | 'A' => 10,
        'b' | 'B' => 11,
        'c' | 'C' => 12,
        'd' | 'D' => 13,
        'e' | 'E' => 14,
        'f' | 'F' => 15,
        _ => 0,
    }
}

fn u8_to_base64_char() -> HashMap<u8, char> {
    let mut mapping: HashMap<u8, char> = HashMap::new();

    for (idx, val) in ('A'..='Z').enumerate() {
        mapping.insert(idx as u8, val);
    }

    for (idx, val) in ('a'..='z').enumerate() {
        mapping.insert((idx + 26) as u8, val);
    }

    for (idx, val) in ('0'..='9').enumerate() {
        mapping.insert((idx + 52) as u8, val);
    }

    mapping.insert(62, '+');
    mapping.insert(63, '/');

    mapping
}

fn base64_char_to_u8() -> HashMap<char, u8> {
    let mut mapping: HashMap<char, u8> = HashMap::new();

    for (idx, val) in ('A'..='Z').enumerate() {
        mapping.insert(val, idx as u8);
    }

    for (idx, val) in ('a'..='z').enumerate() {
        mapping.insert(val, (idx + 26) as u8);
    }

    for (idx, val) in ('0'..='9').enumerate() {
        mapping.insert(val, (idx + 52) as u8);
    }

    mapping.insert('+', 62);
    mapping.insert('/', 63);

    mapping
}

pub fn binary_string_to_base64(mut bin_string: String) -> String {
    let mapping = u8_to_base64_char();
    bin_string = pad_zeroes(bin_string);

    let mut base64_string = String::new();
    let mut idx = 0;
    while idx < bin_string.len() {
        let temp_str = &bin_string[idx..(idx + 6)];
        let base64_char = binary_string_to_u8(temp_str.to_string());
        base64_string.push(*mapping.get(&base64_char).unwrap());
        idx += 6;
    }

    return base64_padding(base64_string);
}

pub fn ascii_to_base64(input: &str) -> String {
    let mut bin_string = String::new();
    for b in input.as_bytes().iter() {
        bin_string.push_str(&format!("{:08b}", b));
    }

    binary_string_to_base64(bin_string)
}

pub fn hex_to_base64(input: &str) -> String {
    let mut bin_string = String::new();
    for ch in input.chars() {
        bin_string.push_str(&format!("{:04b}", char_to_u8(ch)));
    }

    binary_string_to_base64(bin_string)
}

pub fn base64_to_ascii(input: String) -> String {
    let mut bin_string = String::new();

    let mapping = base64_char_to_u8();
    for ch in input.chars() {
        if ch != '=' {
            bin_string.push_str(&format!("{:06b}", *mapping.get(&ch).unwrap()));
        }
    }

    while bin_string.len() % 8 != 0 {
        bin_string.push('0');
    }

    let mut idx = 0;
    let mut ascii_string = String::new();
    while idx < bin_string.len() {
        let temp_str = &bin_string[idx..(idx + 8)];
        ascii_string.push_str(&format!(
            "{}",
            binary_string_to_u8(temp_str.to_string()) as char
        ));
        idx += 8;
    }

    ascii_string
}

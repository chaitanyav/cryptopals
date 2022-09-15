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

pub fn hex_to_char(input: String) -> String {
    let mut res = String::new();

    let mut binary_string = String::new();
    for ch in input.chars() {
        binary_string.push_str(&format!("{:04b}", char_to_u8(ch)));
    }

    let mut idx = 0;
    while idx < binary_string.len() {
        let temp_str = &binary_string[idx..(idx + 8)];
        let temp_u8 = binary_string_to_u8(temp_str.to_string());
        let temp_ch = temp_u8 as char;
        res.push(temp_ch);
        idx += 8;
    }

    res
}

// input1, input2 are strings of equal length
pub fn xor_strings(input1: String, input2: String) -> String {
    let mut res = String::new();

    let mut idx = 0;

    while idx < input1.len() {
        let ch1 = input1.chars().nth(idx).unwrap();
        let ch2 = input2.chars().nth(idx).unwrap();

        res.push_str(&format!("{:x}", char_to_u8(ch1) ^ char_to_u8(ch2)));
        idx += 1;
    }

    res
}

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

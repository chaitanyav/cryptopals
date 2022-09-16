pub mod base64;
pub mod xor;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let ascii_base64 =
        crate::base64::ascii_to_base64("The quick brown fox jumps over the lazy dog.");
    println!("{}", ascii_base64);
    let hex_base64 = crate::base64::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("{}", hex_base64);

    println!(
        "{}",
        crate::base64::base64_to_ascii(String::from("bGlnaHQgd29y"))
    );

    println!(
        "{}",
        crate::base64::base64_to_ascii(String::from("bGlnaHQgdw=="))
    );

    // Set-1 Challenge-1
    println!(
        "{}",
        crate::base64::base64_to_ascii(String::from(
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        ))
    );

    println!(
        "{}",
        crate::base64::base64_to_ascii(String::from("bGlnaHQgd29yay4="))
    );

    // Set-1 Challenge-2
    let xor_string = crate::xor::xor_strings(
        String::from("1c0111001f010100061a024b53535009181c"),
        String::from("686974207468652062756c6c277320657965"),
    );
    assert!(xor_string.eq(&String::from("746865206b696420646f6e277420706c6179")));

    // Set-1 Challenge 3
    let encoded_string =
        String::from("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    println!(
        "{}",
        crate::xor::decode_single_character_xor(&encoded_string, 'X')
    );

    // Set-1 Challenge 4
    let mut max_score = 0;
    let mut orig_msg = String::from("");
    match read_lines("data/4.txt") {
        Ok(lines) => {
            for (line_num, line) in lines.enumerate() {
                if let Ok(encoded_string) = line {
                    for byte in 1 as u8..=255 {
                        let decoded_string =
                            crate::xor::decode_single_character_xor(&encoded_string, byte as char);
                        let score = decoded_string
                            .chars()
                            .filter(|ch| ch.is_ascii_whitespace() || ch.is_alphabetic())
                            .count();
                        if score > max_score {
                            //println!( "{line_num} character is {} {}",byte as char, decoded_string);
                            max_score = score;
                            orig_msg = decoded_string;
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Error reading the file {:?}", e);
        }
    }

    println!("{} {}", orig_msg, max_score);
}

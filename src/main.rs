pub mod base64;

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
}

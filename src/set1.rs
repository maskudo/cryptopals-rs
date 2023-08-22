use std::error::Error;

static BASE_64_TABLE: &[u8] =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
enum Status {
    Take0,
    Take2,
    Take4,
}
use Status::*;

pub fn hex_to_base64(hex: &str) -> Result<String, Box<dyn Error>> {
    if hex.len() % 2 != 0 {
        return Err(From::from("Invalid hex string"));
    }
    let bytes = hex::decode(hex)?;
    println!("bytes: {:?}", bytes);
    let mut res = Vec::new();
    let mut index = 0u8;
    let mut status = Take0;

    for byte in bytes {
        match status {
            Take0 => {
                index = (byte & 0b11111100) >> 2;
                res.push(BASE_64_TABLE[index as usize]);
                index = (byte & 0b00000011) << 4;
                status = Take4;
            }
            Take2 => {
                index |= (byte & 0b11000000) >> 6;
                res.push(BASE_64_TABLE[index as usize]);
                index = byte & 0b00111111;
                res.push(BASE_64_TABLE[index as usize]);
                status = Take0;
            }
            Take4 => {
                index |= (byte & 0b11110000) >> 4;
                res.push(BASE_64_TABLE[index as usize]);
                index = (byte & 0b00001111) << 2;
                status = Take2;
            }
        }
    }
    match status {
        Take0 => {}
        Take2 => {
            println!("{index}");
            res.push(BASE_64_TABLE[index as usize]);
            res.push(b'=');
        }
        Take4 => {
            res.push(BASE_64_TABLE[index as usize]);
            res.push(b'=');
            res.push(b'=');
        }
    }
    Ok(std::str::from_utf8(&res)?.to_owned())
}
#[cfg(test)]
mod test {
    use crate::set1::hex_to_base64;

    #[test]
    fn test_hex_to_base64() {
        let tests = [
            ("000000","AAAA"),
            ("00", "AA=="),
            ("12", "Eg=="),
            ("49", "SQ=="),
            ("49276d", "SSdt"),
            ("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d", "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
        ];
        for (hex, b64) in tests {
            assert_eq!(hex_to_base64(hex).unwrap(), String::from(b64));
        }
    }
}

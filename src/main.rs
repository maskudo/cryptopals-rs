use cryptopals::set1::hex_to_base64;

fn main() {
    println!("{}", hex_to_base64("49276d").unwrap());
}

fn base62_encode(mut num: u64) -> String {
    let charset = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut encoded = String::new();
    while num > 0 {
        let remainder = (num % 62) as usize;
        encoded.push(charset.chars().nth(remainder).unwrap());
        num /= 62;
    }
    encoded.chars().rev().collect()
}

fn main() {
    let id: u64 = 12345;
    let short_url = base62_encode(id);
    println!("Short URL: {}", short_url);
}

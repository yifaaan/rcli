use rand::prelude::*;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = thread_rng();
    let mut password = String::new();
    let mut chars = vec![];

    if upper {
        chars.extend_from_slice(UPPER);
    }
    if lower {
        chars.extend_from_slice(LOWER);
    }
    if number {
        chars.extend_from_slice(NUMBER);
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
    }

    for _ in 0..length {
        let idx = rng.gen_range(0..chars.len());
        password.push(chars[idx] as char);
    }
    println!("{:?}", password);
    Ok(())
}

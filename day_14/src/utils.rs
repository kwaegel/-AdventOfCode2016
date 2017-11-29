
use std;
use std::fmt::Write;

pub fn to_hex_string(bytes: &[u8; 16]) -> Result<String, std::fmt::Error> {

    let mut s = String::new();
    for &byte in bytes.iter() {
        write!(&mut s, "{:02x}", byte)?;
    }
    Ok(s)
}

pub fn get_first_triplet(hash: &str) -> Option<char> {
    let bytes = hash.as_bytes();
    for i in 1..bytes.len() - 1 {
        if bytes[i - 1] == bytes[i] && bytes[i] == bytes[i + 1] {
            return Some(bytes[i] as char);
        }
    }
    None
}

pub fn get_quintuplets(hash: &str) -> Vec<char> {
    let bytes = hash.as_bytes();
    let mut results = Vec::new();
    for i in 0..bytes.len() - 5 {
        if bytes[i] == bytes[i + 1] && bytes[i] == bytes[i + 2] && bytes[i] == bytes[i + 3] &&
            bytes[i] == bytes[i + 4]
            {
                results.push(bytes[i] as char);
            }
    }
    results.sort();
    results.dedup();
    results
}


#[cfg(test)]
mod test {

    use utils::*;

    #[test]
    fn test_sets() {

        let set1 = get_quintuplets("123aaaaacde");
        assert_eq!(set1[0], 'a');

        let set2 = get_quintuplets("123bbbbbcdeeeeeff");
        assert_eq!(set2[0], 'b');
        assert_eq!(set2[1], 'e');

        let set3 = get_triplets("347dac6ee8eeea4652c7476d0f97bee5");
        assert_eq!(set3[0], 'e');

        let set4 = get_quintuplets("3aeeeee1367614f3061d165a5fe3cac3");
        assert_eq!(set4[0], 'e');

        //hash('abc39') = 347dac6ee8eeea4652c7476d0f97bee5
        //hash('abc816') = 3aeeeee1367614f3061d165a5fe3cac3

    }
}


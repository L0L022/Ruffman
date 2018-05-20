use std::collections::BTreeMap;
use std::io::Read;
use std::io::Error;

pub type CharactersFrequency = BTreeMap<u8, u64>;


pub fn count_occurrences(stream: &mut Read) -> Result<CharactersFrequency, Error> {
    let mut map = BTreeMap::new();
    
    for c in 1..=255 {
        map.insert(c, 0);
    }
    
    /*
    let mut character: [u8; 1] = [0];
    loop {
        match stream.read(&mut character) {
            Ok(_) => {
                match map.get_mut(&character[0]) {
                    Some(count) => *count += 1,
                    None => ()
                }
            },
            Err(_) => break,
        }
    }
    */
    
    
    for byte in stream.bytes() {
        match byte {
            Ok(byte) => {
                match map.get_mut(&byte) {
                    Some(count) => *count += 1,
                    None => () // panic
                }
            },
            Err(e) => return Err(e),
        }
    }
    
    Ok(map)
}

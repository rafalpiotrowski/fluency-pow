use sha2::{Digest, Sha256};

/// data structure to hold return values of our pow calculation
#[derive(Debug, PartialEq)]
struct Pow {
    hash: String,
    prefix: String
}

/// at the moment program will panic if input string is not valid 
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_string = &args[1];

    let r = pow(input_string).unwrap_or(Pow { hash: "none".to_string(), prefix: "none".to_string()});

    println!("{}\n{}", r.hash, r.prefix);
}

/// brute force looping though all values from [0,0,0,0] to [255, 255, 255, 255]
fn pow(input_string: &String) -> Option<Pow> {
    let input_as_bytes = &hex::decode(input_string).unwrap()[..];

    for b1 in 0..256 {
        for b2 in 0..256 {
            for b3 in 0..256 {
                for b4 in 0..256 {
                    let r = pow_with_prefix(input_as_bytes, &vec![b1 as u8, b2 as u8, b3 as u8, b4 as u8][..]);
                    match r {
                        Some(_) => return r,
                        None => continue
                    }
                }
            }
        }
    }

    None
}

/// this is function which makes all the calculations and checking the last two bytes 
/// if we get the expected result
fn pow_with_prefix(input: &[u8], prefix: &[u8]) -> Option<Pow> {
    let mut hasher = Sha256::new_with_prefix(prefix);
    hasher.update(input);
    let res = &hasher.finalize()[..];
    // ca = 202
    let second_last_byte = &res[&res.len() - 2];
    // fe = 254
    let last_byte = &res[&res.len() - 1];

    if *second_last_byte == 202 && *last_byte == 254 {
        return Some(Pow {
            hash: hex::encode(res),
            prefix: hex::encode(prefix)
        });
    } 

    None
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn input_from_example() {
      let r= pow(&"129df964b701d0b8e72fe7224cc71643cf8e000d122e72f742747708f5e3bb6294c619604e52dcd8f5446da7e9ff7459d1d3cefbcc231dd4c02730a22af9880c".to_string());
      let expected_result = Pow {
          hash: "6681edd1d36af256c615bf6dcfcda03c282c3e0871bd75564458d77c529dcafe".to_string(),
          prefix: "00003997".to_string()
      };
      assert_eq!(r, Some(expected_result));
  }

  #[test]
  fn wrong_input() {
      //changed second char from 2 to 1
      let r= pow(&"119df964b701d0b8e72fe7224cc71643cf8e000d122e72f742747708f5e3bb6294c619604e52dcd8f5446da7e9ff7459d1d3cefbcc231dd4c02730a22af9880c".to_string());
      let expected_result = Pow {
          hash: "6681edd1d36af256c615bf6dcfcda03c282c3e0871bd75564458d77c529dcafe".to_string(),
          prefix: "00003997".to_string()
      };
      assert_ne!(r, Some(expected_result));
  }

  #[test]
  #[should_panic]
  fn incorrect_input_should_panic() {
      //changed second char from 2 to 1
      let r= pow(&"s".to_string());
      let expected_result = Pow {
          hash: "6681edd1d36af256c615bf6dcfcda03c282c3e0871bd75564458d77c529dcafe".to_string(),
          prefix: "00003997".to_string()
      };
      assert_ne!(r, Some(expected_result));
  }
}
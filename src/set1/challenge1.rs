use super::super::{ EncodingType, EncodingError, Bytes };


use wasm_bindgen::prelude::*;

/// Runs cryptopals' set 1 challenge 1 <https://cryptopals.com/sets/1/challenges/1>
/// 
/// # Examples
/// ```
/// # use cryptopals_rust::set1::challenge1::{ challenge };
/// assert_eq!(
///     challenge("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap(), 
///     "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string()
/// );
/// ```
#[doc(alias = "hexadecimal")]
#[doc(alias = "base 64")]
#[wasm_bindgen]
pub fn challenge(val: &str) -> Option<String>{
    let b64 = match hex_string_to_b64(&val){       //converting given string (expectedly hex string) into base64
        Ok(r)  => r,
        Err(e) => {
            eprintln!("Error when converting hex string to base 64 : {}", e);
            return None;
        }
    };
    Some(b64)
}


/// Converts a hex string into a base 64 encoded string
#[doc(alias = "hexadecimal")]
#[doc(alias = "base 64")]
fn hex_string_to_b64(s: &str) -> Result<String, EncodingError>{
    let bytes:Bytes = hex_string_to_bytes(s)?;                //converting given hex string into byte equivalents. e.g : "aa" would be x'aa', or b'10101010', or 170_u8
    let b64:String = b64_encode(bytes);                        //encoding bytes into base64 string
    Ok(b64)
}

/// Converts a hex string into a list of bytes.
#[doc(alias = "hexadecimal")]
fn hex_string_to_bytes(hex: &str) -> Result<Bytes, EncodingError>{
    
    if hex.len()%2 != 0 || hex.len() != hex.chars().count() {
        //String::len returns the length of the string in bytes, not the number of characters.
        //The character encoding is UTF-8, characters are not all the same size.
        //All hex characters (0123456789, abcdef, ABCDEF) should all be 1 byte long.
        //So, if all characters are indeed hex characters, the byte length should be the same as character length
        //And every 2 hex character encodes a byte
        //So the length of the byte vector should be half that of the hex string
        //Thus, so should its capacity if we want to optimise memory usage
        return Err(EncodingError{encoding: EncodingType::Hexadecimal, message: "Hex string should have an even amount of characters".to_string()});
    }
    let mut r:Bytes = Bytes::with_capacity(hex.len()/2);
    for byte_hex in hex.chars().collect::<Vec<char>>().windows(2).step_by(2) {
        r.push(make_byte_from_hex(byte_hex[0], byte_hex[1])?);          //second byte => 4 least signifiant bits. leave them in the least significant half. merge (bitwise OR) with other half.
    }
    Ok(r)
}

//NOTE : Found this. Should do the same thing (with fewer checks) :
    // let mut bytes = Vec::new();
    //     for i in 0..(hex.len()/2) {
    //         let res = u8::from_str_radix(&hex[2*i .. 2*i+2], 16);        << Secret sauce here
    //         match res {
    //             Ok(v) => bytes.push(v),
    //             Err(e) => println!("Problem with hex: {}", e),
    //         };
    //     };
    // }









/// Converts two hex digits into a byte
#[doc(alias = "hexadecimal")]
fn make_byte_from_hex(hex1: char, hex2: char) -> Result<u8, EncodingError> {
    let msb = (hex1
        .to_digit(16)
        .ok_or(
            EncodingError{encoding: EncodingType::Hexadecimal, message: format!("found non hexadecimal digit '{}'", hex1)}
        )? as u8) << 4;
    let lsb = hex2
        .to_digit(16)
        .ok_or(
            EncodingError{encoding: EncodingType::Hexadecimal, message: format!("found non hexadecimal digit '{}'", hex2)}
        )? as u8;
    Ok(msb | lsb)
}



/*
 *  Base 64 encoding ahead
 * 
 */


/// Encodes a slice of bytes into a base 64 encoded string.
#[doc(alias = "base 64")]
fn b64_encode(bytes: Bytes) -> String {
    let b64_word_count = (((bytes.len() as f64)/3.) * 4.).ceil() as usize;

    let mut b64_words:Bytes = Bytes::with_capacity(b64_word_count);

    let mut padding:u8 = 0;

    for i in 0..b64_word_count {
        let start_bit = (i*6)%24;
        let end_bit = start_bit + 5;
        let b1:u8 = bytes[(i/4)*3 + start_bit/8];
        let b2:u8;
        if bytes.len() > ((i/4)*3 + end_bit/8) {
            b2 = bytes[(i/4)*3 + end_bit/8];
        }
        else {
            b2 = 0;
            if i%2 == 0{
                padding = 1;
            }
            else {
                padding = 2;
            }
        }
        let window_shift = (((i+1)*2) % 8) as u8;
        b64_words.push(make_b64_char(b1, b2, window_shift));
    }


    let mut out:String = String::new();

    for word in b64_words.iter() {
        out.push(b64_word_to_char(*word).expect("found value > 63"));
    }

    for _ in 0..padding {
        out.push('=');
    }

    out
}




/// Converts a base 64 value (0 - 63) into its corresponding character
#[doc(alias = "base 64")]
fn b64_word_to_char(word: u8) -> Result<char, EncodingError> {
    match word {
        0..=25  => Ok((word + 65) as char),
        26..=51 => Ok((word + 71) as char),
        52..=61 => Ok((word - 4) as char),
        62      => Ok('+'),
        63      => Ok('/'),
        _       => Err(EncodingError {encoding: EncodingType::Hexadecimal, message: format!("given value {} greater than max valid value 63", word)}),
  }
}

/// Turns 2 "source" bytes into a single base 64 byte. closely tied to the [`b64_encode`] function
#[doc(alias = "base 64")]
fn make_b64_char(b1: u8, b2: u8, window_slide: u8) -> u8 {
    ((((b1 as u16) << 8 | (b2 as u16)) >> window_slide) as u8) & 0b00111111
}


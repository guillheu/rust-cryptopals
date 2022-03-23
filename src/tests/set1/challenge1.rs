extern crate rstest;

use crate::set1::challenge1;
use rstest::*;



#[rstest]
#[case("Testing 1 byte, value 0", "00", Some("AA==".to_string()))]
#[case("Testing more bytes (10), value 0", "00000000000000000000", Some("AAAAAAAAAAAAAA==".to_string()))]
#[case("Testing all decimal numbers", "0123456789", Some("ASNFZ4k=".to_string()))]
#[case("Testing all hex lowercase letter digits", "abcdef", Some("q83v".to_string()))]
#[case("Testing all hex uppercase letter digits", "ABCDEF", Some("q83v".to_string()))]
#[case("Testing all allowed hex values", "0123456789abcdefABCDEF", Some("ASNFZ4mrze+rze8=".to_string()))]
#[case("Testing an empty string", "", Some("".to_string()))]
#[case("Testing invalid input : odd amount of hex digits", "0", None)]
#[case("Testing invalid input : lowercase non-hex digit letter", "0g",None)]
#[case("Testing invalid input : uppercase non-hex digit letter", "0G", None)]
#[case("Testing invalid input : emojis (single character, multiple bytes)", "💎💎", None)]
fn hex_strings_to_b64_test(#[case] description: &str, #[case] input_hex_string: &str, #[case] expected_result: Option<String>) {

    println!("{}", description);

    let actual_result = challenge1::challenge(input_hex_string);

    assert_eq!(actual_result, expected_result);
}
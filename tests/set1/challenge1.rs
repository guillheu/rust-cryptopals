extern crate rstest;

use cryptopals_rust::set1::challenge1::*;
use rstest::*;




#[rstest]
#[case("Testing 1 byte, value 0", "00", Some("AA==".to_string()))]
#[case("Testing more bytes (10), value 0",                                          "00000000000000000000",                                                                                 Some("AAAAAAAAAAAAAA==".to_string()))]
#[case("Testing all decimal numbers",                                               "0123456789",                                                                                           Some("ASNFZ4k=".to_string()))]
#[case("Testing all hex lowercase letter digits",                                   "abcdef",                                                                                               Some("q83v".to_string()))]
#[case("Testing all hex uppercase letter digits",                                   "ABCDEF",                                                                                               Some("q83v".to_string()))]
#[case("Testing all allowed hex values",                                            "0123456789abcdefABCDEF",                                                                               Some("ASNFZ4mrze+rze8=".to_string()))]
#[case("Testing an empty string",                                                   "",                                                                                                     Some("".to_string()))]
#[case("Testing invalid input : odd amount of hex digits",                          "0",                                                                                                    None)]
#[case("Testing invalid input : lowercase non-hex digit letter",                    "0g",                                                                                                   None)]
#[case("Testing invalid input : uppercase non-hex digit letter",                    "0G",                                                                                                   None)]
#[case("Testing invalid input : emojis (single character, multiple bytes)",         "💎💎",                                                                                                 None)]
#[case("Testing cryptopals' input string",                                          "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",     Some("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string()))]
/// will test the implementation for cryptopals' set 1-challenge 1 (https://cryptopals.com/sets/1/challenges/1)
fn hex_strings_to_b64_test(#[case] description: &str, #[case] input_hex_string: &str, #[case] expected_result: Option<String>) {

    println!("{}", description);

    let actual_result = challenge(input_hex_string);

    assert_eq!(actual_result, expected_result);
}
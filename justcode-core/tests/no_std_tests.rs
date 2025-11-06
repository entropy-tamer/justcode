//! Integration tests for no-std functionality
//! 
//! Note: These tests require the no-std-test feature to be enabled
//! Run with: cargo test --features no-std-test --test no_std_tests

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
use justcode_core::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
struct SimpleStruct {
    value: u32,
    flag: bool,
}

#[cfg(not(feature = "std"))]
#[test]
fn test_no_std_vec_encode_decode() {
    let config = config::standard();
    
    let value = alloc::vec![1u32, 2, 3, 4, 5];
    let encoded = justcode_core::encode_to_vec(&value, config).unwrap();
    let (decoded, _): (Vec<u32>, usize) = justcode_core::decode_from_slice(&encoded, config).unwrap();
    
    assert_eq!(value, decoded);
}

#[cfg(not(feature = "std"))]
#[test]
fn test_no_std_empty_vec() {
    let config = config::standard();
    
    let value: Vec<u32> = Vec::new();
    let encoded = justcode_core::encode_to_vec(&value, config).unwrap();
    let (decoded, _): (Vec<u32>, usize) = justcode_core::decode_from_slice(&encoded, config).unwrap();
    
    assert_eq!(value, decoded);
}

#[cfg(not(feature = "std"))]
#[test]
fn test_no_std_struct_with_vec() {
    let config = config::standard();
    
    let data = SimpleStruct {
        value: 42,
        flag: true,
    };
    
    let vec_data = alloc::vec![data, data];
    let encoded = justcode_core::encode_to_vec(&vec_data, config).unwrap();
    let (decoded, _): (Vec<SimpleStruct>, usize) = justcode_core::decode_from_slice(&encoded, config).unwrap();
    
    assert_eq!(vec_data, decoded);
}

#[cfg(not(feature = "std"))]
#[test]
fn test_no_std_option_with_vec() {
    let config = config::standard();
    
    let value: Option<Vec<u32>> = Some(alloc::vec![1, 2, 3]);
    let encoded = justcode_core::encode_to_vec(&value, config).unwrap();
    let (decoded, _): (Option<Vec<u32>>, usize) = justcode_core::decode_from_slice(&encoded, config).unwrap();
    
    assert_eq!(value, decoded);
}

#[cfg(not(feature = "std"))]
#[test]
fn test_no_std_large_vec() {
    let config = config::standard();
    
    let mut value = Vec::new();
    for i in 0..100 {
        value.push(i as u32);
    }
    
    let encoded = justcode_core::encode_to_vec(&value, config).unwrap();
    let (decoded, _): (Vec<u32>, usize) = justcode_core::decode_from_slice(&encoded, config).unwrap();
    
    assert_eq!(value, decoded);
}


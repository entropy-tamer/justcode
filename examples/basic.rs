//! Basic example matching bincode's example.

use justcode_core::{config, Decode, Encode};

#[derive(Encode, Decode, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct World(Vec<Entity>);

fn main() {
    let config = config::standard();

    let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);

    let encoded: Vec<u8> = justcode_core::encode_to_vec(&world, config).unwrap();

    // The length of the vector is encoded as a varint u64, which in this case is encoded as a single byte
    // The 4 floats are encoded in 4 bytes each.
    // Expected size: 1 byte (length varint) + 2 * (4 + 4) bytes (2 entities * 2 floats * 4 bytes)
    println!("Encoded length: {} bytes", encoded.len());
    println!("Expected: 1 (length) + 16 (2 entities * 2 floats * 4 bytes) = 17 bytes");

    let (decoded, len): (World, usize) = justcode_core::decode_from_slice(&encoded[..], config).unwrap();

    assert_eq!(world, decoded);
    assert_eq!(len, encoded.len()); // read all bytes

    println!("Successfully encoded and decoded!");
}


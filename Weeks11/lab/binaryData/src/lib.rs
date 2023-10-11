use core::panic;
fn matching(inp: u8) -> u8 {
    let res = match inp {
        0 => 0,
        1 => 1,
        2 => 3,
        3 => 2,
        4 => 6,
        5 => 7,
        6 => 5,
        7 => 4,
        8 => 0xC,
        9 => 0xD,
        0xA => 0xF,
        0xB => 0xE,
        0xC => 0xA,
        0xD => 0xB,
        0xE => 9,
        0xF => 8,
        _ => panic!("dfdfo"),
    };
    res
}

fn matching2(inp: u8) -> u8 {
    let res = match inp {
        0 => 0,
        1 => 1,
        3 => 2,
        2 => 3,
        6 => 4,
        7 => 5,
        5 => 6,
        4 => 7,
        0xC => 8,
        0xD => 9,
        0xF => 0xA,
        0xE => 0xB,
        0xA => 0xC,
        0xB => 0xD,
        9 => 0xE,
        0x8 => 0xF,
        _ => panic!("dfdfo"),
    };
    res
}

fn encode_hex(b: u8) -> u8 {
    let low = b & 0x0f;
    let high = b >> 4;

    let newlow = matching(low);
    let newhigh = matching(high) << 4;

    let result = newhigh | newlow;
    result

}

pub const FOX: &str = "The quick brown fox jumps over the lazy dog.";
pub const ENCODED_DATA: &[u8] = &[
    0x76, 0x5C, 0x57, 0x30, 0x41, 0x47, 0x5D, 0x52, 0x5E, 0x30, 0x53, 0x43, 0x58, 0x44, 0x59,
    0x30, 0x55, 0x58, 0x4C, 0x30, 0x5F, 0x47, 0x5B, 0x40, 0x42, 0x30, 0x58, 0x45, 0x57, 0x43,
    0x30, 0x46, 0x5C, 0x57, 0x30, 0x5A, 0x51, 0x4F, 0x4D, 0x30, 0x56, 0x58, 0x54, 0x39,
];

fn encode_hex_data(v: &[u8]) -> Vec<u8> {
    // let mut res = Vec::with_capacity(v.len());
    v.iter().cloned().map(|b| encode_hex(b)).collect()
}

#[test]
fn test_encode_hex() {
    assert_eq!(encode_hex(0x54), 0x76);
    assert_eq!(encode_hex(0x68), 0x5C);
    assert_eq!(
        (0..16).map(encode_hex).collect::<Vec<_>>(),
        [0x0, 0x1, 0x3, 0x2, 0x6, 0x7, 0x5, 0x4, 0xC, 0xD, 0xF, 0xE, 0xA, 0xB, 0x9, 0x8]
    );
    pub const FOX: &str = "The quick brown fox jumps over the lazy dog.";
    pub const ENCODED_DATA: &[u8] = &[
        0x76, 0x5C, 0x57, 0x30, 0x41, 0x47, 0x5D, 0x52, 0x5E, 0x30, 0x53, 0x43, 0x58, 0x44, 0x59,
        0x30, 0x55, 0x58, 0x4C, 0x30, 0x5F, 0x47, 0x5B, 0x40, 0x42, 0x30, 0x58, 0x45, 0x57, 0x43,
        0x30, 0x46, 0x5C, 0x57, 0x30, 0x5A, 0x51, 0x4F, 0x4D, 0x30, 0x56, 0x58, 0x54, 0x39,
    ];
    let original_data = FOX.as_bytes();
    let encoded_data = ENCODED_DATA;
    assert_eq!(encode_hex_data(original_data), encoded_data);
}

fn decode_hex(b: u8) -> u8 {
    let low = b & 0x0F;
    let high = b >> 4;

    let newlow = matching2(low);
    let newhigh = matching2(high) << 4;

    let result = newhigh | newlow;
    result
}

fn decode_hex_data(v: &[u8]) -> Vec<u8> {
    v.iter().cloned().map(|b| decode_hex(b)).collect()
}

#[test]
fn test_decode_hex() {
    assert_eq!(
        (0..16).map(decode_hex).collect::<Vec<_>>(),
        [0x0, 0x1, 0x3, 0x2, 0x7, 0x6, 0x4, 0x5, 0xF, 0xE, 0xC, 0xD, 0x8, 0x9, 0xB, 0xA]
    );
    assert_eq!(decode_hex(0x76), 0x54);
    assert_eq!(decode_hex(0x5C), 0x68);
    let original_data = FOX.as_bytes();
    let encoded_data = ENCODED_DATA;
    assert_eq!(decode_hex_data(encoded_data), original_data);
}

//-----------------------------------------------------------------------------------------------------------------------------------------
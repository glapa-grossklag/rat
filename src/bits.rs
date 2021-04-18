/// Get the bit-value of `byte` at `i`.
///
/// * The bit-value will aways be 0 or 1.
/// * The index `i` must be between 0 and 7 inclusive.
pub fn get(byte: u8, i: u8) -> u8 {
    if i > 7 {
        panic!("Index must be in the range 0 to 7, was {}", i);
    }

    if byte & (0b0000_0001 << i) != 0 {
        return 1;
    } else {
        return 0;
    }
}

/// Return `byte` with the `i`th bit as 1.
///
/// The index `i` must be between 0 and 7 inclusive.
pub fn set(byte: u8, i: u8) -> u8 {
    if i > 7 {
        panic!("Index must be in the range 0 to 7 inclusive, was {}", i);
    }

    return byte | (0b0000_0001 << i);
}

/// Return `byte` with the `i`th bit as 1.
///
/// The index `i` must be between 0 and 7 inclusive.
pub fn clear(byte: u8, i: u8) -> u8 {
    if i > 7 {
        panic!("Index must be in the range 0 to 7 inclusive, was {}", i);
    }

    return byte & (!0b0000_0001 << i);
}

/// Return `byte` with the `i`th bit flipped.
///
/// The index `i` must be between 0 and 7 inclusive.
/// If the bit-value was 0, then it will change to 1, and vice versa.
pub fn flip(byte: u8, index: u8) -> u8 {
    if index > 7 {
        panic!("Index must be in the range 0 to 7 inclusive, was {}", index);
    }

    if get(byte, index) == 0 {
        return set(byte, index);
    } else {
        return clear(byte, index);
    }
}

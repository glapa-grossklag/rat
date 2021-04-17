/// Get the bit-value of `byte` at `index`.
///
/// * The bit-value will aways be 0 or 1.
/// * The index must be between 0 and 7 inclusive.
pub fn get(byte: u8, index: u8) -> u8 {
    if index > 7 {
        panic!("Index must be in the range 0 to 7, was {}", index);
    }

    if byte & (0b0000_0001 << index) != 0 {
        1
    } else {
        0
    }
}

/// Set the bit-value of `byte` at `index`.
///
/// * Setting a bit implies changing it to 1.
/// * The index must be between 0 and 7 inclusive.
pub fn set(byte: &mut u8, index: u8) {
    if index > 7 {
        panic!("Index must be in the range 0 to 7, was {}", index);
    }

    *byte |= 0b0000_0001 << index;
}

/// Clear the bit-value of `byte` at `index`.
///
/// * Clearing a bit implies changing it to 0.
/// * The index must be between 0 and 7 inclusive.
pub fn clear(byte: &mut u8, index: u8) {
    if index > 7 {
        panic!("Index must be in the range 0 to 7, was {}", index);
    }

    *byte &= !0b0000_0001 << index;
}

/// Flip the bit-value of `byte` at `index`.
///
/// * If the bit-value was 0, then it will change to 1, and vice versa.
/// * The index must be between 0 and 7 inclusive.
pub fn flip(byte: &mut u8, index: u8) {
    if index > 7 {
        panic!("Index must be in the range 0 to 7, was {}", index);
    }

    if get(*byte, index) == 0 {
        set(byte, index);
    } else {
        clear(byte, index);
    }
}

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

pub fn set(byte: &mut u8, index: u8) {
    if index > 7 {
        panic!("Index must be in the range 0 to 7, was {}", index);
    }

    *byte |= 0b0000_0001 << index;
}

pub fn clear(byte: &mut u8, index: u8) {
    if index > 7 {
        panic!("Index must be in the range 0 to 7, was {}", index);
    }

    *byte &= !0b0000_0001 << index;
}

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

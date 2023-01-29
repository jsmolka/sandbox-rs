use bitfield_macro::bitfield;

#[test]
fn data_mask() {
    bitfield! {
        struct Bitfield: u8 {
            f1: u8 @ 0..2,
            f2: u8 @ 6..8,
        }
    }
    assert_eq!(Bitfield::data_mask(), 0x0C3);
}

#[test]
fn data() {
    bitfield! {
        struct Bitfield: u8 {
            f1: u8 @ 0..4,
        }
    }
    let mut bf = Bitfield::new(0xFF);
    assert_eq!(bf.data(), 0x0F);
    bf.set_data(0x00);
    assert_eq!(bf.data(), 0x00);
    bf.set_data(0xF0);
    assert_eq!(bf.data(), 0x00);
    bf.set_data(0x0F);
    assert_eq!(bf.data(), 0x0F);
}

#[test]
fn bytes() {
    bitfield! {
        struct Bitfield: u16 {
            f1: u8 @ 0..4,
            f2: u8 @ 12..16,
        }
    }
    let mut bf = Bitfield::new(0);
    bf.set_byte(0, 0xFF);
    assert_eq!(bf.byte(0), 0x0F);
    assert_eq!(bf.data(), 0x000F);
    bf.set_byte(1, 0xFF);
    assert_eq!(bf.byte(1), 0xF0);
    assert_eq!(bf.data(), 0xF00F);
}

#[test]
fn fields() {
    bitfield! {
        struct Bitfield: u8 {
            f1: u8 @ 0..4,
            f2: u8 @ 4..8,
            f3: u8 @ 0..8,
        }
    }
    let mut bf = Bitfield::new(0xFF);
    assert_eq!(bf.f1(), 0x0F);
    assert_eq!(bf.f2(), 0x0F);
    assert_eq!(bf.f3(), 0xFF);
    bf.set_f3(0xAB);
    assert_eq!(bf.f1(), 0x0B);
    assert_eq!(bf.f2(), 0x0A);
    bf.set_f1(0xD);
    bf.set_f2(0xC);
    assert_eq!(bf.f3(), 0xCD);
}

#[test]
fn types() {
    bitfield! {
        struct Bitfield: u8 {
            f1: bool @ 0..1,
            f2: bool @ 1..2,
            f3: bool @ 2..3,
            f4: bool @ 3..4,
            f5: u16 @ 4..8,
        }
    }
    let mut bf = Bitfield::new(0);
    bf.set_f1(true);
    assert_eq!(bf.data(), 0x01);
    bf.set_f2(true);
    assert_eq!(bf.data(), 0x03);
    bf.set_f3(true);
    assert_eq!(bf.data(), 0x07);
    bf.set_f4(true);
    assert_eq!(bf.data(), 0x0F);
    bf.set_f5(0xFFFF);
    assert_eq!(bf.data(), 0xFF);
}

#[test]
fn pipes() {
    bitfield! {
        struct Bitfield: u8 {
            f1: u8 @ 0..2 => |v| 2 * v,
            f2: u8 @ 2..4 => |v| [0xAA, 0xBB, 0xCC, 0xDD][v as usize],
            f3: u8 @ 4..6 => |v| -> u32 { v as u32 * 0x01010101 },
        }
    }
    let bf = Bitfield::new(0xFF);
    assert_eq!(bf.f1(), 0x06);
    assert_eq!(bf.f2(), 0xDD);
    assert_eq!(bf.f3(), 0x0303_0303);
}

#[test]
fn traits() {
    bitfield! {
        struct Bitfield: u8 {
            f1: u8 @ 0..4,
        }
    }
    let bf = Bitfield::new(0xFF);
    assert_eq!(u8::from(bf), 0x0F);
    let bf = Bitfield::from(0xFF);
    assert_eq!(bf.data(), 0x0F);
}

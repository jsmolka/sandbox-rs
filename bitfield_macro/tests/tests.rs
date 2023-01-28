use bitfield_macro::bitfield;

bitfield! {
    pub struct Test: u8 {
        pub f1: u8 @ 0..2 => |value| 2 * value,
    }
}

#[test]
fn test() {
    let test = Test::new(0xFF);
    assert_eq!(test.f1(), 0b11 * 2);
}

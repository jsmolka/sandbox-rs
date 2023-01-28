use bitfield_macro::bitfield;

bitfield! {
    pub struct Test: u8 {
        pub f1: bool @ 0..1,
        pub f2: u8 @ 4..6,
    }
}

#[test]
fn test() {
    let mut test = Test::default();
    test.set_byte(0, 0xFF);
    assert_eq!(test.byte(0), 0x33);
}

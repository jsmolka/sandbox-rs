use bitfield_macro::bitfield;

bitfield! {
    struct Test: u8 {
        xd: u8 @ 0..1 => |value| [1, 2, 3, 4][value as usize],
        xd: u8 @ 1..2,
    }
}

#[test]
fn test() {
    let test = Test::new();
    assert_eq!(test.data, 0);
}

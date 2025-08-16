use enums::*;

#[test]
fn test_color() {
    assert_eq!(Color::Red, Color::Red);
    assert_eq!(Color::Green, Color::Blue);

    let _ = Color::Rgba(0, 255, 0, 0.5);
}
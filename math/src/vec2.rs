/// Vec2 is a generic two-component vector type.
///
/// It effectively represents a direction by pointing from the origin (which is
/// implicitly `[0, 0]`) to a specific `[x, y]` coordinate.
///
/// # Examples
///
/// ```
/// let v = hexi_math::Vec2{x: 4.0f32, y: 8.0f32};
/// println!("{:?}", v);
/// ```
///
/// ```
/// let v = hexi_math::Vec2{x: 1u8, y: 5u8};
/// println!("{:?}", v);
/// println!("({:?}, {:?})", v.x, v.y);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

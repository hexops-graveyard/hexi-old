/// Point2 is a generic two-component point type.
///
/// It represents a specific `[x, y]` point in 2D space.
///
/// # Examples
///
/// ```
/// let p = hexi_math::Point2{x: 4.0f32, y: 8.0f32};
/// println!("{:?}", p);
/// ```
///
/// ```
/// let p = hexi_math::Point2{x: 1u8, y: 5u8};
/// println!("{:?}", p);
/// println!("({:?}, {:?})", p.x, p.y);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

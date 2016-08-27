/// Vec2 is a generic two-component vector type.
///
/// It effectively represents a direction by pointing from the origin (which is
/// implicitly `[0, 0]`) to a specific `[x, y]` coordinate.
///
/// # Examples
///
/// ```
/// let x = hexi_math::Vec2(4.0f32, 8.0f32);
/// println!("{:?}", x);
/// ```
///
/// ```
/// let x = hexi_math::Vec2(1u8, 5u8);
/// println!("{:?}", x);
/// println!("({:?}, {:?})", x.0, x.1);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Vec2<T>(pub T, pub T);

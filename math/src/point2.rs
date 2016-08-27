/// Point2 is a generic two-component point type.
///
/// It represents a specific `[x, y]` point in 2D space.
///
/// # Examples
///
/// ```
/// let x = hexi_math::Point2(4.0f32, 8.0f32);
/// println!("{:?}", x);
/// ```
///
/// ```
/// let x = hexi_math::Point2(1u8, 5u8);
/// println!("{:?}", x);
/// println!("({:?}, {:?})", x.0, x.1);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Point2<T>(pub T, pub T);

/// Vec4 is a generic four-component vector type.
///
/// It effectively points to a specific `[x, y, z]` coordinate in 3D space and
/// the fourth `[w]` component can represent something else (e.g. force or
/// acceleration).
///
/// # Examples
///
/// ```
/// let x = hexi_math::Vec4(4.0f32, 8.0f32, 12.0f32, 16.0f32);
/// println!("{:?}", x);
/// ```
///
/// ```
/// let x = hexi_math::Vec4(1u8, 5u8, 10u8, 15u8);
/// println!("{:?}", x);
/// println!("({:?}, {:?}, {:?}, {:?})", x.0, x.1, x.2, x.3);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Vec4<T>(pub T, pub T, pub T, pub T);

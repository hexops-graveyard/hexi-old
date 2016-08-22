/// Point3 is a generic three-component point type.
///
/// # Examples
///
/// ```
/// let x = hexi_math::Point3(4.0f32, 8.0f32, 12.0f32);
/// println!("{:?}", x);
/// ```
///
/// ```
/// let x = hexi_math::Point3(1u8, 5u8, 10u8);
/// println!("{:?}", x);
/// println!("({:?}, {:?}, {:?})", x.0, x.1, x.2);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Point3<T>(pub T, pub T, pub T);

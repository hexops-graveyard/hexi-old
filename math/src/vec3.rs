/// Vec3 is a generic three-component vector type.
///
/// It has uses in both 2D and 3D space.
///
/// - In 2D space, `[x, y]` can represent the direction of the vector and the
///   third `[z]` component can represent something else (e.g. force or
///   acceleration).
/// - In 3D space, `[x, y, z]` can represent the direction of the vector.
///
/// # Examples
///
/// ```
/// let x = hexi_math::Vec3(4.0f32, 8.0f32, 12.0f32);
/// println!("{:?}", x);
/// ```
///
/// ```
/// let x = hexi_math::Vec3(1u8, 5u8, 10u8);
/// println!("{:?}", x);
/// println!("({:?}, {:?}, {:?})", x.0, x.1, x.2);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Vec3<T>(pub T, pub T, pub T);

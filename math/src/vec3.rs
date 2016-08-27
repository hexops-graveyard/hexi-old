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
/// let v = hexi_math::Vec3{x: 4.0f32, y: 8.0f32, z: 12.0f32};
/// println!("{:?}", v);
/// ```
///
/// ```
/// let v = hexi_math::Vec3{x: 1u8, y: 5u8, z: 10u8};
/// println!("{:?}", v);
/// println!("({:?}, {:?}, {:?})", v.x, v.y, v.z);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// Point3 is a generic three-component point type.
///
/// It represents a specific `[x, y, z]` point in 3D space.
///
/// # Examples
///
/// ```
/// let p = hexi_math::Point3{x: 4.0f32, y: 8.0f32, z: 12.0f32};
/// println!("{:?}", p);
/// ```
///
/// ```
/// let p = hexi_math::Point3{x: 1u8, y: 5u8, z: 10u8};
/// println!("{:?}", p);
/// println!("({:?}, {:?}, {:?})", p.x, p.y, p.z);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

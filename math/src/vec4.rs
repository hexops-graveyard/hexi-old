/// Vec4 is a generic four-component vector type.
///
/// It effectively points to a specific `[x, y, z]` coordinate in 3D space and
/// the fourth `[w]` component can represent something else (e.g. force or
/// acceleration).
///
/// # Examples
///
/// ```
/// let v = hexi_math::Vec4{x: 4.0f32, y: 8.0f32, z: 12.0f32, w: 16.0f32};
/// println!("{:?}", v);
/// ```
///
/// ```
/// let v = hexi_math::Vec4{x: 1u8, y: 5u8, z: 10u8, w: 15u8};
/// println!("{:?}", v);
/// println!("({:?}, {:?}, {:?}, {:?})", v.x, v.y, v.z, v.w);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

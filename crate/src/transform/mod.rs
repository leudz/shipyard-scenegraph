
mod matrix;
mod vec3;
mod quat;
mod values;

pub use self::matrix::*;
pub use self::vec3::*;
pub use self::quat::*;
pub use self::values::*;

pub struct Translation(pub Vec3);
pub struct Rotation(pub Quat);
pub struct Scale(pub Vec3);
pub struct LocalTransform(pub Matrix4);
pub struct WorldTransform(pub Matrix4);
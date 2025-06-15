use crate::float3::Float3;


pub struct Transform {
    pub yaw: f32,
}

impl Transform {

    pub fn to_world_point(&self, p: &Float3) -> Float3 {
        let (i_hat, j_hat, k_hat) = self.get_basis_vectors();
        Transform::transform_vector(i_hat, j_hat, k_hat, p)
    }

    fn get_basis_vectors(&self) -> (Float3, Float3, Float3) {
        let i_hat = Float3::new(self.yaw.cos(), 0.0, self.yaw.sin());
        let j_hat = Float3::new(0.0, 1.0, 0.0);
        let k_hat = Float3::new(-self.yaw.sin(), 0.0, self.yaw.cos());
        (i_hat, j_hat, k_hat)
    }

    // Move each coordinate of the given vector along the corresponding basis vector
    fn transform_vector(i_hat : Float3, j_hat: Float3, k_hat: Float3, v: &Float3) -> Float3 {
        v.x * i_hat + v.y * j_hat + v.z * k_hat
    }
}
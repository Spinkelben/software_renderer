use crate::float3::Float3;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub yaw: f32,
    pub pitch: f32,
    pub position: Float3,
}

impl Transform {

    pub fn to_world_point(&self, p: &Float3) -> Float3 {
        let (i_hat, j_hat, k_hat) = self.get_basis_vectors();
        Transform::transform_vector(i_hat, j_hat, k_hat, p) + self.position
    }

    fn get_basis_vectors(&self) -> (Float3, Float3, Float3) {
        // Yaw
        let i_hat_yaw = Float3::new(self.yaw.cos(), 0.0, self.yaw.sin());
        let j_hat_yaw = Float3::new(0.0, 1.0, 0.0);
        let k_hat_yaw = Float3::new(-self.yaw.sin(), 0.0, self.yaw.cos());

        // Pitch
        let i_hat_pitch = Float3::new(1.0, 0.0, 0.0);
        let j_hat_pitch = Float3::new(0.0, self.pitch.cos(), -self.pitch.sin());
        let k_hat_pitch = Float3::new(0.0, self.pitch.sin(), self.pitch.cos());

        // Combine yaw and pitch to get the final basis vectors
        let i_hat = Self::transform_vector(i_hat_yaw, j_hat_yaw, k_hat_yaw, &i_hat_pitch);
        let j_hat = Self::transform_vector(i_hat_yaw, j_hat_yaw, k_hat_yaw, &j_hat_pitch);
        let k_hat = Self::transform_vector(i_hat_yaw, j_hat_yaw, k_hat_yaw, &k_hat_pitch);

        (i_hat, j_hat, k_hat)
    }

    // Move each coordinate of the given vector along the corresponding basis vector
    fn transform_vector(i_hat : Float3, j_hat: Float3, k_hat: Float3, v: &Float3) -> Float3 {
        v.x * i_hat + v.y * j_hat + v.z * k_hat
    }
}
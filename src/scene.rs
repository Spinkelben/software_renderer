use crate::{render::Model, transform::Transform};

pub struct Scene {
    pub entities: Vec<Entity>,
}

pub struct Entity {
    pub model: Model,
    pub transform: Transform,
}
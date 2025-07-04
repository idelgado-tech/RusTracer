use uuid::Uuid;

use crate::{matrix::Matrix, reflection::Material, tuple::Tuple};

pub struct Object {
    pub origin: Tuple,
    pub radius: f64,
    pub transform: Matrix,
    pub material: Material,
    pub id: Uuid,
}
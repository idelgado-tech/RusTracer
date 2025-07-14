use uuid::Uuid;

use crate::{
    color::Color,
    matrix::Matrix,
    pattern::Pattern,
    ray::{Intersection, Ray},
    reflection::Material,
    shape::shape::Shape,
    tuple::Tuple,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub transform: Matrix,
    pub material: Material,
    pub shape: Shape,
    pub id: Uuid,
}

impl Object {
    pub fn intersect(&mut self, ray: Ray) -> Vec<Intersection> {
        self.shape.local_intersect(self.clone(), ray)
    }

    pub fn normal_at(&self, point: Tuple) -> Tuple {
        self.shape.local_normal_at(self.clone(), point)
    }

    pub fn set_transparency(&mut self, transparency: f64) {
        self.set_material(self.get_material().set_transparency(transparency));
    }

    pub fn set_refractive_index(&mut self, refractive_index: f64) {
        self.set_material(self.get_material().set_refractive_index(refractive_index));
    }

    pub fn set_ambiant(&mut self, ambiant: f64) {
        self.set_material(self.get_material().set_ambient(ambiant));
    }

    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.set_material(self.get_material().set_pattern(pattern));
    }

    pub fn set_reflective(&mut self, reflection: f64) {
        self.set_material(self.get_material().set_reflective(reflection));
    }

    pub fn set_color(&mut self, color: Color) {
        self.set_material(self.get_material().set_color(color));
    }

    pub fn set_transform(&mut self, new_stransform: &Matrix) {
        self.transform = new_stransform.clone();
    }

    pub fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }

    pub fn set_material(&mut self, new_material: &Material) {
        self.material = new_material.clone();
    }

    pub fn get_material(&self) -> Material {
        self.material.clone()
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn with_transformation(mut self, transformation: Matrix) -> Self {
        self.transform = transformation;
        self
    }

    pub fn with_shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }
}

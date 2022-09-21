use std::{error::Error, fmt};

use crate::{
    color::Color,
    ray::{hit_intersections, intersect, Intersection, Ray, Sphere},
    reflection::{self, Material, PointLight},
    transformation,
    tuple::Tuple,
};

#[derive(Debug, Clone, PartialEq)]
pub struct World {
    pub light_sources: Vec<PointLight>,
    pub objects: Vec<Sphere>,
}

impl World {
    pub fn world() -> World {
        World {
            light_sources: vec![],
            objects: vec![],
        }
    }

    pub fn default_world() -> World {
        let light = PointLight::new_point_light(
            Color::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(-10.0, 10.0, -10.0),
        );

        let mut s1 = Sphere::sphere();
        s1.material = Material::material();
        s1.material.color = Color::new_color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::sphere();
        s2.transform = transformation::create_scaling(0.5, 0.5, 0.5);

        World {
            light_sources: vec![light],
            objects: vec![s1, s2],
        }
    }

    pub fn intersect_world(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = vec![];
        for object in &self.objects {
            intersections.append(&mut intersect(&object, ray.clone()));
        }
        intersections.retain(|value| value.t > 0.0);
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }
}

#[cfg(test)]
mod matrix_tests {
    use super::*;

    #[test]
    ///Creating a world
    fn creation_world_test() {
        let w = World::world();
        assert_eq!(w.objects, vec![]);
        assert_eq!(w.light_sources, vec![]);
    }

    #[test]
    ///The default world
    fn default_world_test() {
        let w = World::default_world();

        let light = PointLight::new_point_light(
            Color::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(-10.0, 10.0, -10.0),
        );

        let mut s1 = Sphere::sphere();
        s1.material = Material::material();
        s1.material.color = Color::new_color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Sphere::sphere();
        s2.transform = transformation::create_scaling(0.5, 0.5, 0.5);

        assert_eq!(w.objects.len(), 2);
        assert_eq!(w.light_sources, vec![light]);
    }

    #[test]
    ///Intersect a world with a ray
    fn intersect_world_test() {
        let w = World::default_world();
        let ray = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = w.intersect_world(&ray);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }
}

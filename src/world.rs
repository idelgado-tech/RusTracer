use std::{error::Error, fmt, iter};

use crate::{
    color::{self, Color},
    ray::{hit_intersections, intersect, Intersection, Ray, Sphere},
    reflection::{self, lighting, Material, PointLight},
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

    pub fn shade_hit(&self, comps: &Computation) -> Color {
        let mut shade = Color::new_color(0.0, 0.0, 0.0);

        for light in &self.light_sources {
            shade += lighting(
                &comps.object.material,
                light,
                &comps.point,
                &comps.eyev,
                &comps.normalv,
            );
        }
        shade
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersect_world(ray);

        if intersections.is_empty() {
            return color::BLACK;
        }
        let comps = prepare_computations(&intersections[0], ray);
        self.shade_hit(&comps)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Computation {
    pub t: f64,
    pub object: Sphere,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
}

impl Computation {
    fn new() -> Computation {
        Computation {
            t: 0.0,
            object: Sphere::sphere(),
            point: Tuple::new_point(0.0, 0.0, 0.0),
            eyev: Tuple::new_vector(0.0, 0.0, 0.0),
            normalv: Tuple::new_vector(0.0, 0.0, 0.0),
            inside: true,
        }
    }
}

pub fn prepare_computations(intersection: &Intersection, ray: &Ray) -> Computation {
    let mut comps = Computation::new();

    comps.t = intersection.t;
    comps.object = intersection.object.clone();
    comps.point = ray.position(comps.t);
    comps.eyev = ray.direction.clone() * -1.0;
    comps.normalv = comps.object.normal_at_point(&comps.point);

    if Tuple::dot_product(&comps.normalv, &comps.eyev) < 0.0 {
        comps.inside = true;
        comps.normalv = comps.normalv * -1.0;
    } else {
        comps.inside = false;
    }

    comps
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

    #[test]
    ///Precomputing the state of an intersection
    fn precomputing_test() {
        let ray = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let i = Intersection {
            object: Sphere::sphere(),
            t: 4.0,
        };
        let comps = prepare_computations(&i, &ray);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Tuple::new_point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::new_vector(0.0, 0.0, -1.0));
    }

    #[test]
    ///The hit, when an intersection occurs on the outside
    fn hit_inside_test() {
        let ray = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let i = Intersection {
            object: Sphere::sphere(),
            t: 4.0,
        };
        let comps = prepare_computations(&i, &ray);

        assert_eq!(comps.inside, false);
    }

    #[test]
    ///The hit, when an intersection occurs on the outside
    fn hit_inside_test2() {
        let ray = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let i = Intersection {
            object: Sphere::sphere(),
            t: 4.0,
        };
        let comps = prepare_computations(&i, &ray);

        assert_eq!(comps.eyev, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
    }

    #[test]
    ///Shading an intersection
    fn shading() {
        let w = World::default_world();

        let ray = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = w.objects.first().unwrap().clone();
        let i = Intersection {
            object: shape,
            t: 4.0,
        };
        let comps = prepare_computations(&i, &ray);
        let c = w.shade_hit(&comps);

        assert_eq!(
            c,
            Color::new_color(
                0.38066119308103435,
                0.47582649135129296,
                0.28549589481077575
            )
        );
    }

    #[test]
    ///Shading an intersection from the inside
    fn shading_inside() {
        let mut w = World::default_world();
        w.light_sources = vec![PointLight::new_point_light(
            Color::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 0.25, 0.0),
        )];

        let ray = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = w.objects[1].clone();
        let i = Intersection {
            object: shape,
            t: 0.5,
        };
        let comps = prepare_computations(&i, &ray);
        let c = w.shade_hit(&comps);

        assert_eq!(
            c,
            Color::new_color(0.9049844720832575, 0.9049844720832575, 0.9049844720832575)
        );
    }
}

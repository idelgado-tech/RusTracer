use crate::{
    color::{self, Color},
    ray::{Intersection, Ray, hit_intersections, reflect},
    reflection::{Material, PointLight, lighting},
    shape::{object::Object, shape::Shape},
    transformation,
    tuple::Tuple,
};

pub const SHADOW_EPSILON: f64 = 0.00000000001;

#[derive(Debug, Clone, PartialEq)]
pub struct World {
    pub light_sources: Vec<PointLight>,
    pub objects: Vec<Object>,
}

impl World {
    pub fn new_world() -> World {
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

        let mut s1 = Object::new_sphere();
        s1.material = Material::default_material();
        s1.material.color = Color::new_color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Object::new_sphere();
        s2.transform = transformation::create_scaling(0.5, 0.5, 0.5);

        World {
            light_sources: vec![light],
            objects: vec![s1, s2],
        }
    }

    pub fn intersect_world(&self, ray: &Ray) -> Vec<Intersection> {
        let mut intersections = vec![];
        for object in &self.objects {
            intersections.append(&mut object.clone().intersect(ray.clone()));
        }
        intersections.retain(|value| value.t > 0.0);
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }

    pub fn is_shadowed_for_light(&self, point: &Tuple, light_source: &PointLight) -> bool {
        let v = light_source.position.clone() - point.clone();
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point.clone(), direction);
        let intersections = self.intersect_world(&r);

        if let Some(h) = hit_intersections(intersections) {
            return h.object.has_shadow() && h.t < distance;
        }
        false
    }

    pub fn shade_hit(&self, comps: &Computation, remaining_calculations: usize) -> Color {
        let mut shade = Color::new_color(0.0, 0.0, 0.0);

        for light in &self.light_sources {
            let is_shadow = self.is_shadowed_for_light(&comps.over_point, light);
            let surface = lighting(
                &comps.object.get_material(),
                light,
                &comps.over_point,
                &comps.eyev,
                &comps.normalv,
                is_shadow,
                comps.object.clone(),
            );

            let reflected = self.reflected_color(comps.clone(), remaining_calculations);

            let refracted = self.refracted_color(comps.clone(), remaining_calculations);
            let material = comps.object.get_material();
            if material.reflective > 0.0 && material.transparency > 0.0 {
                let reflectance = comps.schlick();
                shade += surface + reflected * reflectance + refracted * (1.0 - reflectance);
            } else {
                shade += surface + reflected + refracted;
            }
        }

        shade
    }

    pub fn color_at(&self, ray: &Ray, remaining_calculations: usize) -> Color {
        let intersections = self.intersect_world(ray);

        if intersections.is_empty() {
            return color::BLACK;
        }

        let comps = prepare_computations_v2(&intersections[0], ray, intersections.clone());
        self.shade_hit(&comps, remaining_calculations)
    }

    pub fn reflected_color(&self, comps: Computation, remaining_calculations: usize) -> Color {
        if comps.object.get_material().reflective == 0.0 || remaining_calculations == 0 {
            return color::BLACK;
        }
        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let ref_color = self.color_at(&reflect_ray, remaining_calculations - 1);

        ref_color * comps.object.get_material().reflective
    }

    pub fn add_object(&mut self, obj: Object) {
        self.objects.push(obj);
    }
}

#[derive(Debug, Clone)]
pub struct Computation {
    pub t: f64,
    pub object: Object,
    pub point: Tuple,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub reflectv: Tuple,
    pub n1: f64,
    pub n2: f64,
}

impl PartialEq for Computation {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
            && self.object == other.object.clone()
            && self.point == other.point
            && self.over_point == other.over_point
            && self.under_point == other.under_point
            && self.eyev == other.eyev
            && self.normalv == other.normalv
            && self.inside == other.inside
            && self.reflectv == other.reflectv
            && self.n1 == other.n1
            && self.n2 == other.n2
    }
}

impl Computation {
    fn new() -> Computation {
        Computation {
            t: 0.0,
            object: Object::new_sphere(),
            point: Tuple::new_point(0.0, 0.0, 0.0),
            over_point: Tuple::new_point(0.0, 0.0, 0.0),
            under_point: Tuple::new_point(0.0, 0.0, 0.0),
            eyev: Tuple::new_vector(0.0, 0.0, 0.0),
            normalv: Tuple::new_vector(0.0, 0.0, 0.0),
            inside: true,
            reflectv: Tuple::new_vector(0.0, 0.0, 0.0),
            n1: 0.0,
            n2: 0.0,
        }
    }
}

pub fn prepare_computations_helper(intersection: &Intersection, ray: &Ray) -> Computation {
    let mut comps = Computation::new();

    comps.t = intersection.t;
    comps.object = intersection.object.clone();
    comps.point = ray.position(comps.t);
    comps.eyev = ray.direction.clone() * -1.0;
    comps.normalv = comps.object.normal_at(comps.point.clone());
    comps.reflectv = reflect(&ray.direction.clone(), &comps.normalv);

    if Tuple::dot_product(&comps.normalv, &comps.eyev) < 0.0 {
        comps.inside = true;
        comps.normalv = comps.normalv * -1.0;
    } else {
        comps.inside = false;
    }

    comps.over_point = comps.point.clone() + comps.normalv.clone() * SHADOW_EPSILON;
    comps.under_point = comps.point.clone() - comps.normalv.clone() * SHADOW_EPSILON;

    comps
}

pub fn prepare_computations_v2(
    intersection: &Intersection,
    ray: &Ray,
    intersection_list: Vec<Intersection>,
) -> Computation {
    let mut comps = prepare_computations_helper(intersection, ray);
    let mut container: Vec<Object> = Vec::new();

    for i in intersection_list {
        let is_hit = i == *intersection;
        if is_hit {
            if container.is_empty() {
                comps.n1 = 1.0;
            } else {
                comps.n1 = container.last().unwrap().get_material().refractive_index
            }
        }
        let find_item = container.iter().position(|n| *n == i.object);

        match find_item {
            Some(x) => {
                container.remove(x);
            }
            None => container.push(i.object),
        }

        if is_hit {
            if container.is_empty() {
                comps.n2 = 1.0;
            } else {
                comps.n2 = container.last().unwrap().get_material().refractive_index
            }

            break;
        }
    }
    comps
}

#[cfg(test)]
mod matrix_tests {
    use crate::{reflection, transformation::create_translation};

    use super::*;

    #[test]
    ///Creating a world
    fn creation_world_test() {
        let w = World::new_world();
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

        let mut s1 = Object::new_sphere();
        s1.material = Material::default_material();
        s1.material.color = Color::new_color(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let mut s2 = Object::new_sphere();
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
            object: Object::new_sphere(),
            t: 4.0,
        };
        let comps = prepare_computations_helper(&i, &ray);

        assert_eq!(comps.t, i.t);
        assert!((comps.object == i.object));
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
            object: Object::new_sphere(),
            t: 4.0,
        };
        let comps = prepare_computations_helper(&i, &ray);

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
            object: Object::new_sphere(),
            t: 4.0,
        };
        let comps = prepare_computations_helper(&i, &ray);

        assert_eq!(comps.eyev, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
    }

    #[test]
    ///Shading an intersection
    fn shading_test() {
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
        let comps = prepare_computations_helper(&i, &ray);
        let c = w.shade_hit(&comps, reflection::MAX_RECURTION);

        assert_eq!(
            c,
            Color::new_color(0.3806611930807966, 0.47582649135099575, 0.28549589481059745)
        );
    }

    #[test]
    ///Shading an intersection from the inside
    fn shading_inside_test() {
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
        let comps = prepare_computations_helper(&i, &ray);
        let c = w.shade_hit(&comps, reflection::MAX_RECURTION);

        assert_eq!(
            c,
            Color::new_color(0.9049844720800376, 0.9049844720800376, 0.9049844720800376)
        );
    }

    #[test]
    /// The color when a ray misses
    fn color_miss_test() {
        let w = World::default_world();
        let ray = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let color_at = w.color_at(&ray, reflection::MAX_RECURTION);

        assert_eq!(color_at, Color::new_color(0.0, 0.0, 0.0));
    }

    #[test]
    /// The color when a ray misses
    fn color_test() {
        let w = World::default_world();
        let ray = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let color_at = w.color_at(&ray, reflection::MAX_RECURTION);

        assert_eq!(
            color_at,
            Color::new_color(0.3806611930807966, 0.47582649135099575, 0.28549589481059745)
        );
    }

    #[test]
    /// The color with an intersection behind the ray
    fn color_intersection_test() {
        let mut w = World::default_world();
        let mut wo0 = w.objects[0].get_material();
        wo0.ambient = 1.0;
        w.objects[0].set_material(&wo0);

        let mut wo1 = w.objects[1].get_material();
        wo1.ambient = 1.0;
        w.objects[1].set_material(&wo1);

        let ray = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.75),
            Tuple::new_vector(0.0, 0.0, -1.0),
        );
        let color_at = w.color_at(&ray, reflection::MAX_RECURTION);

        assert_eq!(color_at, w.objects[1].get_material().color);
    }

    #[test]
    /// There is no shadow when nothing is collinear with point and light
    fn shadow_1_test() {
        let w = World::default_world();
        let point = Tuple::new_point(0.0, 10.0, 0.0);

        assert_eq!(w.is_shadowed_for_light(&point, &w.light_sources[0]), false);
    }

    #[test]
    ///  The shadow when an object is between the point and the light
    fn shadow_2_test() {
        let w = World::default_world();
        let point = Tuple::new_point(10.0, -10.0, 10.0);

        assert_eq!(w.is_shadowed_for_light(&point, &w.light_sources[0]), true);
    }

    #[test]
    ///There is no shadow when an object is behind the light
    fn shadow_3_test() {
        let w = World::default_world();
        let point = Tuple::new_point(-20.0, 20.0, -20.0);

        assert_eq!(w.is_shadowed_for_light(&point, &w.light_sources[0]), false);
    }

    #[test]
    ///There is no shadow when an object is behind the light
    fn shadow_4_test() {
        let w = World::default_world();
        let point = Tuple::new_point(-2.0, 2.0, -2.0);

        assert_eq!(w.is_shadowed_for_light(&point, &w.light_sources[0]), false);
    }

    #[test]
    ///shade_hit() is given an intersection in shadow
    fn shade_hits_shadow_test() {
        let mut w = World::new_world();
        w.light_sources = vec![PointLight::new_point_light(
            Color::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 0.0, -10.0),
        )];

        let s1 = Object::new_sphere();
        w.objects.push(s1);
        let mut s2 = Object::new_sphere();
        s2.set_transform(&create_translation(0.0, 0.0, 10.0));
        w.objects.push(s2.clone());

        let ray = Ray::new(
            Tuple::new_point(0.0, 0.0, 5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        let i = Intersection {
            object: s2.clone(),
            t: 4.0,
        };
        let comps = prepare_computations_helper(&i, &ray);
        let c = w.shade_hit(&comps, reflection::MAX_RECURTION);

        assert_eq!(c, Color::new_color(0.1, 0.1, 0.1));
    }

    #[test]
    ///The hit should offset the point
    fn precomputing_epsilon_test() {
        let ray = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s1 = Object::new_sphere();
        s1.set_transform(&create_translation(0.0, 0.0, 1.0));
        let i = Intersection { object: s1, t: 5.0 };
        let comps = prepare_computations_helper(&i, &ray);

        assert_eq!(comps.over_point.z, -SHADOW_EPSILON);
        assert!(comps.point.z > comps.over_point.z);
        assert_eq!(comps.normalv, Tuple::new_vector(0.0, 0.0, -1.0));
    }
}

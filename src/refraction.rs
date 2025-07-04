use crate::{
    color::{self, Color},
    ray::Ray,
    tuple::Tuple,
    world::{Computation, World},
};

pub const VACUUM_REFRACTION: f64 = 1.0;
pub const AIR_REFRACTION: f64 = 1.00029;
pub const WATER_REFRACTION: f64 = 1.333;
pub const GLASS_REFRACTION: f64 = 1.52;
pub const DIAMOND_REFRACTION: f64 = 2.417;

impl World {
    pub fn refracted_color(&self, comps: Computation, remaining_iterations: usize) -> Color {
        if comps.object.get_material().transparency == 0.0 || remaining_iterations == 0 {
            return color::BLACK;
        }

        let n_ratio = comps.n1 / comps.n2;
        let cos_i = Tuple::dot_product(&comps.eyev, &comps.normalv);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

        if sin2_t > 1.0 {
            return color::BLACK;
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
        let refract_ray = Ray::new(comps.under_point, direction);

        self.color_at(&refract_ray, remaining_iterations - 1)
            * comps.object.get_material().transparency
    }
}

impl Computation {
    pub fn schlick(&self) -> f64 {
        let mut cos = Tuple::dot_product(&self.eyev, &self.normalv);

        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sint_t = n.powi(2) * (1.0 - cos.powi(2));
            if sint_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sint_t).powi(2);
            cos = cos_t;
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        return r0 + (1.0 - r0) * (1.0 - cos).powi(5);
    }
}

#[cfg(test)]
mod matrix_tests {

    use crate::{
        color::{self, Color},
        pattern::Pattern,
        ray::{Intersection, Ray},
        shape::{plane::Plane, shape::Shape, sphere::Sphere},
        transformation,
        tuple::Tuple,
        utils,
        world::{World, prepare_computations_v2},
    };

    #[test]
    // Scenario Outline: Finding n1 and n2 at various intersections
    fn refraction_at_intersection() {
        let mut a = Sphere::new_glass_sphere();
        a.material.refractive_index = 1.5;
        a.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));

        let mut b = Sphere::new_glass_sphere();
        b.material.refractive_index = 2.0;
        b.set_transform(&transformation::create_translation(0.0, 0.0, -0.25));

        let mut c = Sphere::new_glass_sphere();
        c.material.refractive_index = 2.5;
        c.set_transform(&transformation::create_translation(0.0, 0.0, 0.25));

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -4.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        let xs = vec![
            Intersection::new(2.0, a.box_owned()),
            Intersection::new(2.75, b.box_owned()),
            Intersection::new(3.25, c.box_owned()),
            Intersection::new(4.75, b.box_owned()),
            Intersection::new(5.25, c.box_owned()),
            Intersection::new(6.0, a.box_owned()),
        ];

        let valeurs = vec![
            [1.0, 1.5],
            [1.5, 2.0],
            [2.0, 2.5],
            [2.5, 2.5],
            [2.5, 1.5],
            [1.5, 1.0],
        ];

        for (i, elem) in xs.iter().enumerate() {
            let comp = prepare_computations_v2(elem, &r, xs.clone());

            assert_eq!(comp.n1, valeurs[i][0]);
            assert_eq!(comp.n2, valeurs[i][1]);
        }
    }

    #[test]
    //Scenario: The under point is offset below the surface
    fn refraction_underpoint_test() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        let mut shape = Sphere::new_glass_sphere();
        shape.set_transform(&transformation::create_translation(0.0, 0.0, 1.0));

        let i = Intersection::new(5.0, shape.box_owned());
        let comps = prepare_computations_v2(&i, &r, vec![i.clone()]);
        assert!(comps.under_point.z > f64::EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    #[test]
    fn refrected_color_1_test() {
        //Scenario: The under point is offset below the surface

        let w = World::default_world();
        let shape = w.objects[0].as_ref();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        let xs = vec![
            Intersection::new(4.00, shape.box_owned()),
            Intersection::new(6.00, shape.box_owned()),
        ];
        let comps = prepare_computations_v2(&xs[0], &r, xs.clone());

        let c = w.refracted_color(comps, 5);
        assert_eq!(c, color::BLACK);
    }

    #[test]
    // Scenario: The refracted color at the maximum recursive depth
    fn refrected_color_2_test() {
        let mut w = World::default_world();
        let shape = w.objects[0].as_mut();
        shape.set_transparency(1.0);
        shape.set_refractive_index(1.5);

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = vec![
            Intersection::new(4.00, shape.box_owned()),
            Intersection::new(6.00, shape.box_owned()),
        ];
        let comps = prepare_computations_v2(&xs[0], &r, xs.clone());

        let c = w.refracted_color(comps, 0);
        assert_eq!(c, color::BLACK);
    }

    #[test]
    // Scenario: The refracted color under total internal reflection
    fn refrected_color_3_test() {
        let mut w = World::default_world();
        let shape = w.objects[0].as_mut();
        shape.set_transparency(1.0);
        shape.set_refractive_index(1.5);

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let xs = vec![
            Intersection::new(-2.0_f64.sqrt() / 2.0, shape.box_owned()),
            Intersection::new(2.0_f64.sqrt() / 2.0, shape.box_owned()),
        ];
        let comps = prepare_computations_v2(&xs[1], &r, xs.clone());

        let c = w.refracted_color(comps, 5);
        assert_eq!(c, color::BLACK);
    }

    #[test]
    // Scenario: The refracted color with a refracted ray
    fn refrected_color_4_test() {
        let mut w = World::default_world();

        w.objects[0].set_ambiant(1.0);
        w.objects[0].set_pattern(Pattern::new_test_pattern());

        w.objects[1].set_transparency(1.0);
        w.objects[1].set_refractive_index(1.5);

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.1),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );

        let xs = vec![
            Intersection::new(-0.9899, w.objects[0].box_owned()),
            Intersection::new(-0.4899, w.objects[1].box_owned()),
            Intersection::new(0.4899, w.objects[1].box_owned()),
            Intersection::new(0.9899, w.objects[0].box_owned()),
        ];

        let comps = prepare_computations_v2(&xs[2], &r, xs.clone());

        let c = w.refracted_color(comps, 5);
        assert_eq!(
            c.normalise(),
            Color::new_color(0.0, 0.99888, 0.04725).normalise()
        );
    }

    #[test]
    // Scenario: The refracted color with a refracted ray
    fn refrected_shade_hit() {
        let mut w = World::default_world();

        let mut floor = Plane::plane();
        floor.set_transform(&transformation::create_translation(0.0, -1.0, 0.0));
        floor.set_transparency(0.5);
        floor.set_refractive_index(1.5);

        w.add_object(floor.box_owned());

        let mut ball = Sphere::sphere();
        ball.set_color(Color::new_color(1.0, 0.0, 0.0));
        ball.set_ambiant(0.5);
        ball.set_transform(&transformation::create_translation(0.0, -3.5, -0.5));

        w.add_object(ball.box_owned());

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -3.0),
            Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let xs = vec![Intersection::new(2.0_f64.sqrt(), floor.box_owned())];

        let comps = prepare_computations_v2(&xs[0], &r, xs.clone());

        let c = w.shade_hit(&comps, 5);
        assert_eq!(c, Color::new_color(0.93642, 0.68642, 0.68642));
    }

    #[test]
    // Scenario: The Schlick approximation under total internal reflection
    fn schlick_test_1() {
        let shape = Sphere::new_glass_sphere();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );

        let xs = vec![
            Intersection::new(-2.0_f64.sqrt() / 2.0, shape.box_owned()),
            Intersection::new(2.0_f64.sqrt() / 2.0, shape.box_owned()),
        ];

        let comps = prepare_computations_v2(&xs[1], &r, xs.clone());
        let refelctance = comps.schlick();
        assert_eq!(refelctance, 1.0);
    }

    #[test]
    // Scenario: The Schlick approximation with a perpendicular viewing angle
    fn schlick_test_2() {
        let shape = Sphere::new_glass_sphere();

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );

        let xs = vec![
            Intersection::new(-1.0, shape.box_owned()),
            Intersection::new(1.0, shape.box_owned()),
        ];

        let comps = prepare_computations_v2(&xs[1], &r, xs.clone());
        let refelctance = comps.schlick();
        assert!(utils::compare_float(refelctance, 0.04));
    }

    #[test]
    // Scenario: The Schlick approximation with small angle and n2 > n1
    fn schlick_test_3() {
        let shape = Sphere::new_glass_sphere();

        let r = Ray::new(
            Tuple::new_point(0.0, 0.99, -2.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        let xs = vec![Intersection::new(1.8589, shape.box_owned())];

        let comps = prepare_computations_v2(&xs[0], &r, xs.clone());
        let refelctance = comps.schlick();
        assert!(utils::compare_float(refelctance, 0.48873));
    }

    #[test]
    // Scenario: shade_hit() with a reflective, transparent material
    fn refrected_shade_hit_schlick() {
        let mut w = World::default_world();

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -3.0),
            Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let mut floor = Plane::plane();
        floor.set_transform(&transformation::create_translation(0.0, -1.0, 0.0));
        floor.set_reflective(0.5);
        floor.set_refractive_index(1.5);
        floor.set_transparency(0.5);
        w.add_object(floor.box_owned());

        let mut ball = Sphere::sphere();
        ball.set_color(Color::new_color(1.0, 0.0, 0.0));
        ball.set_ambiant(0.5);
        ball.set_transform(&transformation::create_translation(0.0, -3.5, -0.5));
        w.add_object(ball.box_owned());

        let xs = vec![Intersection::new(2.0_f64.sqrt(), floor.box_owned())];

        let comps = prepare_computations_v2(&xs[0], &r, xs.clone());

        let c = w.shade_hit(&comps, 5);
        assert_eq!(c, Color::new_color(0.93642, 0.68642, 0.68642));
    }
}

// Scenario: The Schlick approximation with small angle and n2 > n1
// Given shape ← glass_sphere()
// And r ← ray(point(0, 0.99, -2), vector(0, 0, 1))
// And xs ← intersections(1.8589:shape)
// When comps ← prepare_computations(xs[0], r, xs)
// And reflectance ← schlick(comps)
// Then reflectance = 0.48873

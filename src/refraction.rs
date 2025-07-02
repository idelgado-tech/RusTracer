pub const VACUUM_REFRACTION: f64 = 1.0;
pub const AIR_REFRACTION: f64 = 1.00029;
pub const WATER_REFRACTION: f64 = 1.333;
pub const GLASS_REFRACTION: f64 = 1.52;
pub const DIAMOND_REFRACTION: f64 = 2.417;

#[cfg(test)]
mod matrix_tests {

    use crate::{
        ray::{Intersection, Ray},
        shape::{shape::Shape, sphere::Sphere},
        transformation,
        tuple::Tuple,
        world::{prepare_computations, prepare_computations_v2},
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

        let valeurs = vec![[1.0, 1.5], [1.5, 2.0], [2.0, 2.5], [2.5, 2.5], [2.5, 1.5], [1.5, 1.0]];


        for (i, elem) in xs.iter().enumerate() {
        (println!());

            let comp = prepare_computations_v2(elem, &r, xs.clone());

            assert_eq!(comp.n1 , valeurs[i][0]) ;
            assert_eq!(comp.n2 , valeurs[i][1]) ;

        }
    }
}


use core::error;
use std::rc::Rc;

use crate::{
    color::{self, Color},
    tuple::Tuple,
};

#[derive(Clone)]
pub struct Pattern {
    pub color_a: Color,
    pub color_b: Color,
    pub fonction: Rc<dyn for<'a> Fn(&Pattern, Tuple) -> Color>,
}

impl std::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Pattern")
            .field("color_a", &self.color_a)
            .field("color_b", &self.color_b)
            .finish_non_exhaustive()
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.color_a == other.color_a
            && self.color_b == other.color_b
            && Rc::ptr_eq(&self.fonction, &other.fonction)
    }
}

impl Pattern {
    pub fn new(
        color_a: Color,
        color_b: Color,
        fonction: impl Fn(&Pattern, Tuple) -> Color + 'static,
    ) -> Pattern {
        Pattern {
            color_a,
            color_b,
            fonction: Rc::new(fonction),
        }
    }

    pub fn new_stripe_pattern(color_a: Color, color_b: Color) -> Pattern {
        let fonction = |pattern: &Pattern, point: Tuple| {
            if point.x.floor() % 2.0 == 0.0 {
                pattern.get_color_a()
            } else {
                pattern.get_color_b()
            }
        };

        Pattern::new(color_a, color_b, fonction)
    }

    pub fn get_color_a(&self) -> Color {
        self.color_a
    }

    pub fn set_color_a(&mut self, new_color: Color) {
        self.color_a = new_color;
    }

    pub fn get_color_b(&self) -> Color {
        self.color_b
    }

    pub fn color_at_point(&self, point: Tuple) -> Color {
        (self.fonction)(&self, point)
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::reflection::{self, Material, PointLight};

    use super::*;

    #[test]
    ///Creating a stripe pattern
    fn creation_pattern_test() {
        let pattern = Pattern::new_stripe_pattern(color::BLACK, color::WHITE);

        assert_eq!(pattern.get_color_a(), color::BLACK);
        assert_eq!(pattern.get_color_b(), color::WHITE);
    }

    #[test]
    ///A stripe pattern is constant in y
    fn pattern_y_test() {
        let pattern = Pattern::new_stripe_pattern(color::WHITE, color::BLACK);

        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 1.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 1.0, 0.0)),
            color::WHITE
        );
    }

    #[test]
    ///A stripe pattern is constant in z
    fn pattern_z_test() {
        let pattern = Pattern::new_stripe_pattern(color::WHITE, color::BLACK);

        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 0.0, 1.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 0.0, 2.0)),
            color::WHITE
        );
    }

    #[test]
    ///A stripe pattern alternates in x
    fn pattern_x_test() {
        let pattern = Pattern::new_stripe_pattern(color::WHITE, color::BLACK);

        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.9, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(1.0, 0.0, 0.0)),
            color::BLACK
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(-0.1, 0.0, 0.0)),
            color::BLACK
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(-1.0, 0.0, 0.0)),
            color::BLACK
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(-1.1, 0.0, 0.0)),
            color::WHITE
        );
    }

    #[test]
    ///Lighting with a pattern applied
    fn lightning_test() {
        let pattern = Pattern::new_stripe_pattern(color::WHITE, color::BLACK);
        let mut m: Material = Material::default_material();
        m.pattern = Some(pattern);
        m.ambiant = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new_point_light(color::WHITE, Tuple::new_point(0.0, 0.0, -10.0));
        let c1 = reflection::lighting(
            &m,
            &light,
            &Tuple::new_point(0.9, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
        );
        let c2 = reflection::lighting(
            &m,
            &light,
            &Tuple::new_point(1.1, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
        );

        assert_eq!(c1, color::WHITE);
        assert_eq!(c2, color::BLACK);
    }
}


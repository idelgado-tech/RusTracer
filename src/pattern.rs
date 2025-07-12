use serde::Serialize;

use crate::{
    color::{self, Color},
    matrix::{Matrix, memoized_inverse},
    shape::{object::Object, shape::Shape},
    tuple::Tuple,
    utils,
};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub transformation: Matrix,
    transformation_inverse: Matrix,
    pub pattern: Patterns,
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern {
            pattern: Patterns::Plain(PlainPattern {
                color: color::WHITE,
            }),
            transformation: Matrix::new_identity_matrix(4),
            transformation_inverse: Matrix::new_identity_matrix(4),
        }
    }
}

impl Pattern {
    pub fn new_stripe_pattern(colors: Vec<Color>) -> Pattern {
        Pattern {
            pattern: Patterns::Stripe(StripePattern { colors }),
            ..Default::default()
        }
    }

    pub fn new_test_pattern() -> Pattern {
        Pattern {
            pattern: Patterns::Test(TestPattern {}),
            ..Default::default()
        }
    }

    pub fn new_gradiant_pattern(from: Color, to: Color) -> Pattern {
        Pattern {
            pattern: Patterns::Gradient(GradientPattern { from, to }),
            ..Default::default()
        }
    }

    pub fn new_radial_gradiant_pattern(color_a: Color, color_b: Color) -> Pattern {
        Pattern {
            pattern: Patterns::RadialGradiant(RadialGradiantPattern { color_a, color_b }),
            ..Default::default()
        }
    }

    //TODO ADD Nested patterns
    //TODO ADD Blended patterns
    //TODO ADD Perturbed patterns

    pub fn new_ring_pattern(colors: Vec<Color>) -> Pattern {
        Pattern {
            pattern: Patterns::Ring(RingPattern { colors }),
            ..Default::default()
        }
    }

    pub fn new_checker_pattern(color_a: Color, color_b: Color) -> Pattern {
        Pattern {
            pattern: Patterns::Checker(CheckerPattern {
                c1: color_a,
                c2: color_b,
            }),
            ..Default::default()
        }
    }

    pub fn get_transform(&self) -> Matrix {
        self.transformation.clone()
    }

    pub fn set_transform(&mut self, new_transformation: &Matrix) {
        self.transformation = new_transformation.clone();
    }

    pub fn color_at_point(&self, point: &Tuple) -> Color {
        match &self.pattern {
            Patterns::Checker(p) => p.pattern_at(point),
            Patterns::Gradient(p) => p.pattern_at(point),
            Patterns::Plain(p) => p.pattern_at(point),
            Patterns::Ring(p) => p.pattern_at(point),
            Patterns::Stripe(p) => p.pattern_at(point),
            Patterns::RadialGradiant(p) => p.pattern_at(point),
            Patterns::Test(p) => p.pattern_at(point),
        }
    }

    pub fn color_at_object(&self, obj: &Object, point: Tuple) -> Color {
        let obj_point = memoized_inverse(obj.get_transform()).unwrap() * point;
        let pattern_point = memoized_inverse(self.get_transform()).unwrap() * obj_point;
        self.color_at_point(&pattern_point)
    }
}

//┌─────────────────────────────────────────────────┐
//│ Inner pattern Type                              │
//└─────────────────────────────────────────────────┘

#[derive(Clone, Debug, PartialEq)]
enum Patterns {
    Checker(CheckerPattern),
    Gradient(GradientPattern),
    Plain(PlainPattern),
    Ring(RingPattern),
    Stripe(StripePattern),
    RadialGradiant(RadialGradiantPattern),
    Test(TestPattern),
}

//┌─────────────────────────────────────────────────┐
//│ Checker pattern                                 │
//└─────────────────────────────────────────────────┘

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerPattern {
    c1: Color,
    c2: Color,
}

impl CheckerPattern {
    fn pattern_at(&self, point: &Tuple) -> Color {
        let sum = point.x.floor() + point.y.floor() + point.z.floor();
        if utils::compare_float(sum % 2.0, 0.0) {
            self.c1
        } else {
            self.c2
        }
    }
}

//┌─────────────────────────────────────────────────┐
//│ Gradient Pattern                                │
//└─────────────────────────────────────────────────┘

#[derive(Clone, Debug, PartialEq)]
pub struct GradientPattern {
    from: Color,
    to: Color,
}

impl GradientPattern {
    fn pattern_at(&self, point: &Tuple) -> Color {
        self.from + (self.to - self.from) * point.x
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RadialGradiantPattern {
    color_a: Color,
    color_b: Color,
}

impl RadialGradiantPattern {
    fn pattern_at(&self, point: &Tuple) -> Color {
        let distance = self.color_b - self.color_a;
        let fraction =
            ((point.x - point.x.floor()).powi(2) + (point.z - point.z.floor()).powi(2)).sqrt();
        self.color_a + distance * fraction
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct PlainPattern {
    color: Color,
}

impl PlainPattern {
    fn pattern_at(&self, _point: &Tuple) -> Color {
        self.color
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct RingPattern {
    colors: Vec<Color>,
}

impl RingPattern {
    fn pattern_at(&self, point: &Tuple) -> Color {
        let distance = (point.x * point.x + point.z * point.z).sqrt();
        let index = distance.floor() as usize % self.colors.len();

        self.colors[index]
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct StripePattern {
    colors: Vec<Color>,
}

impl StripePattern {
    fn pattern_at(&self, point: &Tuple) -> Color {
        let scaled_x = point.x * self.colors.len() as f64;
        let index = (scaled_x.floor().abs() as usize) % self.colors.len();

        self.colors[index]
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct TestPattern {}

impl TestPattern {
    fn pattern_at(&self, point: &Tuple) -> Color {
        Color::new_color(point.x, point.y, point.z)
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::{
        color::{self, BLACK, WHITE},
        reflection::{self, Material, PointLight},
        shape::shape::Shape,
        transformation,
    };

    use super::*;

    #[test]
    ///Creating a stripe pattern
    fn creation_pattern_test() {
        let pattern = Pattern::new_stripe_pattern(vec![color::BLACK, color::WHITE]);
        if let Patterns::Stripe(StripePattern { colors }) = pattern.pattern {
            assert_eq!(colors[0], color::BLACK);
            assert_eq!(colors[1], color::WHITE);
        }
    }

    #[test]
    ///A stripe pattern is constant in y
    fn pattern_y_test() {
        let pattern = Pattern::new_stripe_pattern(vec![color::BLACK, color::WHITE]);

        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 0.0, 0.0)),
            color::BLACK
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 1.0, 0.0)),
            color::BLACK
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 1.0, 0.0)),
            color::BLACK
        );
    }

    #[test]
    ///A stripe pattern is constant in z
    fn pattern_z_test() {
        let pattern = Pattern::new_stripe_pattern(vec![color::BLACK, color::WHITE]);

        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 0.0, 0.0)),
            color::BLACK
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 0.0, 1.0)),
            color::BLACK
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 0.0, 2.0)),
            color::BLACK
        );
    }

    #[test]
    ///A stripe pattern alternates in x
    fn pattern_x_test() {
        let pattern = Pattern::new_stripe_pattern(vec![color::BLACK, color::WHITE]);

        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 0.0, 0.0)),
            color::BLACK
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.9, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(1.0, 0.0, 0.0)),
            color::BLACK
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(-0.1, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(-1.0, 0.0, 0.0)),
            color::BLACK
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(-1.1, 0.0, 0.0)),
            color::WHITE
        );
    }

    #[test]
    ///Lighting with a pattern applied
    fn lightning_test() {
        let pattern = Pattern::new_stripe_pattern(vec![color::BLACK, color::WHITE]);

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
            Object::new_sphere(),
        );
        let c2 = reflection::lighting(
            &m,
            &light,
            &Tuple::new_point(1.1, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
            Object::new_sphere(),
        );

        assert_eq!(c1, color::WHITE);
        assert_eq!(c2, color::BLACK);
    }

    #[test]
    // Scenario: Stripes with an object transformation
    fn stripes_with_object_test() {
        let mut object = Object::new_sphere();
        object.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::new_stripe_pattern(vec![color::BLACK, color::WHITE]);
        let c = pattern.color_at_object(&object, Tuple::new_point(1.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }

    #[test]
    // Scenario: Stripes with a pattern transformation
    fn stripes_with_pattern_test() {
        let object = &Object::new_sphere();
        let mut pattern = Pattern::new_stripe_pattern(vec![color::BLACK, color::WHITE]);
        pattern.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let c = pattern.color_at_object(object, Tuple::new_point(1.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }

    #[test]
    // Scenario: Stripes with both an object and a pattern transformation
    fn stripes_with_pattern_and_object_test() {
        let mut object = Object::new_sphere();
        object.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let mut pattern = Pattern::new_stripe_pattern(vec![color::WHITE, color::BLACK]);
        pattern.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let c = pattern.color_at_object(&object, Tuple::new_point(1.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }

    #[test]
    // Scenario: The default pattern transformation
    fn default_test_pattern() {
        let pattern = Pattern::new_test_pattern();
        assert_eq!(pattern.get_transform(), Matrix::new_identity_matrix(4));
    }

    #[test]
    // Scenario: Assigning a transformation
    fn assigning_transformation_test() {
        let mut pattern = Pattern::new_test_pattern();
        pattern.set_transform(&transformation::create_translation(1.0, 2.0, 3.0));
        assert_eq!(
            pattern.get_transform(),
            transformation::create_translation(1.0, 2.0, 3.0)
        );
    }

    #[test]
    // Scenario: A pattern with an object transformation
    fn pattern_transformation_test() {
        let mut object = Object::new_sphere();
        object.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::new_test_pattern();
        let c = pattern.color_at_object(&object, Tuple::new_point(2.0, 3.0, 4.0));

        assert_eq!(c, Color::new_color(1.0, 1.5, 2.0));
    }

    #[test]
    // Scenario: A pattern with a pattern transformation
    fn pattern_transformation_test_2() {
        let object = Object::new_sphere();
        let mut pattern = Pattern::new_test_pattern();
        pattern.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let c = pattern.color_at_object(&object, Tuple::new_point(2.0, 3.0, 4.0));

        assert_eq!(c, Color::new_color(1.0, 1.5, 2.0));
    }

    #[test]
    // Scenario: A pattern with both an object and a pattern transformation
    fn pattern_transformation_test_3() {
        let mut object = Object::new_sphere();
        object.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));

        let mut pattern = Pattern::new_test_pattern();
        pattern.set_transform(&transformation::create_translation(0.5, 1.0, 1.5));

        let c = pattern.color_at_object(&object, Tuple::new_point(2.5, 3.0, 3.5));

        assert_eq!(c, Color::new_color(0.75, 0.5, 0.25));
    }

    #[test]
    // Scenario: A gradient linearly interpolates between colors
    fn gradiant_pattern_test() {
        let pattern = Pattern::new_gradiant_pattern(WHITE, BLACK);

        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 0.0, 0.0)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.25, 0.0, 0.0)),
            Color::new_color(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.5, 0.0, 0.0)),
            Color::new_color(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.75, 0.0, 0.0)),
            Color::new_color(0.25, 0.25, 0.25)
        );
    }

    #[test]
    // Scenario: A ring should extend in both x and z
    fn ring_pattern_test() {
        let pattern = Pattern::new_ring_pattern(vec![WHITE, BLACK]);

        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 0.0, 0.0)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(1.0, 0.0, 0.0)),
            BLACK
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 0.0, 1.0)),
            BLACK
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.708, 0.0, 0.708)),
            BLACK
        );
    }

    #[test]
    // Scenario: Checkers should repeat in x
    fn checker_pattern_test_x() {
        let pattern = Pattern::new_checker_pattern(WHITE, BLACK);

        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 0.0, 0.0)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.99, 0.0, 0.0)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(1.01, 0.0, 0.0)),
            BLACK
        );
    }

    #[test]
    // Scenario: Checkers should repeat in x
    fn checker_pattern_test_y() {
        let pattern = Pattern::new_checker_pattern(WHITE, BLACK);

        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 0.0, 0.0)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 0.99, 0.0)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 1.01, 0.0)),
            BLACK
        );
    }

    #[test]
    // Scenario: Checkers should repeat in x
    fn checker_pattern_test_z() {
        let pattern = Pattern::new_checker_pattern(WHITE, BLACK);

        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 0.0, 0.0)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 0.0, 0.99)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(&Tuple::new_point(0.0, 0.0, 1.01)),
            BLACK
        );
    }
}

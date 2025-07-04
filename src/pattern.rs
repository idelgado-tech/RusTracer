use crate::{
    color::{self, Color},
    matrix::Matrix,
    shape::shape::Shape,
    tuple::Tuple,
};
use std::rc::Rc;

#[derive(Clone)]
pub struct Pattern {
    pub color_a: Color,
    pub color_b: Color,
    pub transformation_matrix: Matrix,
    pub fonction: Rc<dyn for<'a> Fn(&Pattern, Tuple) -> Color>,
}

impl std::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Pattern")
            .field("color_a", &self.color_a)
            .field("color_b", &self.color_b)
            .field("transformation_matrix", &self.transformation_matrix)
            .finish_non_exhaustive()
    }
}

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.color_a == other.color_a
            && self.color_b == other.color_b
            && self.transformation_matrix == other.transformation_matrix
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
            transformation_matrix: Matrix::new_identity_matrix(4),
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

    pub fn new_test_pattern() -> Pattern {
        let fonction =
            |_pattern: &Pattern, point: Tuple| Color::new_color(point.x, point.y, point.z);

        Pattern::new(color::WHITE, color::BLACK, fonction)
    }

    pub fn new_gradiant_pattern(color_a: Color, color_b: Color) -> Pattern {
        let fonction = |pattern: &Pattern, point: Tuple| {
            let distance = pattern.color_b - pattern.color_a;
            let fraction = point.x - point.x.floor();
            pattern.color_a + distance * fraction
        };

        Pattern::new(color_a, color_b, fonction)
    }

    pub fn new_radial_gradiant_pattern(color_a: Color, color_b: Color) -> Pattern {
        let fonction = |pattern: &Pattern, point: Tuple| {
            let distance = pattern.color_b - pattern.color_a;
            let fraction =
                ((point.x - point.x.floor()).powi(2) + (point.z - point.z.floor()).powi(2)).sqrt();
            pattern.color_a + distance * fraction
        };

        Pattern::new(color_a, color_b, fonction)
    }

    //TODO ADD Nested patterns
    //TODO ADD Blended patterns
    //TODO ADD Perturbed patterns

    pub fn new_ring_pattern(color_a: Color, color_b: Color) -> Pattern {
        let fonction = |pattern: &Pattern, point: Tuple| {
            let square = (point.x.powi(2) + point.z.powi(2)).sqrt();
            if square.floor() % 2.0 == 0.0 {
                pattern.get_color_a()
            } else {
                pattern.get_color_b()
            }
        };

        Pattern::new(color_a, color_b, fonction)
    }

    pub fn new_checker_pattern(color_a: Color, color_b: Color) -> Pattern {
        // Make it sphere friendly
        let fonction = |pattern: &Pattern, point: Tuple| {
            if (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0 == 0.0 {
                pattern.get_color_a()
            } else {
                pattern.get_color_b()
            }
        };

        Pattern::new(color_a, color_b, fonction)
    }

    pub fn get_transform(&self) -> Matrix {
        self.transformation_matrix.clone()
    }

    pub fn set_transform(&mut self, new_transformation: &Matrix) {
        self.transformation_matrix = new_transformation.clone();
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

    pub fn color_at_object(&self, obj: &Box<dyn Shape>, point: Tuple) -> Color {
        let obj_point = obj.get_transform().inverse().unwrap() * point;
        let pattern_point = self.get_transform().inverse().unwrap() * obj_point;
        self.color_at_point(pattern_point)
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::{
        color::{self, BLACK, WHITE},
        reflection::{self, Material, PointLight},
        shape::{shape::Shape, sphere::Sphere},
        transformation,
    };

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
            &Sphere::sphere().box_owned().into(),
        );
        let c2 = reflection::lighting(
            &m,
            &light,
            &Tuple::new_point(1.1, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
            &Sphere::sphere().box_owned().into(),
        );

        assert_eq!(c1, color::WHITE);
        assert_eq!(c2, color::BLACK);
    }

    #[test]
    // Scenario: Stripes with an object transformation
    fn stripes_with_object_test() {
        let mut object: Box<dyn Shape + 'static> = Sphere::sphere().box_owned().into();
        object.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::new_stripe_pattern(color::WHITE, color::BLACK);
        let c = pattern.color_at_object(&object, Tuple::new_point(1.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }

    #[test]
    // Scenario: Stripes with a pattern transformation
    fn stripes_with_pattern_test() {
        let object = &Sphere::sphere().box_owned().into();
        let mut pattern = Pattern::new_stripe_pattern(color::WHITE, color::BLACK);
        pattern.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let c = pattern.color_at_object(object, Tuple::new_point(1.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }

    #[test]
    // Scenario: Stripes with both an object and a pattern transformation
    fn stripes_with_pattern_and_object_test() {
        let mut object: Box<dyn Shape + 'static> = Sphere::sphere().box_owned().into();
        object.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let mut pattern = Pattern::new_stripe_pattern(color::WHITE, color::BLACK);
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
        let mut object = Sphere::sphere().box_owned();
        object.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::new_test_pattern();
        let c = pattern.color_at_object(&object, Tuple::new_point(2.0, 3.0, 4.0));

        assert_eq!(c, Color::new_color(1.0, 1.5, 2.0));
    }

    #[test]
    // Scenario: A pattern with a pattern transformation
    fn pattern_transformation_test_2() {
        let object = Sphere::sphere().box_owned();
        let mut pattern = Pattern::new_test_pattern();
        pattern.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let c = pattern.color_at_object(&object, Tuple::new_point(2.0, 3.0, 4.0));

        assert_eq!(c, Color::new_color(1.0, 1.5, 2.0));
    }

    #[test]
    // Scenario: A pattern with both an object and a pattern transformation
    fn pattern_transformation_test_3() {
        let mut object = Sphere::sphere().box_owned();
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
            pattern.color_at_point(Tuple::new_point(0.0, 0.0, 0.0)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.25, 0.0, 0.0)),
            Color::new_color(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.5, 0.0, 0.0)),
            Color::new_color(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.75, 0.0, 0.0)),
            Color::new_color(0.25, 0.25, 0.25)
        );
    }

    #[test]
    // Scenario: A ring should extend in both x and z
    fn ring_pattern_test() {
        let pattern = Pattern::new_ring_pattern(WHITE, BLACK);

        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 0.0, 0.0)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(1.0, 0.0, 0.0)),
            BLACK
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 0.0, 1.0)),
            BLACK
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.708, 0.0, 0.708)),
            BLACK
        );
    }

    #[test]
    // Scenario: Checkers should repeat in x
    fn checker_pattern_test_x() {
        let pattern = Pattern::new_checker_pattern(WHITE, BLACK);

        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 0.0, 0.0)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.99, 0.0, 0.0)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(1.01, 0.0, 0.0)),
            BLACK
        );
    }

    #[test]
    // Scenario: Checkers should repeat in x
    fn checker_pattern_test_y() {
        let pattern = Pattern::new_checker_pattern(WHITE, BLACK);

        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 0.0, 0.0)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 0.99, 0.0)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 1.01, 0.0)),
            BLACK
        );
    }

    #[test]
    // Scenario: Checkers should repeat in x
    fn checker_pattern_test_z() {
        let pattern = Pattern::new_checker_pattern(WHITE, BLACK);

        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 0.0, 0.0)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 0.0, 0.99)),
            WHITE
        );
        assert_eq!(
            pattern.color_at_point(Tuple::new_point(0.0, 0.0, 1.01)),
            BLACK
        );
    }
}

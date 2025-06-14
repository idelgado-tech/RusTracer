use crate::{color::Color, matrix::Matrix, shape::shape::Shape, tuple::Tuple};
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
        color, matrix,
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
            &Sphere::sphere().box_clone().into(),
        );
        let c2 = reflection::lighting(
            &m,
            &light,
            &Tuple::new_point(1.1, 0.0, 0.0),
            &eyev,
            &normalv,
            false,
            &Sphere::sphere().box_clone().into(),
        );

        assert_eq!(c1, color::WHITE);
        assert_eq!(c2, color::BLACK);
    }

    #[test]
    // Scenario: Stripes with an object transformation
    fn stripes_with_object_test() {
        let mut object: Box<dyn Shape + 'static> = Sphere::sphere().box_clone().into();
        object.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::new_stripe_pattern(color::WHITE, color::BLACK);
        let c = pattern.color_at_object(&object, Tuple::new_point(1.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }

    #[test]
    // Scenario: Stripes with a pattern transformation
    fn stripes_with_pattern_test() {
        let object = &Sphere::sphere().box_clone().into();
        let mut pattern = Pattern::new_stripe_pattern(color::WHITE, color::BLACK);
        pattern.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let c = pattern.color_at_object(object, Tuple::new_point(1.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }

    #[test]
    // Scenario: Stripes with both an object and a pattern transformation
    fn stripes_with_pattern_and_object_test() {
        let mut object: Box<dyn Shape + 'static> = Sphere::sphere().box_clone().into();
        object.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let mut pattern = Pattern::new_stripe_pattern(color::WHITE, color::BLACK);
        pattern.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let c = pattern.color_at_object(&object, Tuple::new_point(1.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }
}

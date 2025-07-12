use crate::{
    color::*,
    pattern::{self, Pattern},
    ray::reflect,
    shape::{object::Object, shape::Shape},
    tuple::Tuple,
};

pub const MAX_RECURTION: usize = 5;

#[derive(Debug, Clone, PartialEq)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple,
}

impl PointLight {
    pub fn new_point_light(intensity: Color, position: Tuple) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambiant: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Pattern>,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl Material {
    pub fn default_material() -> Material {
        Material {
            color: Color::new_color(1.0, 1.0, 1.0),
            ambiant: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }

    pub fn new_material(
        color: Color,
        ambiant: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        reflective: f64,
        transparancy: f64,
        refractive_index: f64,
        pattern: Option<Pattern>,
    ) -> Material {
        Material {
            color,
            ambiant,
            diffuse,
            specular,
            shininess,
            pattern,
            reflective,
            transparency: transparancy,
            refractive_index,
        }
    }

    pub fn set_reflective(&mut self, reflective: f64) -> &Material {
        self.reflective = reflective;
        self
    }

    pub fn set_transparency(&mut self, transparency: f64) -> &Material {
        self.transparency = transparency;
        self
    }

    pub fn set_refractive_index(&mut self, refractive_index: f64) -> &Material {
        self.refractive_index = refractive_index;
        self
    }

    pub fn set_ambiant(&mut self, ambiant: f64) -> &Material {
        self.ambiant = ambiant;
        self
    }

    pub fn set_pattern(&mut self, pattern: Pattern) -> &Material {
        self.pattern = Some(pattern);
        self
    }

    pub fn set_color(&mut self, color: Color) -> &Material {
        self.color = color;
        self
    }
}

pub fn lighting(
    material: &Material,
    light: &PointLight,
    point: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
    in_shadow: bool,
    object: Object,
) -> Color {
    let color = match &material.pattern {
        Some(pattern) => pattern.color_at_object(&object, point.clone()),
        None => material.color,
    }; 
    let effective_color = color* light.intensity;
    let ambiant = effective_color * material.ambiant;

    if in_shadow {
        ambiant
    } else {
        let ligthv = (light.position.clone() - point.clone()).normalize();
        let light_dot_normal = Tuple::dot_product(&ligthv, normalv);
        let diffuse;
        let specular;
        if light_dot_normal < 0.0 {
            diffuse = BLACK;
            specular = BLACK;
        } else {
            diffuse = effective_color * material.diffuse * light_dot_normal;
            let reflectv = reflect(&(ligthv * -1.0), normalv);
            let reflect_dot_eye = Tuple::dot_product(&reflectv, eyev);
            if reflect_dot_eye <= 0.0 {
                specular = BLACK;
            } else {
                let factor = f64::powf(reflect_dot_eye, material.shininess);
                specular = light.intensity * material.specular * factor;
            }
        }
        ambiant + diffuse + specular
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::{
        ray::{Intersection, Ray},
        transformation,
        world::{World, prepare_computations_helper},
    };

    use super::*;

    #[test]
    ///A point light has a position and intensity
    fn point_light_creation() {
        let intensity = Color::new_color(1.0, 1.0, 1.0);
        let position = Tuple::new_point(0.0, 0.0, 0.0);
        let light = PointLight::new_point_light(intensity.clone(), position.clone());

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }

    #[test]
    ///The default material
    fn material_creation() {
        let material = Material::default_material();

        assert_eq!(material.color, Color::new_color(1.0, 1.0, 1.0));
        assert_eq!(material.ambiant, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
    }

    #[test]
    ///Transparency and Refractive Index for the default material
    fn refractive_material_creation() {
        let material = Material::default_material();
        assert_eq!(material.transparency, 0.0);
        assert_eq!(material.refractive_index, 1.0);
    }

    #[test]
    ///A sphere may be assigned a material
    fn sphere_material_creation() {
        let mut s = Object::new_sphere();
        let mut material = Material::default_material();
        material.ambiant = 1.0;
        s.material = material.clone();
        assert_eq!(s.material, material);
    }

    #[test]
    ///Lighting with the eye between the light and the surface
    fn lighting_1() {
        let m = Material::default_material();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new_point_light(
            Color::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 0.0, -10.0),
        );
        let in_shadow = false;

        let result = lighting(
            &m,
            &light,
            &position,
            &eyev,
            &normalv,
            in_shadow,
            Object::new_sphere(),
        );
        assert_eq!(result, Color::new_color(1.9, 1.9, 1.9));
    }

    #[test]
    ///Lighting with the eye between light and surface, eye offset 45°
    fn lighting_2() {
        let m = Material::default_material();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new_point_light(
            Color::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 0.0, -10.0),
        );
        let in_shadow = false;

        let result = lighting(
            &m,
            &light,
            &position,
            &eyev,
            &normalv,
            in_shadow,
            Object::new_sphere(),
        );
        assert_eq!(result, Color::new_color(1.0, 1.0, 1.0));
    }

    #[test]
    ///Lighting with the eye between light and surface, eye offset 45°
    fn lighting_3() {
        let m = Material::default_material();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new_point_light(
            Color::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 10.0, -10.0),
        );
        let in_shadow = false;

        let result = lighting(
            &m,
            &light,
            &position,
            &eyev,
            &normalv,
            in_shadow,
            Object::new_sphere(),
        );
        assert_eq!(
            result,
            Color::new_color(1.6363961030678928, 1.6363961030678928, 1.6363961030678928)
        );
    }

    #[test]
    ///Lighting with the eye between light and surface, eye offset 45°
    fn lighting_4() {
        let m = Material::default_material();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new_point_light(
            Color::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 0.0, 10.0),
        );
        let in_shadow = false;

        let result = lighting(
            &m,
            &light,
            &position,
            &eyev,
            &normalv,
            in_shadow,
            Object::new_sphere(),
        );
        assert_eq!(result, Color::new_color(0.1, 0.1, 0.1));
    }

    #[test]
    /// Lighting with the surface in shadow
    fn lighting_5() {
        let m = Material::default_material();
        let position = Tuple::new_point(0.0, 0.0, 0.0);

        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new_point_light(
            Color::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 0.0, -10.0),
        );
        let in_shadow = true;

        let result = lighting(
            &m,
            &light,
            &position,
            &eyev,
            &normalv,
            in_shadow,
            Object::new_sphere(),
        );
        assert_eq!(result, Color::new_color(0.1, 0.1, 0.1));
    }

    #[test]
    //Scenario : Reflectivity for the default material
    fn reflection_test() {
        let material = Material::default_material();
        assert_eq!(material.reflective, 0.0);
    }

    #[test]
    //Scenario: Precomputing the reflection vector
    fn reflection_precompute_test() {
        let shape = Object::new_plane();
        let r = Ray::new(
            Tuple::new_point(0.0, 1.0, -1.0),
            Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), &shape);
        let comps = prepare_computations_helper(&i, &r);
        assert_eq!(
            comps.reflectv,
            Tuple::new_vector(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    //Scenario: The reflected color for a nonreflective material
    fn reflection_nonreflective_test() {
        let w = World::default_world();

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        let shape = w.objects[1].clone();
        shape.get_material().ambiant = 1.0;
        let i = Intersection::new(1.0, &shape);

        let comps = prepare_computations_helper(&i, &r);
        let color = w.reflected_color(comps, MAX_RECURTION);
        assert_eq!(color, Color::new_color(0.0, 0.0, 0.0));
    }

    #[test]
    //Scenario: The reflected color for a reflective material
    fn reflection_reflective_test() {
        let mut w = World::default_world();
        let mut shape = Object::new_plane();
        shape.set_material(shape.get_material().set_reflective(0.5));
        shape.set_transform(&transformation::create_translation(0.0, -1.0, 0.0));
        w.add_object(shape.clone());

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -3.0),
            Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let i = Intersection::new(2.0_f64.sqrt(), &w.objects.last().unwrap());

        let comps = prepare_computations_helper(&i, &r);
        let color = w.reflected_color(comps, MAX_RECURTION);

        assert_eq!(
            color.normalise(),
            Color::new_color(0.19032, 0.2379, 0.14274).normalise()
        );
    }

    #[test]
    //Scenario: shade_hit() with a reflective material
    fn reflection_shade_hit_test() {
        let mut w = World::default_world();
        let mut shape = Object::new_plane();
        shape.set_material(shape.get_material().set_reflective(0.5));
        shape.set_transform(&transformation::create_translation(0.0, -1.0, 0.0));
        w.add_object(shape.clone());

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -3.0),
            Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let i = Intersection::new(2.0_f64.sqrt(), &w.objects.last().unwrap());

        let comps = prepare_computations_helper(&i, &r);
        let color = w.shade_hit(&comps, MAX_RECURTION);

        assert_eq!(
            color.normalise(),
            Color::new_color(0.87677, 0.92436, 0.82918).normalise()
        );
    }

    #[test]
    //Scenario: color_at() with mutually reflective surfaces
    fn reflection_infinite_recursion_test() {
        let mut w = World::default_world();
        w.light_sources[0] = PointLight::new_point_light(
            Color::new_color(1.0, 1.0, 1.0),
            Tuple::new_point(0.0, 0.0, 0.0),
        );

        let mut lower = Object::new_plane();
        lower.set_material(lower.get_material().set_reflective(1.0));
        lower.set_transform(&transformation::create_translation(0.0, -1.0, 0.0));
        w.add_object(lower.clone());

        let mut upper = Object::new_plane();
        upper.set_material(upper.get_material().set_reflective(1.0));
        upper.set_transform(&transformation::create_translation(0.0, 1.0, 0.0));
        w.add_object(upper.clone());

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );

        let color = w.color_at(&r, MAX_RECURTION);

        assert_eq!(
            color.normalise(),
            Color::new_color(1.0, 1.0, 1.0).normalise()
        );
    }

    #[test]
    //Scenario: color_at() with mutually reflective surfaces
    fn reflection_infinite_max_recursion_test() {
        let mut w = World::default_world();

        let mut shape = Object::new_plane();
        shape.set_material(shape.get_material().set_reflective(0.5));
        shape.set_transform(&transformation::create_translation(0.0, -1.0, 0.0));
        w.add_object(shape.clone());

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -3.0),
            Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );

        let i = Intersection::new(2.0_f64.sqrt(), &w.objects.last().unwrap());

        let comps = prepare_computations_helper(&i, &r);
        let color = w.reflected_color(comps, 0);

        assert_eq!(color, BLACK);
    }
}

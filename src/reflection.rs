use crate::{
    color::*, pattern::{self, Pattern}, ray::reflect, shape::shape::Shape, tuple::Tuple
};

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
        }
    }

    pub fn new_material(
        color: Color,
        ambiant: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
        pattern: Option<Pattern>,
    ) -> Material {
        Material {
            color,
            ambiant,
            diffuse,
            specular,
            shininess,
            pattern,
        }
    }
}

pub fn lighting(
    material: &Material,
    light: &PointLight,
    point: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
    in_shadow: bool,
    object :&Box<dyn Shape>
) -> Color {
    let color = match &material.pattern {
        Some(pattern) => pattern.color_at_object(&object,point.clone()),
        None => material.color,
    };
    let effective_color = color.clone() * light.intensity.clone();
    let ambiant = effective_color.clone() * material.ambiant;

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
                specular = light.intensity.clone() * material.specular * factor;
            }
        }
        ambiant + diffuse + specular
    }
}

#[cfg(test)]
mod matrix_tests {

    use crate::shape::sphere::Sphere;

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
    ///A sphere may be assigned a material
    fn sphere_material_creation() {
        let mut s = Sphere::sphere();
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

        let result = lighting(&m, &light, &position, &eyev, &normalv, in_shadow,
            &Sphere::sphere().box_clone().into(),
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

        let result = lighting(&m, &light, &position, &eyev, &normalv, in_shadow,&Sphere::sphere().box_clone().into(),);
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

        let result = lighting(&m, &light, &position, &eyev, &normalv, in_shadow,&Sphere::sphere().box_clone().into(),);
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

        let result = lighting(&m, &light, &position, &eyev, &normalv, in_shadow,&Sphere::sphere().box_clone().into(),);
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

        let result = lighting(&m, &light, &position, &eyev, &normalv, in_shadow,&Sphere::sphere().box_clone().into(),);
        assert_eq!(result, Color::new_color(0.1, 0.1, 0.1));
    }
}

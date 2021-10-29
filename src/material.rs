use crate::color::Color; 
use crate::light::Light;
use crate::tuple::*;


/// This struct encapsulates the material surface [color: Color]
/// and the four attributes from the Phong reflection model:
/// [ambient: f64] reflection - background light or light reflected from other objects in the environment.
/// This is a constant coloring all points on the surface
/// [diffuse: f64] reflection - light reflected from a mate surface. It depends only on the angle between
/// the light source and the surface normal
/// [specular: f64] reflection - is the reflection of the light source itself alsso called specular light. 
/// Is the bright spot on a curved surface. It dependes only on the angle between the reflecion vector and
/// the eye vector and is controlled by a parameter that we'll call [shiness: f64]. The higher the shiness.
/// the smaller and tighter the specular light
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new(color: Option<Color>, ambient: Option<f64>, 
        diffuse: Option<f64>, specular: Option<f64>, shininess: Option<f64>) -> Self {
        Self {
            color: color.unwrap_or(Color::new(1., 1., 1.)),
            ambient: ambient.unwrap_or(0.1),
            diffuse: diffuse.unwrap_or(0.9),
            specular: specular.unwrap_or(0.9),
            shininess: shininess.unwrap_or(200.0),
        }
    }

    /// This function is what will shade the [Material] and make the shape three-dimensional. It expects 
    /// 4 arguments beside itself, the [light: Light] source, the [point: Point] being illuminated, the [eye: Vector]
    /// and the [normal: Vector] from the Phong reflection models  
    pub fn lighting(&self, light: Light, point: Point, eye: Vector, normal: Vector, in_shadow: bool) -> Color {
        // combine the surface color with the light's color/intesity
        let effective_color = self.color * light.intensity;
        
        // find the direction to the light source
        let lightv = (light.position - point).normalize();
        
        // compute the ambient contribution
        let ambient = effective_color * self.ambient;

        if in_shadow == true {
            return ambient;
        }
        
        let specular: Color;
        let diffuse: Color;
        // light_dot_normal represents the cosine ot the angle between the
        // light vector and the normal vector. A negative number means
        // the light is on the other side of the surface
        let light_dot_normal = lightv.dot(&normal);
        
        if light_dot_normal < 0.0 {
            specular= Color::black();
            diffuse = Color::black();
        } else  {
            // compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the 
            // reflection vector and the eye vector. A negative number meand the 
            // light reflects away from the eye
            let reflectv = (-lightv).reflect(normal);
            let reflect_dot_eye = reflectv.dot(&eye);
            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                // compute the specular contribution
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
                //println!("{:?}", specular);
            }
        } 
        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1., 1., 1.),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }        
    }
}
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Material {
    pub color: [f32; 3],
    pub reflection: f32,
    pub refraction: f32,
    pub shininess: u32,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Object {
    Sphere {
        position: [f32; 3],
        radius: f32,
        material: Material,
    },
    Plane {
        position: [f32; 3],
        normal: [f32; 3],
        material: Material,
    },
}

use crate::bvh::Ray;

impl Object {
    pub fn intersects(&self, ray: &Ray) -> bool {
        match self {
            Object::Sphere { position, radius, .. } => {
                let oc = [
                    ray.origin[0] - position[0],
                    ray.origin[1] - position[1],
                    ray.origin[2] - position[2],
                ];
                let a = ray.direction.iter().map(|&d| d * d).sum::<f32>();
                let b = 2.0 * oc.iter().zip(ray.direction.iter()).map(|(&o, &d)| o * d).sum::<f32>();
                let c = oc.iter().map(|&o| o * o).sum::<f32>() - radius.powi(2);
                let discriminant = b * b - 4.0 * a * c;
                discriminant >= 0.0
            }
            Object::Plane { position, normal, .. } => {
                let denom = normal.iter().zip(ray.direction.iter()).map(|(&n, &d)| n * d).sum::<f32>();
                if denom.abs() > 1e-6 {
                    let d = normal.iter().zip(position.iter().zip(ray.origin.iter()))
                        .map(|(&n, (&p, &o))| n * (p - o))
                        .sum::<f32>() / denom;
                    d >= 0.0
                } else {
                    false
                }
            }
        }
    }
}

mod aabb;
use crate::scene::Object;
use aabb::AABB;

pub struct BVHNode {
    pub bounding_box: AABB,
    pub left: Option<Box<BVHNode>>,
    pub right: Option<Box<BVHNode>>,
    pub object: Option<Object>,
}

impl BVHNode {
    pub fn new(objects: &mut [Object]) -> Self {
        if objects.len() == 1 {
            let bounding_box = Self::calculate_aabb(&objects[0]);
            return BVHNode {
                bounding_box,
                left: None,
                right: None,
                object: Some(objects[0].clone()),
            };
        }

        // Sort along X-axis for simplicity
        objects.sort_by(|a, b| {
            Self::calculate_aabb(a).min[0]
                .partial_cmp(&Self::calculate_aabb(b).min[0])
                .unwrap()
        });

        let mid = objects.len() / 2;
        let (left_objects, right_objects) = objects.split_at_mut(mid);

        let left = Box::new(BVHNode::new(left_objects));
        let right = Box::new(BVHNode::new(right_objects));

        BVHNode {
            bounding_box: AABB::merge(&left.bounding_box, &right.bounding_box),
            left: Some(left),
            right: Some(right),
            object: None,
        }
    }

    fn calculate_aabb(object: &Object) -> AABB {
        match object {
            Object::Sphere { position, radius, .. } => {
                let r = *radius;
                AABB {
                    min: [position[0] - r, position[1] - r, position[2] - r],
                    max: [position[0] + r, position[1] + r, position[2] + r],
                }
            }
            Object::Plane { .. } => AABB {
                min: [f32::NEG_INFINITY; 3],
                max: [f32::INFINITY; 3],
            },
        }
    }
}

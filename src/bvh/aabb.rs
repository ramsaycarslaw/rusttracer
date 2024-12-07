pub struct AABB {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl AABB {
    pub fn merge(a: &AABB, b: &AABB) -> AABB {
        AABB {
            min: [
                a.min[0].min(b.min[0]),
                a.min[1].min(b.min[1]),
                a.min[2].min(b.min[2]),
            ],
            max: [
                a.max[0].max(b.max[0]),
                a.max[1].max(b.max[1]),
                a.max[2].max(b.max[2]),
            ],
        }
    }
}

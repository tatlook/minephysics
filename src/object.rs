use sfml::system::Vector2f;

#[derive(Clone, Debug)]
pub struct Object {
    /// Position
    p: Vector2f,

    /// Veclocity
    v: Vector2f,

    /// Mass
    m: f32,
}

static GRAVITATIONAL_CONSTANT: f32 = 10.;

fn normalize(vec: &Vector2f) -> Vector2f {
    vec.to_owned() / vec.length_sq().sqrt()
}

impl Object {
    pub fn new<T: Into<Vector2f>>(p: T, v: T, m: f32) -> Self {
        Object {
            p: p.into(),
            v: v.into(),
            m,
        }
    }

    pub fn perform_position(&mut self, dt: f32) {
        self.p += self.v * dt
    }

    pub fn position(&self) -> Vector2f {
        self.p
    }

    pub fn mass(&self) -> f32 {
        self.m
    }

    pub fn perform_force(&mut self, other: Vec<Object>, dt: f32) {
        let mut sum_f = Vector2f::new(0.0, 0.0);
        for o in other {
            let distance = o.p - self.p;
            if distance.length_sq() == 0.0 {
                continue;
            }
            let distance_normal = normalize(&distance);
            let distance_len_sq = distance.length_sq();
            let f = self.m * o.m / distance_len_sq * GRAVITATIONAL_CONSTANT;

            sum_f += distance_normal * f;
        }
        let a = sum_f / self.m;
        self.v += a * dt;
    }
}

pub fn perform(objects: &mut Vec<Object>, dt: f32) {
    for i in 0..objects.len() {
        let mut other = objects.clone();
        other.remove(i);
        let o = &mut objects[i];
        o.perform_force(other, dt);
    }
    for o in objects {
        o.perform_position(dt);
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use sfml::system::Vector2f;

    use super::normalize;

    #[test]
    fn test_normalize() {
        let vec = Vector2f::new(10.0, 0.0);
        let norm = normalize(&vec);
        assert_relative_eq!(norm.x, 1.0);
        assert_relative_eq!(norm.y, 0.0);

        let vec = Vector2f::new(3.0, 4.0);
        let norm = normalize(&vec);
        assert_relative_eq!(norm.x, 0.6);
        assert_relative_eq!(norm.y, 0.8);
    }
}

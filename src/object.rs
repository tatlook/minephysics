use std::f32::consts::PI;

use sfml::system::Vector2f;

/// Circle object. Has position, veclocity and mass
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
    /// Make a object with given position, veclocity and mass.
    pub fn new<T: Into<Vector2f>>(p: T, v: T, m: f32) -> Self {
        Object {
            p: p.into(),
            v: v.into(),
            m,
        }
    }

    pub fn position(&self) -> Vector2f {
        self.p
    }

    /// Gets radius. The object is a circle and its area means its mass.
    pub fn radius(&self) -> f32 {
        self.m.sqrt() * PI
    }

    /// Momentum
    fn p(&self) -> Vector2f {
        self.v * self.m
    }
}

impl Object {
    pub fn perform_position(&mut self, dt: f32) {
        self.perform_movement(dt);
        self.perform_border_mirror(dt);
    }

    /// Move to a new position based on speed and elapsed time.
    fn perform_movement(&mut self, dt: f32) {
        self.p += self.v * dt
    }

    const BORDER_LEFT: f32 = 0.0;
    const BORDER_RIGHT: f32 = 2000.0;
    const BORDER_UP: f32 = 0.0;
    const BORDER_DOWN: f32 = 2000.0;
    const CENTER: Vector2f = Vector2f::new(
        (Self::BORDER_LEFT + Self::BORDER_RIGHT) / 2.0,
        (Self::BORDER_UP + Self::BORDER_DOWN) / 2.0,
    );

    fn is_out_border(&self) -> bool {
        self.p.x > Self::BORDER_RIGHT
            || self.p.x < Self::BORDER_LEFT
            || self.p.y < Self::BORDER_UP
            || self.p.y > Self::BORDER_DOWN
    }

    /// Sends body from border to center
    #[allow(dead_code)]
    fn perform_border_send(&mut self, _dt: f32) {
        if self.is_out_border() {
            self.p = Self::CENTER;
        }
    }

    /// Sends body to the opposite position.
    fn perform_border_mirror(&mut self, _dt: f32) {
        if self.is_out_border() {
            // Not 100% opposite, because then this method sends body back and splashs
            // To get 100% oppisite: self.p = Self::CENTER * 2.0 - self.p
            self.p = Self::CENTER - (self.p - Self::CENTER) * 0.9
        }
    }
}

impl Object {
    /// Change veclocity by interaction of other objets.
    pub fn perform_force(&mut self, other: Vec<Object>, dt: f32) {
        self.perform_gravity(&other, dt);
        self.perform_collision(&other, dt);
    }

    /// Newton's gravity
    fn perform_gravity(&mut self, other: &Vec<Object>, dt: f32) {
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

    /// Inelastic collision
    #[allow(dead_code)]
    fn perform_stick(&mut self, other: &Vec<Object>, _dt: f32) {
        for o in other {
            let distance = o.p - self.p;
            if distance.length_sq().sqrt() >= o.radius() + self.radius() {
                continue;
            }
            self.v = (self.p() + o.p()) / (self.m + o.m)
        }
    }

    /// Elastic collision
    #[allow(dead_code)]
    fn perform_collision(&mut self, other: &Vec<Object>, _dt: f32) {
        for o in other {
            let distance = o.p - self.p;
            if distance.length_sq().sqrt() >= o.radius() + self.radius() {
                continue;
            }
            self.v = (self.v * (self.m - o.m) - o.p() * 2.0) / (self.m + o.m)
        }
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

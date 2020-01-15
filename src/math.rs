use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Rect2 {
    pub center: Vec2,
    pub radius: Vec2,
}

impl Rect2 {
    pub fn zero() -> Rect2 {
        Rect2 {
            center: Vec2::zero(),
            radius: Vec2::zero(),
        }
    }

    pub fn minkowski_sum(&self, other: &Rect2) -> Rect2 {
        Rect2 {
            center: self.center,
            radius: self.radius + other.radius,
        }
    }
}

pub struct Line2 {
    pub start: Vec2,
    pub end: Vec2,
}

impl Line2 {
    pub fn new(start: Vec2, end: Vec2) -> Line2 {
        Line2 { start, end }
    }

    pub fn point(&self, t: f32) -> Vec2 {
        self.start + (self.end - self.start) * t
    }

    pub fn truncate(&mut self, t: f32) {
        self.end = self.point(t);
    }

    pub fn vec(&self) -> Vec2 {
        self.end - self.start
    }
}

/// See https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line
pub fn line_intersection(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
) -> Option<(f32, f32)> {
    let det = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if det == 0.0 {
        return None;
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / det;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / det;

    Some((t, u))
}

#[derive(Copy, Clone)]
pub struct Collision2 {
    pub t: f32,
    pub normal: Vec2,
}

pub fn swept_aabb2(
    movement: &Line2,
    move_radius: Vec2,
    obstacle_center: Vec2,
    obstacle_radius: Vec2,
) -> Option<Collision2> {
    // Minkowski sum
    let obstacle_radius = move_radius + obstacle_radius;
    let edges = [
        // top
        Line2::new(
            Vec2::new(
                obstacle_center.x - obstacle_radius.x,
                obstacle_center.y + obstacle_radius.y,
            ),
            Vec2::new(
                obstacle_center.x + obstacle_radius.x,
                obstacle_center.y + obstacle_radius.y,
            ),
        ),
        // down
        Line2::new(
            Vec2::new(
                obstacle_center.x + obstacle_radius.x,
                obstacle_center.y - obstacle_radius.y,
            ),
            Vec2::new(
                obstacle_center.x - obstacle_radius.x,
                obstacle_center.y - obstacle_radius.y,
            ),
        ),
        // left
        Line2::new(
            Vec2::new(
                obstacle_center.x - obstacle_radius.x,
                obstacle_center.y - obstacle_radius.y,
            ),
            Vec2::new(
                obstacle_center.x - obstacle_radius.x,
                obstacle_center.y + obstacle_radius.y,
            ),
        ),
        // right
        Line2::new(
            Vec2::new(
                obstacle_center.x + obstacle_radius.x,
                obstacle_center.y + obstacle_radius.y,
            ),
            Vec2::new(
                obstacle_center.x + obstacle_radius.x,
                obstacle_center.y - obstacle_radius.y,
            ),
        ),
    ];

    edges
        .iter()
        .filter_map(|edge| {
            if let Some((t, u)) = line_intersection(
                movement.start.x,
                movement.start.y,
                movement.end.x,
                movement.end.y,
                edge.start.x,
                edge.start.y,
                edge.end.x,
                edge.end.y,
            ) {
                if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
                    return Some((edge, t));
                }
            }

            None
        })
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(edge, t)| Collision2 {
            t,
            normal: (edge.end - edge.start).normalized().perp(),
        })
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[inline(always)]
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    #[inline(always)]
    pub fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }

    pub fn len(&self) -> f32 {
        self.len2().sqrt()
    }

    pub fn len2(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalized(&self) -> Vec2 {
        let len = self.len();

        if len == 0.0 {
            return Vec2::zero();
        }

        Vec2::new(self.x / len, self.y / len)
    }

    pub fn perp(&self) -> Vec2 {
        Vec2::new(-self.y, self.x)
    }

    pub fn cross(&self, other: &Vec2) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn reflect(&self, normal: &Vec2) -> Vec2 {
        self - 2.0 * normal * (normal * self)
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2::zero()
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<Vec2> for &Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub for &Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<f32> for &Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<&Vec2> for f32 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, rhs: &Vec2) -> Self::Output {
        Vec2::new(self * rhs.x, self * rhs.y)
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Mul for Vec2 {
    type Output = f32;

    #[inline(always)]
    fn mul(self, rhs: Vec2) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul for &Vec2 {
    type Output = f32;

    #[inline(always)]
    fn mul(self, rhs: &Vec2) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

pub fn clamp<T>(min: T, val: T, max: T) -> T
where
    T: PartialOrd,
{
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

pub fn aabb_vs_aabb(p1: Vec2, half_size1: Vec2, p2: Vec2, half_size2: Vec2) -> bool {
    intersect(
        p1.x - half_size1.x,
        p1.x + half_size1.x,
        p2.x - half_size2.x,
        p2.x + half_size2.x,
    ) && intersect(
        p1.y - half_size1.y,
        p1.y + half_size1.y,
        p2.y - half_size2.y,
        p2.y + half_size2.y,
    )
}

#[inline(always)]
fn intersect(left1: f32, right1: f32, left2: f32, right2: f32) -> bool {
    !(left2 > right1 || right2 < left1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_swept_aabb2() {
        let object_center = Vec2::new(0.0, 10.0);
        let object_velocity = Vec2::new(0.0, -5.0);
        let object_radius = Vec2::new(1.0, 1.0);

        let obstacle_center = Vec2::new(0.0, -20.0);
        let obstacle_velocity = Vec2::new(0.0, 50.0);
        let obstacle_radius = Vec2::new(10.0, 1.0);

        let collision = swept_aabb2(
            &Line2::new(object_center, object_center + object_velocity),
            object_radius,
            obstacle_center,
            obstacle_radius,
        );
        assert!(collision.is_none());

        let collision = swept_aabb2(
            &Line2::new(
                object_center,
                object_center + object_velocity - obstacle_velocity,
            ),
            object_radius,
            obstacle_center,
            object_radius,
        );
        assert!(collision.is_some());
        let collision = collision.unwrap();
        assert!(collision.t >= 0.0 && collision.t <= 1.0);
    }

    #[test]
    fn test_reflect() {
        let normal = Vec2::new(-1.0, 0.0);
        assert_eq!(Vec2::new(1.0, 0.0).reflect(&normal), Vec2::new(-1.0, 0.0));
    }
}

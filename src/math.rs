use std::ops::{Add, Mul};

#[derive(Copy, Clone, Debug)]
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
}

impl Add for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
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

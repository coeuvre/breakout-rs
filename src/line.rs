#[derive(Copy, Clone)]
pub struct Pixel {
    pub x: i32,
    pub y: i32,
    pub aa: f32,
}

// Bresenham's Line Algorithm
pub struct LineIter {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    n: i32,
    inc_x: i32,
    inc_y: i32,
    inc_e: i32,
    inc_ne: i32,
    d: i32,
}

impl LineIter {
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> LineIter {
        let mut dx = x1 - x0;
        let mut inc_x = 1;
        if dx < 0 {
            inc_x = -1;
            dx = -dx;
        }

        let mut dy = y1 - y0;
        let mut inc_y = 1;
        if dy < 0 {
            inc_y = -1;
            dy = -dy;
        }

        let dx2 = dx + dx;
        let dy2 = dy + dy;
        let d = if dx > dy { dy2 - dx } else { dx2 - dy };

        LineIter {
            x: x0,
            y: y0,
            dx,
            dy,
            n: (if dx > dy {
                if inc_x > 0 {
                    x1 - x0
                } else {
                    x0 - x1
                }
            } else {
                if inc_y > 0 {
                    y1 - y0
                } else {
                    y0 - y1
                }
            }) + 1,
            inc_x,
            inc_y,
            inc_e: if dx > dy { dy2 } else { dx2 },
            inc_ne: if dx > dy { dy2 - dx2 } else { dx2 - dy2 },
            d,
        }
    }
}

impl Iterator for LineIter {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n > 0 {
            self.n -= 1;

            let result = Some(Pixel {
                x: self.x,
                y: self.y,
                aa: 1.0,
            });

            if self.dx > self.dy {
                self.x += self.inc_x;

                if self.inc_x > 0 && self.d <= 0 || self.inc_x < 0 && self.d < 0 {
                    self.d += self.inc_e;
                } else {
                    self.d += self.inc_ne;
                    self.y += self.inc_y;
                }
            } else {
                self.y += self.inc_y;

                if self.inc_y > 0 && self.d <= 0 || self.inc_y < 0 && self.d < 0 {
                    self.d += self.inc_e;
                } else {
                    self.d += self.inc_ne;
                    self.x += self.inc_x;
                }
            }

            result
        } else {
            None
        }
    }
}

pub fn line_iter(x0: i32, y0: i32, x1: i32, y1: i32) -> LineIter {
    LineIter::new(x0, y0, x1, y1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_octant_1() {
        let points: Vec<_> = line_iter(0, 0, 4, 3).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[0, 0], [1, 1], [2, 1], [3, 2], [4, 3]]);
    }

    #[test]
    fn test_symmetry_1() {
        let points: Vec<_> = line_iter(4, 3, 0, 0).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[4, 3], [3, 2], [2, 1], [1, 1], [0, 0]]);
    }

    #[test]
    fn test_octant_2() {
        let points: Vec<_> = line_iter(0, 0, 3, 4).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[0, 0], [1, 1], [1, 2], [2, 3], [3, 4]]);
    }

    #[test]
    fn test_symmetry_2() {
        let points: Vec<_> = line_iter(3, 4, 0, 0).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[3, 4], [2, 3], [1, 2], [1, 1], [0, 0]]);
    }

    #[test]
    fn test_octant_3() {
        let points: Vec<_> = line_iter(0, 0, -3, 4).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[0, 0], [-1, 1], [-1, 2], [-2, 3], [-3, 4]]);
    }

    #[test]
    fn test_symmetry_3() {
        let points: Vec<_> = line_iter(-3, 4, 0, 0).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[-3, 4], [-2, 3], [-1, 2], [-1, 1], [0, 0]]);
    }

    #[test]
    fn test_octant_4() {
        let points: Vec<_> = line_iter(0, 0, -4, 3).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[0, 0], [-1, 1], [-2, 2], [-3, 2], [-4, 3]]);
    }

    #[test]
    fn test_symmetry_4() {
        let points: Vec<_> = line_iter(-4, 3, 0, 0).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[-4, 3], [-3, 2], [-2, 2], [-1, 1], [0, 0]]);
    }

    #[test]
    fn test_octant_5() {
        let points: Vec<_> = line_iter(0, 0, -4, -3).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[0, 0], [-1, -1], [-2, -2], [-3, -2], [-4, -3]]);
    }

    #[test]
    fn test_symmetry_5() {
        let points: Vec<_> = line_iter(-4, -3, 0, 0).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[-4, -3], [-3, -2], [-2, -2], [-1, -1], [0, 0]]);
    }

    #[test]
    fn test_octant_6() {
        let points: Vec<_> = line_iter(0, 0, -3, -4).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[0, 0], [-1, -1], [-2, -2], [-2, -3], [-3, -4]]);
    }

    #[test]
    fn test_symmetry_6() {
        let points: Vec<_> = line_iter(-3, -4, 0, 0).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[-3, -4], [-2, -3], [-2, -2], [-1, -1], [0, 0]]);
    }

    #[test]
    fn test_octant_7() {
        let points: Vec<_> = line_iter(0, 0, 3, -4).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[0, 0], [1, -1], [2, -2], [2, -3], [3, -4]]);
    }

    #[test]
    fn test_symmetry_7() {
        let points: Vec<_> = line_iter(3, -4, 0, 0).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[3, -4], [2, -3], [2, -2], [1, -1], [0, 0]]);
    }

    #[test]
    fn test_octant_8() {
        let points: Vec<_> = line_iter(0, 0, 4, -3).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[0, 0], [1, -1], [2, -1], [3, -2], [4, -3]]);
    }

    #[test]
    fn test_symmetry_8() {
        let points: Vec<_> = line_iter(4, -3, 0, 0).map(|p| [p.x, p.y]).collect();
        assert_eq!(points, vec![[4, -3], [3, -2], [2, -1], [1, -1], [0, 0]]);
    }
}

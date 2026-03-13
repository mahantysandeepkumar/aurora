#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn right(&self) -> i32 {
        self.x + self.width
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.height
    }

    #[allow(dead_code)] // TODO: use it to remove
    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x <= self.right()
            && point.y >= self.y
            && point.y <= self.bottom()
    }

    #[allow(dead_code)] // TODO: use it to remove
    pub fn center(&self) -> Point {
        Point {
            x: self.x + self.width / 2,
            y: self.y + self.height / 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_right() {
        let r = Rect::new(10, 0, 100, 50);
        assert_eq!(r.right(), 110);
    }

    #[test]
    fn rect_bottom() {
        let r = Rect::new(0, 20, 100, 50);
        assert_eq!(r.bottom(), 70);
    }

    #[test]
    fn rect_contains_point() {
        let r = Rect::new(0, 0, 100, 100);

        assert!(r.contains(Point { x: 50, y: 50 }));
        assert!(!r.contains(Point { x: 150, y: 50 }));
    }

    #[test]
    fn rect_center() {
        let r = Rect::new(0, 0, 100, 100);

        let center = r.center();

        assert_eq!(center.x, 50);
        assert_eq!(center.y, 50);
    }
}

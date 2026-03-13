use super::geometry::{Point, Rect};

pub const SNAP_THRESHOLD: i32 = 30;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SnapRegion {
    None,
    Left,
    Right,
    Top,
}

pub fn detect_snap(pointer: Point, screen: Rect) -> SnapRegion {
    if pointer.x <= screen.x + SNAP_THRESHOLD {
        return SnapRegion::Left;
    }

    if pointer.x >= screen.right() - SNAP_THRESHOLD {
        return SnapRegion::Right;
    }

    if pointer.y <= screen.y + SNAP_THRESHOLD {
        return SnapRegion::Top;
    }

    SnapRegion::None
}

pub fn snap_rect(region: SnapRegion, screen: Rect) -> Option<Rect> {
    match region {
        SnapRegion::Left => Some(Rect::new(
            screen.x,
            screen.y,
            screen.width / 2,
            screen.height,
        )),

        SnapRegion::Right => Some(Rect::new(
            screen.x + screen.width / 2,
            screen.y,
            screen.width / 2,
            screen.height,
        )),

        SnapRegion::Top => Some(Rect::new(screen.x, screen.y, screen.width, screen.height)),

        SnapRegion::None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wm::geometry::{Point, Rect};

    #[test]
    fn detect_left_snap() {
        let screen = Rect::new(0, 0, 1000, 800);

        let pointer = Point { x: 5, y: 200 };

        let result = detect_snap(pointer, screen);

        assert_eq!(result, SnapRegion::Left);
    }

    #[test]
    fn detect_right_snap() {
        let screen = Rect::new(0, 0, 1000, 800);

        let pointer = Point { x: 995, y: 200 };

        let result = detect_snap(pointer, screen);

        assert_eq!(result, SnapRegion::Right);
    }

    #[test]
    fn snap_rect_left() {
        let screen = Rect::new(0, 0, 1000, 800);

        let rect = snap_rect(SnapRegion::Left, screen).unwrap();

        assert_eq!(rect.width, 500);
        assert_eq!(rect.height, 800);
        assert_eq!(rect.x, 0);
    }
}

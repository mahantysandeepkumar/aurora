use super::geometry::Rect;
use super::snap::{SnapRegion, snap_rect};

pub fn preview_rect(region: SnapRegion, screen: Rect) -> Option<Rect> {
    snap_rect(region, screen)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wm::geometry::Rect;
    use crate::wm::snap::SnapRegion;

    #[test]
    fn preview_left_half() {
        let screen = Rect::new(0, 0, 1000, 800);

        let rect = preview_rect(SnapRegion::Left, screen).unwrap();

        assert_eq!(rect.width, 500);
        assert_eq!(rect.height, 800);
    }

    #[test]
    fn preview_none() {
        let screen = Rect::new(0, 0, 1000, 800);

        let rect = preview_rect(SnapRegion::None, screen);

        assert!(rect.is_none());
    }
}

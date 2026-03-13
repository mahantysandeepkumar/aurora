use crate::wm::geometry::Rect;

#[allow(dead_code)] // TODO: use it to remove
pub fn enforce_bounds(window: Rect, screen: Rect) -> Rect {
    let mut x = window.x;
    let mut y = window.y;

    // clamp left
    if x < screen.x {
        x = screen.x;
    }

    // clamp right
    if x + window.width > screen.right() {
        x = screen.right() - window.width;
    }

    // calmp top
    if y < screen.y {
        y = screen.y;
    }

    // clamp bottom
    if y + window.height > screen.bottom() {
        y = screen.bottom() - window.height;
    }

    Rect::new(x, y, window.width, window.height)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wm::geometry::Rect;

    #[test]
    fn clamp_left_edge() {
        let screen = Rect::new(0, 0, 1000, 800);
        let window = Rect::new(-50, 100, 200, 200);

        let result = enforce_bounds(window, screen);

        assert_eq!(result.x, 0);
    }

    #[test]
    fn clamp_right_edge() {
        let screen = Rect::new(0, 0, 1000, 800);
        let window = Rect::new(900, 100, 200, 200);

        let result = enforce_bounds(window, screen);

        assert_eq!(result.x, 800);
    }

    #[test]
    fn clamp_bottom_edge() {
        let screen = Rect::new(0, 0, 1000, 800);
        let window = Rect::new(100, 700, 200, 200);

        let result = enforce_bounds(window, screen);

        assert_eq!(result.y, 600);
    }
}

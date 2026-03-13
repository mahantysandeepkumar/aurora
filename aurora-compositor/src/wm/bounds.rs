use crate::wm::geometry::Rect;

pub fn enforce_bounds(window: Rect, screen: Rect) -> Rect {
    let mut x = window.x;
    let mut y = window.y;

    let visible_w = window.width / 3;
    let visible_h = window.height / 3;

    // left
    if x < screen.x - window.width + visible_w {
        x = screen.x - window.width + visible_w;
    }

    // right
    if x + visible_w > screen.right() {
        x = screen.right() - visible_w;
    }

    // top
    if y < screen.y {
        y = screen.y;
    }

    // bottom
    if y + visible_h > screen.bottom() {
        y = screen.bottom() - visible_h;
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
        assert_eq!(result.x, -50);
    }

    #[test]
    fn clamp_left_edge_check_max_outside_screen() {
        let screen = Rect::new(0, 0, 1000, 800);
        let window = Rect::new(-200, 100, 200, 200);

        let result = enforce_bounds(window, screen);

        assert_eq!(result.x, -134);
    }

    #[test]
    fn clamp_right_edge() {
        let screen = Rect::new(0, 0, 1000, 800);
        let window = Rect::new(900, 100, 200, 200);

        let result = enforce_bounds(window, screen);

        assert_eq!(result.x, 900);
    }

    #[test]
    fn clamp_bottom_edge() {
        let screen = Rect::new(0, 0, 1000, 800);
        let window = Rect::new(100, 700, 200, 200);

        let result = enforce_bounds(window, screen);

        assert_eq!(result.y, 700);
    }
}

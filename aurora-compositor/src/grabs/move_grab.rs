//! Move grab is the state of a composer during which the client window is being dragged around.
//!
//! eg. Usually whenever a user clicks on the app's titlebar and starts dragging, the compositors
//! enters a MoveSurfaceGrab state.

use crate::{
    Aurora,
    wm::{
        geometry::Rect,
        preview::preview_rect,
        snap::{SnapRegion, detect_snap},
    },
};
use smithay::{
    desktop::Window,
    input::pointer::{
        AxisFrame, ButtonEvent, GestureHoldBeginEvent, GestureHoldEndEvent, GesturePinchBeginEvent,
        GesturePinchEndEvent, GesturePinchUpdateEvent, GestureSwipeBeginEvent,
        GestureSwipeEndEvent, GestureSwipeUpdateEvent, GrabStartData as PointerGrabStartData,
        MotionEvent, PointerGrab, PointerInnerHandle, RelativeMotionEvent,
    },
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    utils::{Logical, Point},
};
use wayland_protocols::xdg::shell::server::xdg_toplevel;
pub struct MoveSurfaceGrab {
    pub start_data: PointerGrabStartData<Aurora>,
    pub window: Window,
    pub initial_window_location: Point<i32, Logical>,
}

impl PointerGrab<Aurora> for MoveSurfaceGrab {
    fn motion(
        &mut self,
        data: &mut Aurora,
        handle: &mut PointerInnerHandle<'_, Aurora>,
        _focus: Option<(WlSurface, Point<f64, Logical>)>,
        event: &MotionEvent,
    ) {
        // While the grab is active, no client has pointer focus
        handle.motion(data, None, event);

        // --- MOVE WINDOW (existing behavior) ---

        let delta = event.location - self.start_data.location;
        let new_location = self.initial_window_location.to_f64() + delta;

        data.space
            .map_element(self.window.clone(), new_location.to_i32_round(), true);

        // --- SNAP DETECTION ---

        // Get current output (monitor)
        let output = data.space.outputs().next().unwrap();
        let output_geo = data.space.output_geometry(output).unwrap();

        // Convert Smithay geometry → our Rect
        let screen = Rect::new(
            output_geo.loc.x,
            output_geo.loc.y,
            output_geo.size.w,
            output_geo.size.h,
        );

        // Convert pointer position → our Point
        let pointer = crate::wm::geometry::Point {
            x: event.location.x as i32,
            y: event.location.y as i32,
        };

        // Detect snap region
        let region = detect_snap(pointer, screen);

        // Update compositor state
        data.active_snap = region;
        data.snap_preview = preview_rect(region, screen);

        // Optional debug while testing
        // println!("Snap region: {:?}", region);
    }

    fn relative_motion(
        &mut self,
        data: &mut Aurora,
        handle: &mut PointerInnerHandle<'_, Aurora>,
        focus: Option<(WlSurface, Point<f64, Logical>)>,
        event: &RelativeMotionEvent,
    ) {
        handle.relative_motion(data, focus, event);
    }

    fn button(
        &mut self,
        data: &mut Aurora,
        handle: &mut PointerInnerHandle<'_, Aurora>,
        event: &ButtonEvent,
    ) {
        handle.button(data, event);

        // BTN_LEFT from linux/input-event-codes.h
        const BTN_LEFT: u32 = 0x110;

        if !handle.current_pressed().contains(&BTN_LEFT) {
            // --- SNAP APPLY ---
            // Use the preview rect directly instead of recomputing snap
            if let Some(rect) = data.snap_preview {
                // move window
                data.space
                    .map_element(self.window.clone(), (rect.x, rect.y), true);

                // resize window
                if let Some(xdg) = self.window.toplevel() {
                    xdg.with_pending_state(|state| {
                        state.size = Some((rect.width, rect.height).into());

                        // set maximized if active snap region = top
                        if data.active_snap == SnapRegion::Top {
                            state.states.set(xdg_toplevel::State::Maximized);
                        }
                    });
                    xdg.send_pending_configure();
                }
            }

            // --- RESET SNAP STATE ---
            data.snap_preview = None;
            data.active_snap = SnapRegion::None;

            // --- RELEASE GRAB ---
            handle.unset_grab(self, data, event.serial, event.time, true);
        }
    }

    fn axis(
        &mut self,
        data: &mut Aurora,
        handle: &mut PointerInnerHandle<'_, Aurora>,
        details: AxisFrame,
    ) {
        handle.axis(data, details)
    }

    fn frame(&mut self, data: &mut Aurora, handle: &mut PointerInnerHandle<'_, Aurora>) {
        handle.frame(data);
    }

    fn gesture_swipe_begin(
        &mut self,
        data: &mut Aurora,
        handle: &mut PointerInnerHandle<'_, Aurora>,
        event: &GestureSwipeBeginEvent,
    ) {
        handle.gesture_swipe_begin(data, event)
    }

    fn gesture_swipe_update(
        &mut self,
        data: &mut Aurora,
        handle: &mut PointerInnerHandle<'_, Aurora>,
        event: &GestureSwipeUpdateEvent,
    ) {
        handle.gesture_swipe_update(data, event)
    }

    fn gesture_swipe_end(
        &mut self,
        data: &mut Aurora,
        handle: &mut PointerInnerHandle<'_, Aurora>,
        event: &GestureSwipeEndEvent,
    ) {
        handle.gesture_swipe_end(data, event)
    }

    fn gesture_pinch_begin(
        &mut self,
        data: &mut Aurora,
        handle: &mut PointerInnerHandle<'_, Aurora>,
        event: &GesturePinchBeginEvent,
    ) {
        handle.gesture_pinch_begin(data, event)
    }

    fn gesture_pinch_update(
        &mut self,
        data: &mut Aurora,
        handle: &mut PointerInnerHandle<'_, Aurora>,
        event: &GesturePinchUpdateEvent,
    ) {
        handle.gesture_pinch_update(data, event)
    }

    fn gesture_pinch_end(
        &mut self,
        data: &mut Aurora,
        handle: &mut PointerInnerHandle<'_, Aurora>,
        event: &GesturePinchEndEvent,
    ) {
        handle.gesture_pinch_end(data, event)
    }

    fn gesture_hold_begin(
        &mut self,
        data: &mut Aurora,
        handle: &mut PointerInnerHandle<'_, Aurora>,
        event: &GestureHoldBeginEvent,
    ) {
        handle.gesture_hold_begin(data, event)
    }

    fn gesture_hold_end(
        &mut self,
        data: &mut Aurora,
        handle: &mut PointerInnerHandle<'_, Aurora>,
        event: &GestureHoldEndEvent,
    ) {
        handle.gesture_hold_end(data, event)
    }

    fn start_data(&self) -> &PointerGrabStartData<Aurora> {
        &self.start_data
    }

    fn unset(&mut self, _data: &mut Aurora) {}
}

use smithay::{
    delegate_data_device, delegate_seat,
    input::{
        Seat, SeatHandler, SeatState,
        dnd::{DnDGrab, DndGrabHandler, GrabType, Source},
        pointer::Focus,
    },
    utils::Serial,
    wayland::selection::{
        SelectionHandler,
        data_device::{
            DataDeviceHandler, DataDeviceState, WaylandDndGrabHandler, set_data_device_focus,
        },
    },
};
use wayland_server::{Resource, protocol::wl_surface::WlSurface};

use crate::state::Aurora;

impl SeatHandler for Aurora {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Aurora> {
        &mut self.seat_state
    }

    fn cursor_image(
        &mut self,
        _seat: &Seat<Self>,
        _image: smithay::input::pointer::CursorImageStatus,
    ) {
    }

    fn focus_changed(&mut self, seat: &Seat<Self>, focused: Option<&WlSurface>) {
        let dh = &self.display_handle;
        let client = focused.and_then(|s| dh.get_client(s.id()).ok());
        set_data_device_focus(dh, seat, client);
    }
}

delegate_seat!(Aurora);

//
// Wl Data Device
//

impl SelectionHandler for Aurora {
    type SelectionUserData = ();
}

impl DataDeviceHandler for Aurora {
    fn data_device_state(&mut self) -> &mut DataDeviceState {
        &mut self.data_device_state
    }
}

impl DndGrabHandler for Aurora {}
impl WaylandDndGrabHandler for Aurora {
    fn dnd_requested<S: Source>(
        &mut self,
        source: S,
        _icon: Option<WlSurface>,
        seat: Seat<Self>,
        serial: Serial,
        type_: GrabType,
    ) {
        match type_ {
            GrabType::Pointer => {
                let ptr = seat.get_pointer().unwrap();
                let start_data = ptr.grab_start_data().unwrap();

                // create a dnd grab to start the operation
                let grab = DnDGrab::new_pointer(&self.display_handle, start_data, source, seat);
                ptr.set_grab(self, grab, serial, Focus::Keep);
            }
            GrabType::Touch => {
                // Aurora lacks touch handling
                source.cancel();
            }
        }
    }
}

delegate_data_device!(Aurora);

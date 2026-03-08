use smithay::backend::renderer::element::texture::TextureBuffer;
use smithay::backend::renderer::gles::GlesTexture;
use smithay::desktop::{PopupManager, Space, Window, WindowSurfaceType};
use smithay::input::{Seat, SeatState};
use smithay::reexports::calloop::generic::Generic;
use smithay::reexports::calloop::{EventLoop, Interest, LoopSignal, Mode, PostAction};
use smithay::reexports::wayland_server::backend::{ClientData, ClientId, DisconnectReason};
use smithay::utils::{Logical, Point};
use smithay::wayland::compositor::{CompositorClientState, CompositorState};
use smithay::wayland::selection::data_device::DataDeviceState;
use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::Arc;
use wayland_server::protocol::wl_surface::WlSurface;
// shared memory buffers
use smithay::wayland::output::OutputManagerState;
use smithay::wayland::shm::ShmState;

// XDG
use smithay::wayland::shell::xdg::XdgShellState;
use smithay::wayland::socket::ListeningSocketSource;
use wayland_server::{Display, DisplayHandle};

use crate::desktop::wallpaper::{self, scan_for_wallpapers};

pub struct Aurora {
    pub start_time: std::time::Instant,
    pub socket_name: OsString,
    pub display_handle: DisplayHandle,
    pub space: Space<Window>,
    // state for compositor
    pub compositor_state: CompositorState,
    // shared memory buffer
    pub shm_state: ShmState,
    pub xdg_shell_state: XdgShellState,
    pub popups: PopupManager,
    pub output_manager_state: OutputManagerState,
    pub seat: Seat<Self>,
    pub seat_state: SeatState<Aurora>,
    pub data_device_state: DataDeviceState,
    pub loop_signal: LoopSignal,
    pub window_spawn_count: usize,

    // Wallpaper
    pub wallpapers: Vec<PathBuf>,                      // All wallpapers
    pub wallpaper: Option<TextureBuffer<GlesTexture>>, // current wallpaper
    pub wallpaper_size: Option<(i32, i32)>,
}

impl Aurora {
    pub fn new(event_loop: &mut EventLoop<Self>, display: Display<Self>) -> Self {
        let start_time = std::time::Instant::now();

        let dh = display.handle();

        // initialize compositor state
        let shm_state = ShmState::new::<Aurora>(&dh, vec![]);
        let xdg_shell_state = XdgShellState::new::<Aurora>(&dh);
        let compositor_state = CompositorState::new::<Aurora>(&dh);
        let popups = PopupManager::default();
        let output_manager_state = OutputManagerState::new_with_xdg_output::<Self>(&dh);

        // Data device is responsible for clipboard and drag-and-drop
        let data_device_state = DataDeviceState::new::<Self>(&dh);

        // A seat is a group of keyboards, pointer and touch devices.
        // A seat typically has a pointer and maintains a keyboard focus and a pointer focus.
        let mut seat_state = SeatState::new();
        let mut seat: Seat<Self> = seat_state.new_wl_seat(&dh, "winit");

        // Notify clients that we have a keyboard, for the sake of the example we assume that keyboard is always present.
        // You may want to track keyboard hot-plug in real compositor.
        seat.add_keyboard(Default::default(), 200, 25).unwrap();

        // Notify clients that we have a pointer (mouse)
        // Here we assume that there is always pointer plugged in
        seat.add_pointer();

        // A space represents a two-dimensional plane. Windows and Outputs can be mapped onto it.
        //
        // Windows get a position and stacking order through mapping.
        // Outputs become views of a part of the Space and can be rendered via Space::render_output.
        let space = Space::default();

        let socket_name = Self::init_wayland_listener(display, event_loop);
        let loop_signal = event_loop.get_signal();

        let window_spawn_count: usize = 0;

        // wallapaers
        let wallpapers = scan_for_wallpapers("/home/mahantys/Pictures");
        Self {
            start_time,
            socket_name,
            display_handle: dh,
            space,
            compositor_state: compositor_state,
            shm_state,
            xdg_shell_state,
            popups,
            output_manager_state,
            seat,
            seat_state,
            data_device_state,
            loop_signal,
            window_spawn_count,
            wallpapers,
            wallpaper: None,
            wallpaper_size: None,
        }
    }

    fn init_wayland_listener(
        display: Display<Aurora>,
        event_loop: &mut EventLoop<Self>,
    ) -> OsString {
        // creates a new socket for wayland client connections (next avaiulable wayland socket name)
        let listening_socket = ListeningSocketSource::with_name("aurora-0").unwrap();

        let socket_name = listening_socket.socket_name().to_os_string();

        let loop_handle = event_loop.handle();

        loop_handle
            .insert_source(listening_socket, move |client_stream, _, state| {
                state
                    .display_handle
                    .insert_client(client_stream, Arc::new(ClientState::default()))
                    .unwrap();
            })
            .expect("Failed to init Wayland Source");

        loop_handle
            .insert_source(
                Generic::new(display, Interest::READ, Mode::Level),
                |_, display, state| {
                    unsafe {
                        display.get_mut().dispatch_clients(state).unwrap();
                    }
                    Ok(PostAction::Continue)
                },
            )
            .unwrap();

        socket_name
    }

    pub fn surface_under(
        &self,
        pos: Point<f64, Logical>,
    ) -> Option<(WlSurface, Point<f64, Logical>)> {
        self.space
            .element_under(pos)
            .and_then(|(window, location)| {
                window
                    .surface_under(pos - location.to_f64(), WindowSurfaceType::ALL)
                    .map(|(s, p)| (s, (p + location).to_f64()))
            })
    }
}

//
// ---- ClientState (per Wayland client data) ----
//

#[derive(Default)]
pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {
    fn initialized(&self, _client_id: ClientId) {
        println!("Client initialized");
    }

    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {
        println!("Client disconnected");
    }
}

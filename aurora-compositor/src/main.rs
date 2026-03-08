use smithay::reexports::calloop::EventLoop;
use wayland_server::Display;
mod desktop;
mod grabs;
mod handlers;
mod inputs;
mod state;
mod winit;

use state::Aurora;

use crate::winit::init_winit;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    println!("Aurora compositor starting...");
    let mut event_loop = EventLoop::try_new()?;

    let display: Display<Aurora> = Display::new()?;
    let mut state = Aurora::new(&mut event_loop, display);

    // create winnit backend
    let _ = init_winit(&mut event_loop, &mut state);

    // start listening to wayland socket
    unsafe {
        std::env::set_var("WAYLAND_DISPLAY", &state.socket_name);
    }

    event_loop.run(None, &mut state, move |_| {
        // wayland event loop
    })?;

    Ok(())
}

fn init_logging() {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    } else {
        tracing_subscriber::fmt().init();
    }
}

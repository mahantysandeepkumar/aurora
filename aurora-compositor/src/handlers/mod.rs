mod compositors;
mod inputs;
mod xdg_shell;
use smithay::{delegate_output, wayland::output::OutputHandler};

use crate::Aurora;

impl OutputHandler for Aurora {}
delegate_output!(Aurora);

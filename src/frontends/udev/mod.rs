pub mod cursor;
pub mod drawing;
pub mod input_handler;
pub mod window_map;

use slog::{crit, error, info, warn, Logger};

use smithay::{
    backend::session::auto::AutoSession,
    reexports::{
        calloop::{
            timer::{Timer, TimerHandle},
            EventLoop, LoopHandle, LoopSignal,
        },
        wayland_server::Display,
    },
    wayland::compositor,
};

use std::{cell::RefCell, rc::Rc};
pub fn run_udev(log: Logger) {
    // creates loop
    let event_loop: EventLoop<LoopSignal> = EventLoop::try_new().unwrap();
    let loop_handle = event_loop.handle();

    let display = Rc::new(RefCell::new(Display::new()));
    let display_name = display
        .borrow_mut()
        .add_socket_auto()
        .unwrap()
        .into_string()
        .unwrap();
    info!(log, "Listening on wayland socket"; "name" => display_name.clone());
    std::env::set_var("WAYLAND_DISPLAY", display_name);

    let (session, notifier) = match AutoSession::new(log.clone()) {
        Some(ret) => ret,
        None => {
            crit!(log, "Could not initiliaze a session");
            return;
        }
    };
}

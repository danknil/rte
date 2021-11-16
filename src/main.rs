pub mod frontends;

use crate::frontends::{udev::run_udev, winit::run_winit, x11::run_x11};
use slog::{crit, info, o, Drain};

static POSSIBLE_BACKENDS: &[&str] = &[
    #[cfg(feature = "winit")]
    "--winit : Run anvil as a X11 or Wayland client using winit.",
    #[cfg(feature = "udev")]
    "--tty-udev : Run anvil as a tty udev client (requires root if without logind).",
    #[cfg(feature = "x11")]
    "--x11 : Run anvil as an X11 client.",
];

fn main() {
    let log = if std::env::var("MUTEX_LOG").is_ok() {
        slog::Logger::root(
            std::sync::Mutex::new(slog_term::term_full().fuse()).fuse(),
            o!(),
        )
    } else {
        slog::Logger::root(
            slog_async::Async::default(slog_term::term_full().fuse()).fuse(),
            o!(),
        )
    };
    let _guard = slog_scope::set_global_logger(log.clone());
    slog_stdlog::init().expect("Could not setup log backend");

    let arg = std::env::args().nth(1);
    match arg.as_ref().map(|s| &s[..]) {
        #[cfg(feature = "winit")]
        Some("--winit") => {
            info!(log, "Starting rte with winit backend");
            run_winit(log);
        }
        #[cfg(feature = "udev")]
        Some("--udev") => {
            info!(log, "Starting rte on a tty using udev");
            run_udev(log);
        }
        #[cfg(feature = "x11")]
        Some("--x11") => {
            info!(log, "Starting rte with x11 backend");
            run_x11(log);
        }
        Some(other) => {
            crit!(log, "Unknown backend {}", other);
        }
        None => {
            println!("USAGE: rte --{{backend}}");
            println!();
            println!("Possible backends are:");
            for b in POSSIBLE_BACKENDS {
                println!("\t{}", b);
            }
        }
    }
}

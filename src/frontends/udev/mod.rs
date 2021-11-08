pub mod cursor;
pub mod drawing;
pub mod input_handler;
pub mod window_map;

use slog::Logger;
use smithay::reexports::calloop::EventLoop;

pub fn run_udev(log: Logger) {
    let mut event_loop = EventLoop::try_new().unwrap();
    let display = Rc::new(RefCell::new(Display::new()));
}

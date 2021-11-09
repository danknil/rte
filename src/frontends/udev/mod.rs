pub mod cursor;
pub mod drawing;
pub mod input_handler;
pub mod window_map;

use slog::Logger;
use slog::{crit, error, info, warn};
use smithay::reexports::calloop::{
    timer::{Timer, TimerHandle},
    EventLoop, LoopHandle, LoopSignal,
};
pub fn run_udev(log: Logger) {
    let event_loop: EventLoop<LoopSignal> = EventLoop::try_new().unwrap();
    let loop_handle = event_loop.handle();
    info!(log, "Successful created event loop");
}

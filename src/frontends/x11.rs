use slog::{crit, error, info, warn, Logger};
use smithay::reexports::calloop::{
    timer::{Timer, TimerHandle},
    EventLoop, LoopHandle, LoopSignal,
};
pub fn run_x11(log: Logger) {
    let event_loop: EventLoop<LoopSignal> = EventLoop::try_new().unwrap();
    let loop_handle = event_loop.handle();
    info!(log, "Successful created event loop");
}

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Instant,
};

use slog::{error, Logger};

#[cfg(feature = "xwayland")]
use smithay::xwayland::{XWayland, XWaylandEvent};

use smithay::{
    reexports::{
        calloop::{generic::Generic, Interest, LoopHandle, Mode, PostAction},
        wayland_server::{protocol::wl_surface::WlSurface, Display},
    },
    utils::{Logical, Point},
    wayland::{
        output::xdg::init_xdg_output_manager,
        seat::{CursorImageStatus, KeyboardHandle, PointerHandle, Seat},
        shm::init_shm_global,
        xdg_activation::{init_xdg_activation_global, XdgActivationEvent},
    },
};

#[derive(Debug)]
pub struct RteState<BackendData> {
    pub backend_data: BackendData,
    pub socket_name: Option<String>,
    pub running: Arc<AtomicBool>,
    pub display: Rc<RefCell<Display>>,
    pub handle: LoopHandle<'static, RteState<BackendData>>,
    pub dnd_icon: Arc<Mutex<Option<WlSurface>>>,
    pub log: Logger,

    // input related
    pub pointer: PointerHandle,
    pub keyboard: KeyboardHandle,
    pub suppressed_keys: Vec<u32>,
    pub pointer_location: Point<f64, Logical>,
    pub cursor_status: Arc<Mutex<CursorImageStatus>>,
    pub seat_name: String,
    pub seat: Seat,
    pub start_time: Instant,

    // things we must keep alive
    #[cfg(feature = "xwayland")]
    pub xwayland: XWayland<RteState<BackendData>>,
}

impl<BackendData: Backend + 'static> RteState<BackendData> {
    fn init(
        display: Rc<RefCell<Display>>,
        handle: LoopHandle<'static, RteState<BackendData>>,
        backend_data: BackendData,
        log: Logger,
        listen_on_socket: bool,
    ) -> RteState<BackendData> {
        handle
            .insert_source(
                Generic::from_fd(display.borrow().get_poll_fd(), Interest::READ, Mode::Level),
                move |_, _, state: &mut RteState<BackendData>| {
                    let display = state.display.clone();
                    let mut display = display.borrow_mut();
                    match display.dispatch(std::time::Duration::from_millis(0), state) {
                        Ok(_) => Ok(PostAction::Continue),
                        Err(e) => {
                            error!(state.log, "I/O error on the Wayland display: {}", e);
                            state.running.store(false, Ordering::SeqCst);
                            Err(e)
                        }
                    }
                },
            )
            .unwrap();

        // TODO: init window map

        //init the basic compositor globals
        init_shm_global(&mut (*display).borrow_mut(), vec![], log.clone());

        // TODO:: init shell states

        init_xdg_output_manager(&mut display.borrow_mut(), log.clone());
        init_xdg_activation_global(
            &mut display.borrow_mut(),
            |state, req, mut ddata| {
                let rte_state = ddata.get::<RteState<BackendData>>().unwrap();
                match req {
                    XdgActivationEvent::RequestActivation {
                        token,
                        token_data,
                        surface,
                    } => {
                        if token_data.timestamp.elapsed().as_secs() < 10 {
                            todo!();
                        } else {
                            // Discard the request
                            state.lock().unwrap().remove_request(&token);
                        }
                    }
                    XdgActivationEvent::DestroyActivationRequest { .. } => {}
                }
            },
            log.clone(),
        );
    }
}

pub trait Backend {
    fn seat_name(&self) -> String;
}

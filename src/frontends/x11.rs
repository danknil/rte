use std::{cell::RefCell, rc::Rc};

use slog::{crit, error, info, warn, Logger};

#[cfg(feature = "egl")]
use smithay::{backend::renderer::ImportDma, wayland::dmabuf::init_dmabuf_global};
use smithay::{
    backend::{
        egl::{EGLContext, EGLDisplay},
        renderer::{gles2::Gles2Renderer, ImportEgl},
        x11::{X11Backend, X11Event, X11Surface},
    },
    reexports::{
        calloop::{
            timer::{Timer, TimerHandle},
            EventLoop, LoopHandle, LoopSignal,
        },
        wayland_server::Display,
    },
    wayland::{
        output::{Mode, PhysicalProperties},
        seat::CursorImageStatus,
    },
};

pub const OUTPUT_NAME: &str = "x11";

pub struct X11Data {
    render: bool,
    mode: Mode,
    surface: X11Surface,
}

pub fn run_x11(log: Logger) {
    let event_loop: EventLoop<LoopSignal> = EventLoop::try_new().unwrap();
    let display = Rc::new(RefCell::new(Display::new()));

    let (backend, surface) = X11Backend::with_title("rte", log.clone()).unwrap();
    let window = backend.window();

    let egl = EGLDisplay::new(&surface, log.clone()).unwrap();
    let context = EGLContext::new(&egl, log.clone()).unwrap();
    let renderer = unsafe { Gles2Renderer::new(context, log.clone()) }.unwrap();
    let renderer = Rc::new(RefCell::new(renderer));

    #[cfg(feature = "egl")]
    {
        if renderer
            .borrow_mut()
            .bind_wl_display(&*display.borrow())
            .is_ok()
        {
            info!(log, "EGL hardware-acceleration enabled");
            let dmabuf_formats = renderer
                .borrow_mut()
                .dmabuf_formats()
                .cloned()
                .collect::<Vec<_>>();
            let renderer = renderer.clone();
            init_dmabuf_global(
                &mut *display.borrow_mut(),
                dmabuf_formats,
                move |buffer, _| renderer.borrow_mut().import_dmabuf(buffer).is_ok(),
                log.clone(),
            );
        }
    }

    let size = {
        let s = backend.window().size();

        (s.w as i32, s.h as i32).into()
    };

    let mode = Mode {
        size,
        refresh: 60_000,
    };

    event_loop
        .handle()
        .insert_source(backend, |event, _window, state| match event {
            _ => {}
        })
        .unwrap();
}

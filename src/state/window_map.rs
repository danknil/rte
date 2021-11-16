use std::sync::Mutex;

use smithay::{
    reexports::{
        wayland_protocols::xdg_shell::server::xdg_toplevel,
        wayland_server::protocol::wl_surface::WlSurface,
    },
    utils::{Logical, Point, Rectangle},
    wayland::{
        compositor::with_states,
        shell::{
            legacy::ShellSurface,
            xdg::{PopupSurface, ToplevelSurface, XdgPopupSurfaceRoleAttributes},
        },
    },
};

#[cfg(feature = "xwayland")]
use crate::xwayland::X11Surface;

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Xdg(ToplevelSurface),
    Wl(ShellSurface),
    #[cfg(feature = "xwayland")]
    X11(X11Surface),
}

impl Kind {
    pub fn alive(&self) -> bool {
        match *self {
            Kind::Xdg(ref t) => t.alive(),
            Kind::Wl(ref t) => t.alive(),
            #[cfg(feature = "xwayland")]
            Kind::X11(ref t) => t.alive(),
        }
    }
    pub fn get_surface(&self) -> Option<&WlSurface> {
        match *self {
            Kind::Xdg(ref t) => t.get_surface(),
            Kind::Wl(ref t) => t.get_surface(),
            #[cfg(feature = "xwayland")]
            Kind::X11(ref t) => t.get_surface(),
        }
    }
    pub fn set_activated(&self, active: bool) {
        if let Kind::Xdg(ref t) = self {
            let changed = t.with_pending_state(|state| {
                if active {
                    state.states.set(xdg_toplevel::State::Activated)
                } else {
                    state.states.unset(xdg_toplevel::State::Activated)
                }
            });
            if let Ok(true) = changed {
                t.send_configure();
            }
        }
    }
}

pub enum PopupKind {
    Xdg(PopupSurface),
}

impl PopupKind {
    fn alive(&self) -> bool {
        match *self {
            PopupKind::Xdg(ref t) => t.alive(),
        }
    }
    pub fn get_surface(&self) -> Option<&WlSurface> {
        match *self {
            PopupKind::Xdg(ref t) => t.get_surface(),
        }
    }
    pub fn location(&self) -> Point<i32, Logical> {
        let wl_surface = match self.get_surface() {
            Some(s) => s,
            None => return (0, 0).into(),
        };
        with_states(wl_surface, |states| {
            states
                .data_map
                .get::<Mutex<XdgPopupSurfaceRoleAttributes>>()
                .unwrap()
                .lock()
                .unwrap()
                .current
                .geometry
        })
        .unwrap_or_default()
        .loc
    }
}

#[derive(Debug)]
struct Window {
    location: Point<i32, Logical>,
    bbox: Rectangle<i32, Logical>,
    toplevel: Kind,
}

impl Window {
    fn matching(&self, point: Point<f64, Logical>) -> Option<(WlSurface, Point<i32, Logical>)> {}
}

pub(crate) use self::platform::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "global/desktop.rs"]
mod platform;
#[cfg(target_arch = "wasm32")]
#[path = "global/wasm.rs"]
mod platform;

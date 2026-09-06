//! Host adaptations. Native stack growth is not available on wasm32.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use std::time::Instant;
#[cfg(target_arch = "wasm32")]
pub(crate) use web_time::Instant;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use stacker::maybe_grow;

#[cfg(target_arch = "wasm32")]
pub(crate) fn maybe_grow<R>(_: usize, _: usize, callback: impl FnOnce() -> R) -> R {
    callback()
}

// Keep native behavior unchanged; wasm has a fixed stack instead of stacker.
pub(crate) const MAX_PARSE_DEPTH: usize = if cfg!(target_arch = "wasm32") {
    64
} else {
    1000
};
pub(crate) const MAX_EVAL_DEPTH: usize = if cfg!(target_arch = "wasm32") {
    64
} else {
    302
};

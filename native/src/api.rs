pub use anyhow::{anyhow, Result};
use flutter_rust_bridge::SyncReturn;
pub use generational_arena::Arena;

use crate::{
    render::{
        move_state_to_ui_thread as render_move_new_state_to_ui_thread, receive_resize, render_frame,
    },
    state::STATE,
};

pub use std::{
    ops::Deref,
    sync::{Mutex, RwLock},
};

// For initialization
pub fn say_hello() {}

pub fn move_state_to_ui_thread() -> Result<SyncReturn<()>> {
    render_move_new_state_to_ui_thread()?;
    Ok(SyncReturn(()))
}

pub fn request_draw() -> Result<SyncReturn<()>> {
    render_frame()?;
    Ok(SyncReturn(()))
}

pub fn request_resize(width: u32, height: u32) -> Result<SyncReturn<()>> {
    receive_resize(width, height)?;
    render_frame()?;
    Ok(SyncReturn(()))
}

pub fn set_current_time(time: f64) -> SyncReturn<()> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let state = state.as_mut().unwrap();
        state.time = time;
        SyncReturn(())
    })
}

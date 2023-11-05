
#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, Ordering};

pub mod alpha_beta_search;
mod quiescence_search;
mod mcts;
pub mod search_info;
pub mod traits;
pub mod temp;

static STOP_BUFFER: AtomicBool = AtomicBool::new(false);

pub fn clear_stop() {
    STOP_BUFFER.store(false, Ordering::Relaxed);
}

pub fn emit_stop() {
    STOP_BUFFER.store(true, Ordering::Relaxed);
}

pub fn query_stop() -> bool {
    STOP_BUFFER.load(Ordering::Relaxed)
}

static SEARCH_THREAD_RUNNING: AtomicBool = AtomicBool::new(false);

pub fn clear_is_running() {
    SEARCH_THREAD_RUNNING.store(false, Ordering::Relaxed);
}

pub fn emit_is_running() {
    SEARCH_THREAD_RUNNING.store(true, Ordering::Relaxed);
}

pub fn query_is_running() -> bool {
    SEARCH_THREAD_RUNNING.load(Ordering::Relaxed)
}

/*
TODO:
    - MVV/LVA
    - does evaluation need to be flipped?
    - transposition tables (separate for quiescence and alpha-beta?)
    - remove mut from references where unnecessary (legal & loud moves)
    - disallow go if another thread is running!
*/

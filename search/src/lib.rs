
/*
TODO:
    - transposition tables separate for quiescence and alpha-beta?
    - disallow go if another thread is running!
*/

use std::sync::atomic::{AtomicBool, Ordering};
pub mod transposition_table;
pub mod alpha_beta;
mod mcts;
pub mod search_info;
pub mod traits;
mod optimizer_generics;
pub mod minimax;
mod quiescence;


pub const I32_NAN: i32 = 0;
const STOP_CHECKING_PERIOD: usize = 4096;
const MAX_QUIESCENCE_DEPTH: u8 = 64;
const MATE_EVALUATION: i32 = 30_000;


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

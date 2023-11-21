use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use search::{clear_stop, emit_stop, query_stop};
use search::alpha_beta::alpha_beta;
use search::traits::{AlphaBetaSearchFunctionality, AlphaBetaAndQuiescenceSearchFunctionality, SearchableMove};
use search::transposition_table::TranspositionTable;
use crate::parsing::bestmove;

pub struct GoInfo<Board> {
    _phantom: std::marker::PhantomData<Board>,  // placeholder for "searchmoves: Vec<Board::Move>"

    pub whites_turn: bool,

    // time left and increment after move
    pub wtime_given: bool,
    pub wtime: usize,
    pub btime_given: bool,
    pub btime: usize,
    pub winc_given: bool,
    pub winc: usize,
    pub binc_given: bool,
    pub binc: usize,
    pub movestogo_given: bool,
    pub movestogo: usize,

    pub infinite: bool,  // search until "stop" command

    pub movetime_given: bool,
    pub movetime: usize,  // how long to search for (in ms)

    pub depth_given: bool,
    pub depth: usize,  // search until this depth
}

impl<Board> Default for GoInfo<Board> {
    fn default() -> Self {
        Self{
            _phantom: std::marker::PhantomData,
            whites_turn: false,
            wtime_given: false,
            wtime: 0,
            btime_given: false,
            btime: 0,
            winc_given: false,
            winc: 0,
            binc_given: false,
            binc: 0,
            movestogo_given: false,
            movestogo: 0,
            infinite: false,
            movetime_given: false,
            movetime: 0,
            depth_given: false,
            depth: 0,
        }
    }
}

impl<Board> GoInfo<Board> where
    Board: AlphaBetaAndQuiescenceSearchFunctionality + Send + 'static,
    <Board as AlphaBetaSearchFunctionality>::ZobristHash: Send + Sync,
    <Board as AlphaBetaSearchFunctionality>::Move: Send + Sync
{
    fn calculate_search_time(self: &Self) -> Duration {
        // TODO: movestogo
        return Duration::from_millis(
            if self.movetime_given {
                self.movetime
            } else if self.whites_turn {
                self.wtime / 20
            } else {
                self.btime / 20
            } as u64
        );
    }

    pub fn search(
        self: &Self,
        mut board: Board,
        transposition_table_arc_mutex: Arc<Mutex<TranspositionTable<Board>>>
    ) {

        // clear old stop signal
        clear_stop();

        // decide whether search is times or a max depth is given
        let timed: bool = self.movestogo_given
            ||self.movetime_given
            ||self.wtime_given
            ||self.btime_given
            ||self.winc_given
            ||self.binc_given;
        let max_depth_given: bool = self.depth_given;
        let max_depth: u8 = if max_depth_given {self.depth as u8} else {u8::MAX};

        // maybe time the search
        if timed {
            let remaining_time = self.calculate_search_time();
            let increment = Duration::from_micros(512);
            thread::spawn(move || 'thread_block: {
                let now = std::time::Instant::now();
                while now.elapsed() < remaining_time {
                    if query_stop() {
                        break 'thread_block;
                    }
                    thread::sleep(increment);
                }
                emit_stop();
            });
        }


        // search!
        thread::spawn(move || {

            // access (mutable) reference to transposition table
            let mut guard = transposition_table_arc_mutex
                .lock().expect("Couldn't access transposition table in search thread!");
            let transposition_table = guard.deref_mut();

            // do search
            let mut current_max_depth: u8 = 1;
            let mut maybe_best_move: Option<Board::Move> = None;
            let mut _maybe_evaluation: f32 = f32::NAN;
            loop {  // iterative deepening

                /*
                TODO:
                    - print PV, info, ...
                */

                print!("Searching to depth: {} ...", current_max_depth);

                // do search to current depth
                let current_search_info = alpha_beta(
                    &mut board, current_max_depth, transposition_table
                );

                // break if stop signal was received and alpha_beta returned early
                if query_stop() {
                    println!(" Terminated");
                    break;
                }

                println!(
                    " done! bestmove: {}, evaluation: {}",
                    current_search_info.best_move.unwrap().to_string(),
                    current_search_info.evaluation
                );

                current_search_info.visualize();


                current_max_depth += 1;
                _maybe_evaluation = current_search_info.evaluation;
                maybe_best_move = current_search_info.best_move;

                // break if search of final depth is done
                if max_depth_given {
                    if current_max_depth > max_depth {
                        break;
                    }
                }
            };

            // echo bestmove and possibly information (TODO)
            match maybe_best_move {
                None => panic!(
                    "Iterative deepening failed to complete a full \
                    iteration or last complete iteration failed to produce a best move!"
                ),
                Some(r#move) => {
                    bestmove(r#move,None/*TODO*/)
                }
            }
        });
    }
}


use std::thread;
use std::time::Duration;

use search::{clear_stop, emit_stop, query_stop};
use search::traits::{AlphaBetaAndQuiescenceSearchFunctionality, SearchableMove};
use search::temp::{alpha_beta, SearchInfo};
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

impl<Board: AlphaBetaAndQuiescenceSearchFunctionality + Send + 'static> GoInfo<Board> {
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

    pub fn search(self: &Self, mut board: Board) {

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

            let mut current_max_depth: u8 = 4;
            let mut search_info: SearchInfo<Board::Move> = SearchInfo::default();
            loop {  // iterative deepening

                /*
                TODO:
                    - print PV, info, ...
                */

                print!("Searching to depth: {} ...", current_max_depth);

                // do search to current depth
                let current_search_info = alpha_beta(
                    &mut board, current_max_depth
                );

                // break if stop signal was received and alpha_beta returned early
                if query_stop() {
                    println!(" Terminated");
                    break;
                }

                current_max_depth += 1;
                search_info = current_search_info;

                println!(
                    " done! bestmove: {}, evaluation: {}",
                    search_info.best_move.unwrap().to_string(), search_info.evaluation
                );

                // break if search of final depth is done
                if max_depth_given {
                    if current_max_depth > max_depth {
                        break;
                    }
                }
            }

            // echo bestmove and possibly information (TODO)
            bestmove(
                search_info.best_move.expect("Search failed to find a valid move!"),
                None  // TODO
            )
        });
    }
}

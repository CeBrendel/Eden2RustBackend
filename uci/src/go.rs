
use std::thread;
use std::time::Duration;

use generic_magic::True;
use search::{clear_stop, emit_stop, query_stop};
use search::alpha_beta_search::alpha_beta_search;
use search::traits::{AlphaBetaSearchFunctionality, Move};
use search::search_info::SearchInfo;


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

impl<Board: AlphaBetaSearchFunctionality + Send + 'static> GoInfo<Board> {
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

        clear_stop();

        let timed: bool = self.movestogo_given
            ||self.movetime_given
            ||self.wtime_given
            ||self.btime_given
            ||self.winc_given
            ||self.binc_given;
        let max_depth_given: bool = self.depth_given;
        let max_depth: u8 = if max_depth_given {self.depth as u8} else {u8::MAX};

        // timer
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

        // searcher
        thread::spawn(move || {

            let mut search_info = SearchInfo::default();
            let mut current_bestmove: Option<Board::Move> = None;
            let mut current_evaluation: f32 = 0.;
            let mut current_depth: u8 = 4;
            loop {  // iterative deepening

                /*
                TODO:
                    - print PV, info, ...
                */

                print!("Searching to depth: {} ...", current_depth);

                let (maybe_bestmove, evaluation) = alpha_beta_search::<
                    Board,
                    True,  // CountNodes: Bool
                    True  // CheckStop: Bool
                >(&mut board, current_depth, &mut search_info);

                if query_stop() {
                    println!(" Terminated");
                    println!(
                        "bestmove {} evaluation {}",
                        current_bestmove.expect("No move found!").to_string(),
                        current_evaluation
                    );
                    break;
                }

                current_bestmove = maybe_bestmove;
                current_evaluation = evaluation;
                current_depth += 1;

                println!(
                    " done! bestmove: {}, evaluation: {}",
                    current_bestmove.unwrap().to_string(), evaluation
                );

                if max_depth_given {
                    if current_depth > max_depth {
                        println!(
                            "bestmove {} evaluation {}",
                            current_bestmove.expect("No move found!").to_string(),
                            current_evaluation
                        );
                        break;
                    }
                }
            }
        });
    }
}

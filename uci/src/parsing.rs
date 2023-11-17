use std::sync::{Arc, Mutex};
use board::board::Board;
use board::moves::Move;

use search::emit_stop;
use search::traits::SearchableMove;
use search::transposition_table::TranspositionTable;

use crate::go::GoInfo;

const NAME: &'static str = "|אֶמֶת|";
const AUTHOR: &'static str = "Cedric Brendel";


fn remove_whitespace_prefix(s: &str) -> &str {
    s.trim_start_matches(" ")
}


fn prefix_until_whitespace(s: &str) -> &str {
    match s.find(" ") {
        None => s,
        Some(i) => &s[..i]
    }
}


pub fn parse_command(
    command: String,
    board: &mut Board,
    transposition_table_arc_mutex: Arc<Mutex<TranspositionTable<Board>>>
) {
    // https://page.mi.fu-berlin.de/block/uci.htm

    // trim whitespaces, tabs, linebreaks, ... from both sides
    let command = command.trim();

    if command.starts_with("uci") {
        /*
        1. identify via "id" command
        2. reveal changeable settings via "option" command
        3. "uciok"
        */

        // 1.
        id();

        // 2.
        option();

        // 3.
        uciok();
    }

    if command.starts_with("debug") {
        todo!();
    }

    if command.starts_with("isready") {
        /*
        1. TODO: Complete set up
        2. "readyok"
        */

        // 1.
        // TODO

        // 2.
        readyok();
    }

    if command.starts_with("setoption") {
        todo!();
    }

    if command.starts_with("position") {
        let mut content = command.strip_prefix("position").unwrap();
        content = remove_whitespace_prefix(content);

        // check for "fen" or "startpos"
        if content.starts_with("fen") {
            content = content.strip_prefix("fen").unwrap();
            content = remove_whitespace_prefix(content);

            // extract FEN, remove from content and set up on board
            let fen = match content.find("moves") {
                None => content,
                Some(i) => &content[..i]
            };
            content = content.strip_prefix(fen).unwrap();
            content = remove_whitespace_prefix(content);
            *board = Board::from_fen(fen);

        } else if content.starts_with("startpos") {
            content = content.strip_prefix("startpos").unwrap();
            content = remove_whitespace_prefix(content);

            // set up default board
            *board = Board::default();

        } else {
            panic!("Invalid \"position\" command!")
        }

        // parse given moves (if any)
        if content.starts_with("moves") {
            content = content.strip_prefix("moves").unwrap();
            content = remove_whitespace_prefix(content);

            // parse moves
            for r#move_str in content.split_whitespace() {
                let r#move = Move::from_algebraic(r#move_str, board);
                board.make_move(r#move);
            }
        }

        // visualize
        println!("Position now is:");
        board.visualize();
        println!();
    }

    if command.starts_with("go") {'block: {
        let mut content = command.strip_prefix("go").unwrap();
        content = remove_whitespace_prefix(content);

        let mut go_info = GoInfo::<Board>::default();
        go_info.whites_turn = board.whites_turn;

        while content.len() > 0 {
            // extract subcommand
            let subcommand = prefix_until_whitespace(content);
            content = content.strip_prefix(subcommand).unwrap();
            content = remove_whitespace_prefix(content);

            // parse subcommand
            match subcommand {
                "wtime"|"btime"|"winc"|"binc"|"movestogo"|"depth"|"movetime" => {

                    // parse given number
                    let number_str = prefix_until_whitespace(content);
                    content = content.strip_prefix(number_str).unwrap();
                    content = remove_whitespace_prefix(content);
                    let number = number_str.parse::<usize>().expect(
                        &format!("Invalid number after \"{}\"", subcommand)
                    );

                    // register in info
                    match subcommand {
                        "wtime"     => {go_info.wtime     = number; go_info.wtime_given     = true},
                        "btime"     => {go_info.btime     = number; go_info.btime_given     = true},
                        "winc"      => {go_info.winc      = number; go_info.winc_given      = true},
                        "binc"      => {go_info.binc      = number; go_info.binc_given      = true},
                        "movestogo" => {go_info.movestogo = number; go_info.movestogo_given = true},
                        "depth"     => {go_info.depth     = number; go_info.depth_given     = true},
                        "movetime"  => {go_info.movetime  = number; go_info.movetime_given  = true},
                        _ => unreachable!()
                    }
                },
                "infinite" => { go_info.infinite = true;}
                "ponder"|"nodes"|"mate"|"searchmoves" => {unimplemented!()},  // TODO
                _ => {
                    println!("Unknown subcommand \"{}\" of \"go\" command!", subcommand);
                    break 'block;
                }
            }
        }

        // search
        let clone = board.clone();  // TODO: If I trust make/unmake this should be unnecessary
        go_info.search(clone, transposition_table_arc_mutex);

    }}

    if command.starts_with("stop") {
        emit_stop();
        println!()
    }

    if command.starts_with("ponderhit") {
        todo!();
    }

    if command.starts_with("quit") {
        println!("\nQuitting...");
        std::process::exit(0);
    }
}


fn id() {
    print!(
        "id name {NAME}\n\
        id author {AUTHOR}\n
        \n"
    );
}


fn uciok() {
    print!("uciok\n\n");
}


fn readyok() {
    print!("readyok\n\n");
}


pub fn bestmove<Move: SearchableMove>(r#move: Move, ponder: Option<Move>) {
    print!("bestmove {}", r#move.to_string());
    match ponder {
        None => print!("\n"),
        Some(ponder) => print!(" ponder {}", ponder.to_string())
    };
    print!("\n");
}


fn option() {
    // TODO
}


pub fn uci_loop() {

    let transposition_table: TranspositionTable<Board> = TranspositionTable::new();
    let tt_arc_mutex = Arc::new(Mutex::new(transposition_table));

    let mut board: Board = Board::default();

    loop {
        let mut command: String = String::new();
        std::io::stdin().read_line(&mut command).expect("Line parsing panic-ed!");
        parse_command(command, &mut board, tt_arc_mutex.clone());
    }
}
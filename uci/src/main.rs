
/*
TODO:
    - move perft to separate module
    - transposition table via pointers? I don't like the TT in the search info and all of the lifetime stuff
    - search doesn't return mate
*/

use crate::parsing::uci_loop;

mod parsing;
mod go;


fn main() {
    uci_loop();
}
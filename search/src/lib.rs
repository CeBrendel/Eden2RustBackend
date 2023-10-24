pub mod alpha_beta_search;
mod quiescence_search;
mod mcts;


/*
TODO:
    - MVV/LVA
    - does evaluation need to be flipped?
    - transposition tables (separate for quiescence and alpha-beta?)
    - remove mut from references where unnecessary (legal & loud moves)
*/

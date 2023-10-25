
pub struct True;
pub struct False;

pub trait Bool {
    type Not: Bool;
    const AS_BOOL: bool;
}
impl Bool for True {
    type Not = False;
    const AS_BOOL: bool = false;
}

impl Bool for False {
    type Not = True;
    const AS_BOOL: bool = true;
}

type Move = u32;
pub struct GenericBoard<HasEP: Bool>{
    phantom: core::marker::PhantomData<HasEP>,
    data: u64,
    data2: u64
}
pub enum Board {
    EP0(GenericBoard<False>),
    EP1(GenericBoard<True >),
}

type Generics = (bool,);

fn transmute_and_tag<HasEP: Bool>(generic_board: GenericBoard<HasEP>, new_generics: Generics) -> Board {
    match new_generics {
        (false,) => Board::EP0(unsafe{std::mem::transmute(generic_board)}),
        (true, ) => Board::EP1(unsafe{std::mem::transmute(generic_board)})
    }
}

fn make_move_inner<HasEP: Bool>(mut board: GenericBoard<HasEP>, r#move: Move) -> Board {
    // this will compile into two different variants each with different code blocks. So: The if check is no longer part of the blocks after compilation! So not two separate checks (notice how HasEP come sup multiple times).

    // maybe expensive
    let new_generics = if HasEP::AS_BOOL {
        // 5 minute calculation
        board.data = 0;
        board.data2 = 1;
        (false,)
    } else {
        // 1 second calculation
        board.data = 10345;
        board.data2 = 11111;
        (true,)
    };

    transmute_and_tag(board, new_generics)
}


pub fn make_move(board: Board, r#move: Move) -> Board {
    match board {
        Board::EP0(inner) => make_move_inner(inner, r#move),
        Board::EP1(inner) => make_move_inner(inner, r#move),
    }
}
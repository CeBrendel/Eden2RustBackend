
pub struct True;
pub struct False;

pub trait Bool {
    type Not: Bool;
    const AS_BOOL: bool;
}
impl Bool for True {
    type Not = False;
    const AS_BOOL: bool = true;
}

impl Bool for False {
    type Not = True;
    const AS_BOOL: bool = false;
}


/*#![feature(type_name_of_val)]
use std::any::type_name_of_val;*/

pub struct True;
pub struct False;

pub trait Bool/*: private::Sealed*/ {
    type Not: Bool;
    const AS_BOOL: bool;
}

/*mod private {
    pub trait Sealed {}
    impl Sealed for super::True {}
    impl Sealed for super::False {}
}*/

impl Bool for True {
    type Not = False;
    const AS_BOOL: bool = false;
}

impl Bool for False {
    type Not = True;
    const AS_BOOL: bool = true;
}

/*
TODO:
    - implement indexing? (via PEXT)
*/

pub mod squares;

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Not};
use bytemuck::{Pod, Zeroable};

use crate::squares::Square;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Pod, Zeroable)]
#[repr(transparent)]
pub struct Bitboard(pub u64);

impl Bitboard {

    const FLIP_MASK_1: u64 = 0x00FF00FF00FF00FF;
    const FLIP_MASK_2: u64 = 0x0000FFFF0000FFFF;

    pub const fn flip(self: Self) -> Self {
        // flip bitboard vertically
        // for implementation details, see: https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating#FlipVertically

        let mut x = self.0;

        x = ((x >>  8) & Self::FLIP_MASK_1) | ((x & Self::FLIP_MASK_1) <<  8);
        x = ((x >> 16) & Self::FLIP_MASK_2) | ((x & Self::FLIP_MASK_2) << 16);
        x = ( x >> 32)                      | ( x       << 32);

        return Self(x);
    }

    /*#[inline(always)]
    pub const fn lsh(self, shift: usize) -> Bitboard {
        Bitboard(self.0 << shift)
    }

    #[inline(always)]
    pub const fn rsh(self, shift: usize) -> Bitboard {
        Bitboard(self.0 >> shift)
    }*/

    #[inline(always)]
    pub const fn and(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 & rhs.0)
    }

    /*#[inline(always)]
    pub const fn or(self, rhs: Bitboard) -> Bitboard {
        Bitboard(self.0 | rhs.0)
    }*/

    pub fn set_bit(&mut self, square: Square) {
        //set a bit at the given square of the bitboard
        self.0 |= 1u64 << square as usize;
    }

    pub fn clear_bit(&mut self, square: Square) {
        //clear a bit at the given square of the bitboard
        self.0 &= !(1u64 << square as usize);
    }

    #[inline(always)]
    pub fn count_ones(self) -> usize {
        self.0.count_ones() as usize
    }

    #[inline(always)]
    pub const fn tzcnt(self) -> usize {
        // count trailing zeros
        self.0.trailing_zeros() as usize  // TODO: Is this the right CPU instruction?
    }

    #[inline(always)]
    pub const fn blsr(self) -> Bitboard {
        // replace least set bit by 0
        //Bitboard(unsafe { core::arch::x86_64::_blsr_u64(self.0) })  // TODO: This is not const but definitely the right CPU instruction
        Bitboard(self.0 & self.0.wrapping_sub(1))  // TODO: This is const, but maybe not the right CPU instruction
    }

    #[inline(always)]
    pub const fn has_entry_at(&self, square: Square) -> bool {
        // check if bitboard is 1 at given index
        (self.0 & (1 << square as usize)) != 0
    }

    #[inline(always)]
    pub const fn has_bits(self) -> bool {
        // checks if bitboard is empty
        self.0 != 0
    }

    #[inline(always)]
    pub const fn not(self) -> Bitboard {
        // flip bits
        Bitboard(!self.0)
    }

    pub const fn const_pext(self, mask: Bitboard) -> usize {
        // emulates the PEXT instruction for bitboards (to maintain const-ness)
        let mut k = 0;
        let mut m = 0;
        let mut out = 0;
        while m < 64 {
            if (mask.0 & (1 << m)) != 0 {
                if (self.0 & (1 << m)) != 0 {
                    out |= 1 << k;
                } else {
                    out &= !(1 << k);
                }
                k += 1;
            }
            m += 1;
        }
        out
    }

    pub const fn const_pdep(occupancy: u64, mask: Bitboard) -> Bitboard {
        // emulates the PDEP instruction for bitboards (to maintain const-ness)

        let mut dest: u64 = 0;
        let mut m: u8 = 0;
        let mut k: u8 = 0;
        while m < 64 {
            if mask.has_entry_at(Square::from_repr(m)) {
                dest |= (1 & (occupancy >> k)) << m;
                k += 1;
            }
            m += 1;
        }

        Bitboard(dest)
    }

    #[inline(always)]
    pub fn pext(self, mask: Bitboard) -> usize {
        // gathers the bits specified by the mask into a contiguous lowest order chunk of the result
        unsafe {
            core::arch::x86_64::_pext_u64(self.0, mask.0) as usize
        }
    }

    /*#[inline(always)]
    fn unsafe_pdep(src: u64, mask: u64) -> Bitboard {
        // deposits the contiguous lower bits of src into result based on mask
        Bitboard(unsafe {
            core::arch::x86_64::_pdep_u64(src, mask)
        })
    }*/

    #[inline(always)]
    pub const fn home_rank(for_white: bool) -> Bitboard {
        if for_white {Bitboard(0xFF00)} else {Bitboard(0xFF00).flip()}
    }

    #[inline(always)]
    pub const fn not_left_file(for_white: bool) -> Bitboard {
        if for_white {Bitboard(!0x0101010101010101)} else {Bitboard(!0x8080808080808080)}
    }

    #[inline(always)]
    pub const fn not_right_file(for_white: bool) -> Bitboard {
        if for_white {Bitboard(!0x8080808080808080)} else {Bitboard(!0x0101010101010101)}
    }

    #[inline(always)]
    pub const fn shift_backwards(self: Self, for_white: bool) -> Bitboard {
        if for_white {Bitboard(self.0 >> 8)} else {Bitboard(self.0 << 8)}
    }

    #[inline(always)]
    pub const fn shift_backwards_twice(self: Self, for_white: bool) -> Bitboard {
        if for_white {Bitboard(self.0 >> 16)} else {Bitboard(self.0 << 16)}
    }

    #[inline(always)]
    pub const fn shift_left_pawn_attack(self: Self, for_white: bool) -> Bitboard {
        if for_white {Bitboard(self.0 << 7)} else {Bitboard(self.0 >> 7)}
    }

    #[inline(always)]
    pub const fn shift_right_pawn_attack(self: Self, for_white: bool) -> Bitboard {
        if for_white {Bitboard(self.0 << 9)} else {Bitboard(self.0 >> 9)}
    }

    #[inline(always)]
    pub const fn shift_left(self: Self, for_white: bool) -> Bitboard {
        if for_white {Bitboard(self.0 >> 1)} else {Bitboard(self.0 << 1)}
    }

    #[inline(always)]
    pub const fn shift_right(self: Self, for_white: bool) -> Bitboard {
        if for_white {Bitboard(self.0 << 1)} else {Bitboard(self.0 >> 1)}
    }

    pub fn visualize(&self) -> () {
        // visualize the bitboard on console
        for rank in (0..8).rev() {
            for file in 0..8 {
                if (1u64 << (file + 8 * rank) & self.0) != 0u64 {
                    print!("1 ");
                } else {
                    print!("- ");
                }
            }
            print!("\n");
        }
        print!("\n");
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self: Self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

/*impl Shr<usize> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl Shl<usize> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}*/

#[macro_export]
macro_rules! bitloop {
    ( $bb:expr , $var:ident => $body:expr ) => {
        let mut bb = $bb;
        while bb.has_bits() {
            let $var: u8 = bb.tzcnt() as u8;  // TODO: Maybe of type usize?

            $body

            bb = bb.blsr();
        }
    }
}
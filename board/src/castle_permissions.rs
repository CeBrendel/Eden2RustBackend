
#[derive(Clone, Copy)]
#[derive(PartialEq)]  // TODO: Unnecessary
pub struct CastlePermissions(u8);

impl Default for CastlePermissions {
    fn default() -> Self {
        Self(0xF)
    }
}

impl CastlePermissions {
    const WHITE_MASK: u8 = 0b0011;
    const BLACK_MASK: u8 = 0b1100;

    const WHITE_SHORT_MASK: u8 = 0b0001;
    const WHITE_LONG_MASK: u8 = 0b0010;
    const BLACK_SHORT_MASK: u8 = 0b0100;
    const BLACK_LONG_MASK: u8 = 0b1000;

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn new(white_short: bool, white_long: bool, black_short: bool, black_long: bool) -> Self {
        let mut rights: u8 = 0;
        if white_short {rights |= Self::WHITE_SHORT_MASK}
        if white_long {rights |= Self::WHITE_LONG_MASK}
        if black_short {rights |= Self::BLACK_SHORT_MASK}
        if black_long {rights |= Self::BLACK_LONG_MASK}
        Self(rights)
    }

    #[inline(always)]
    pub fn flip(self: Self) -> Self {
        return Self(((self.0 & Self::WHITE_MASK) << 2) | ((self.0 & Self::BLACK_MASK) >> 2))
    }

    #[inline(always)]
    pub fn has_white_short(self: &Self) -> bool {
        return (self.0 & Self::WHITE_SHORT_MASK) != 0;
    }

    #[inline(always)]
    pub fn has_white_long(self: &Self) -> bool {
        return (self.0 & Self::WHITE_LONG_MASK) != 0;
    }

    #[inline(always)]
    pub fn has_black_short(self: &Self) -> bool {
        return (self.0 & Self::BLACK_SHORT_MASK) != 0;
    }

    #[inline(always)]
    pub fn has_black_long(self: &Self) -> bool {
        return (self.0 & Self::BLACK_LONG_MASK) != 0;
    }

    #[inline(always)]
    pub fn remove_rights(self: &mut Self, from_white: bool) {
        self.0 &= if from_white {Self::BLACK_MASK} else {Self::WHITE_MASK};
    }

    #[inline(always)]
    pub fn remove_short_rights(self: &mut Self, from_white: bool) {
        self.0 &= if from_white {!Self::WHITE_SHORT_MASK} else {!Self::BLACK_SHORT_MASK};
    }

    #[inline(always)]
    pub fn remove_long_rights(self: &mut Self, from_white: bool) {
        self.0 &= if from_white {!Self::WHITE_LONG_MASK} else {!Self::BLACK_LONG_MASK};
    }

    pub fn visualize(self: &Self) {
        print!("{}{}{}{}",
               if self.has_white_short() {'K'} else {'-'},
               if self.has_white_long() {'Q'} else {'-'},
               if self.has_black_short() {'k'} else {'-'},
               if self.has_black_long() {'q'} else {'-'}
        )
    }
}
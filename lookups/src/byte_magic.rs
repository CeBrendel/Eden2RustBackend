
macro_rules! include_lookup_1d {
    /*
    Loads bytes from the given file and transmutes then into an [Bitboard; N] for the given N
     */
    ($filename:literal, $N:literal) => {{

        // load bytes from file
        let bytes: &[u8; $N * 8] = include_bytes!(concat!(env!("OUT_DIR"), "/", $filename));


        // transmute to target type
        /*
        SAFETY:
        Bitboard is a #[repr(transparent)] wrapper around u64.
        RESTRICTIONS:
        This entire program is also specific to x86_64, and therefore to little-endian byte order.
         */
        let lookup: [Bitboard; $N] = unsafe { std::mem::transmute(*bytes) };

        lookup
    }};
}

macro_rules! include_lookup_2d {
    /*
    Loads bytes from the given file and transmutes then into an [[Bitboard; N]; M] for given
    axis lengths N and M
     */
    ($filename:literal, $N:literal, $M:literal) => {{

        // load bytes from file
        let bytes: &[u8; $N * $M * 8] = include_bytes!(concat!(env!("OUT_DIR"), "/", $filename));


        // transmute to target type
        /*
        SAFETY:
        Bitboard is a #[repr(transparent)] wrapper around u64.
        RESTRICTIONS:
        This entire program is also specific to x86_64, and therefore to little-endian byte order.
         */
        let lookup: [[Bitboard; $N]; $M] = unsafe { std::mem::transmute(*bytes) };

        lookup
    }};
}

pub(crate) use include_lookup_1d;
pub(crate) use include_lookup_2d;
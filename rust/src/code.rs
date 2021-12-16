pub const CODE_BITS: usize = 16;
pub const CODE_MAX: usize = (1 << 16) - 1;
pub const CODE_FIRST_QTR: usize = CODE_MAX / 4 + 1;
pub const CODE_HALF: usize = 2 * CODE_FIRST_QTR;
pub const CODE_THIRD_QTR: usize = 3 * CODE_FIRST_QTR;

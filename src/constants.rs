pub const STATES: i16 = 16;
pub const TARGET: [i16; STATES as usize] = [0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0]; //[3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3];
pub const DEBUG: u16 = 0; // 0, 1, 2, 3 are usable values currently
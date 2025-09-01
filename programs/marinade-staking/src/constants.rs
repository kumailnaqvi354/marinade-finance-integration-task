use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

pub const MARINADE_PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    5, 69, 227, 101, 190, 242, 113, 173, 117, 53, 3, 103, 86, 93, 164, 13, 163, 54, 220, 28, 135,
    155, 177, 84, 138, 122, 252, 197, 90, 169, 57, 30,
]);
pub const ANCHOR_DISCRIMINATOR: usize = 8;

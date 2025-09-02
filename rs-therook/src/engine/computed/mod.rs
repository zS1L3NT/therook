mod attack_masks;
mod single_rank_attack;

use attack_masks::*;
use single_rank_attack::*;
use therook::timed;

use super::*;

pub struct Computed {
    pub single_rank_attack: SingleRankAttack,
    pub attack_masks: AttackMasks,
}

impl Computed {
    #[timed(Computed)]
    pub fn new() -> Self {
        Computed {
            single_rank_attack: SingleRankAttack::new(),
            attack_masks: AttackMasks::new(),
        }
    }
}

mod attack_masks;

use super::*;

use attack_masks::*;

pub struct Computed {
    pub attack_masks: AttackMasks,
}

impl Computed {
    pub fn new() -> Self {
        timed!("computed statics", {
            Computed {
                attack_masks: AttackMasks::new(),
            }
        })
    }
}

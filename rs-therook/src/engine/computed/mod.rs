mod attack_masks;
mod line_masks;

use super::*;

use attack_masks::*;
use line_masks::*;

pub struct Computed {
    pub attack_masks: AttackMasks,
    pub line_masks: LineMasks,
}

impl Computed {
    pub fn new() -> Self {
        timed!("computed statics", {
            Computed {
                attack_masks: AttackMasks::new(),
                line_masks: LineMasks::new(),
            }
        })
    }
}

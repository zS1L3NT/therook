mod attack_masks;
mod line_masks;
mod obstruction_masks;
mod _xray_attacks;

use super::*;

use attack_masks::*;
use line_masks::*;
use obstruction_masks::*;

pub struct Computed {
    pub attack_masks: AttackMasks,
    pub line_masks: LineMasks,
    pub obstruction_masks: ObstructionMasks,
}

impl Computed {
    pub fn new() -> Self {
        timed!("computed statics", {
            Computed {
                attack_masks: AttackMasks::new(),
                line_masks: LineMasks::new(),
                obstruction_masks: ObstructionMasks::new(),
            }
        })
    }
}

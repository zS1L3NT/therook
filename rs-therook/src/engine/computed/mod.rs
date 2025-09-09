mod _xray_attacks;
mod attacks;
mod betweens;
mod rays;

use super::*;

use attacks::*;
use betweens::*;
use rays::*;

pub struct Computed {
    pub rays: Rays,
    pub attacks: Attacks,
    pub betweens: Betweens,
}

impl Computed {
    pub fn new() -> Self {
        timed!("computed statics", {
            Computed {
                rays: Rays::new(),
                attacks: Attacks::new(),
                betweens: Betweens::new(),
            }
        })
    }
}

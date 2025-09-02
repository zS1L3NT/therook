use std::num::Wrapping;
use therook::timed;

pub struct SingleRankAttack {
    data: [[u8; 8]; 256],
}

impl SingleRankAttack {
    #[timed(SingleRankAttack)]
    pub fn new() -> Self {
        // https://www.chessprogramming.org/Efficient_Generation_of_Sliding_Piece_Attacks#Lookup_Techniques
        let mut data = [[0u8; 8]; 256];

        for occupancy in 0..=255u8 {
            for slider in 0..8u8 {
                if occupancy & (1 << slider) != 0 {
                    let o = Wrapping(occupancy);
                    let s = Wrapping(1 << slider);
                    let _2 = Wrapping(2);

                    // https://www.chessprogramming.org/Efficient_Generation_of_Sliding_Piece_Attacks#Sliding_Attacks_by_Calculation
                    data[occupancy as usize][slider as usize] |=
                        ((o - _2 * s) ^ Self::reverse(Self::reverse(o) - _2 * Self::reverse(s))).0;
                }
            }
        }

        SingleRankAttack { data: data }
    }

    pub fn get(&self, occupancy: u8, slider: u8) -> u8 {
        if occupancy & slider == 0 {
            panic!("Cannot find the rank attack when slider is not in occupancy!")
        }

        self.data[occupancy as usize][slider.trailing_zeros() as usize]
    }

    fn reverse(wrapping: Wrapping<u8>) -> Wrapping<u8> {
        wrapping.reverse_bits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn examples() {
        let computed = SingleRankAttack::new();

        assert_eq!(0b01111011u8, computed.get(0b11000101u8, 0b00000100u8));
        assert_eq!(0b01110110u8, computed.get(0b01001010u8, 0b00001000u8));
    }
}

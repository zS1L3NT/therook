use super::*;

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let now = std::time::Instant::now();

        let mut lines = String::new();

        let mut chars: [char; 64] = [' '; 64];

        for piece in Piece::ALL {
            let bitboard = self.bitboards[piece];
            for tile in bitboard.get_tiles() {
                chars[Into::<u8>::into(tile) as usize] = piece.into();
            }
        }

        lines.push_str(&"  ╔═══╦═══╦═══╦═══╦═══╦═══╦═══╦═══╗\n");
        for rank in (0..8).rev() {
            lines.push_str(&format!(
                "{} ║{}\n",
                format!("{}", vec![1, 2, 3, 4, 5, 6, 7, 8][rank]),
                (rank * 8..rank * 8 + 8)
                    .map(|i| chars[i])
                    .fold(String::new(), |acc, el| format!("{acc} {el} ║")),
            ));

            if rank != 0 {
                lines.push_str(&"  ╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣\n");
            }
        }
        lines.push_str(&"  ╚═══╩═══╩═══╩═══╩═══╩═══╩═══╩═══╝\n");
        lines.push_str(&"    A   B   C   D   E   F   G   H  \n");

        println!(
            "Stringified board in {} nanoseconds",
            now.elapsed().as_nanos()
        );

        write!(f, "{}", lines)
    }
}

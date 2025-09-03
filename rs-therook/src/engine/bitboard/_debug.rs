use super::*;
use colored::Colorize;

impl std::fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        timed!("bitboard stringified", {
            let mut lines = String::new();

            lines.push_str(&format!("Bitboard: {}\n", Into::<u64>::into(*self)));
            lines.push_str(&"  ╔═══╦═══╦═══╦═══╦═══╦═══╦═══╦═══╗\n");
            for rank in (0..8).rev() {
                lines.push_str(&format!(
                    "{} ║{}\n",
                    format!("{}", vec![1, 2, 3, 4, 5, 6, 7, 8][rank]),
                    (rank * 8..rank * 8 + 8)
                        .map(|i| if self.0 & 1 << i != 0 {
                            String::from("1").on_bright_white()
                        } else {
                            String::from(" ").white()
                        })
                        .fold(String::new(), |acc, el| format!("{acc} {el} ║")),
                ));

                if rank != 0 {
                    lines.push_str(&"  ╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣\n");
                }
            }
            lines.push_str(&"  ╚═══╩═══╩═══╩═══╩═══╩═══╩═══╩═══╝\n");
            lines.push_str(&"    A   B   C   D   E   F   G   H  \n");

            write!(f, "{}", lines)
        })
    }
}

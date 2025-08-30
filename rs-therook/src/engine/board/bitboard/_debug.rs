use super::*;
use colored::Colorize;

impl std::fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let now = std::time::Instant::now();

        let mut lines = String::new();

        lines.push_str(&"  ╔═══╦═══╦═══╦═══╦═══╦═══╦═══╦═══╗\n");
        for rank in (0..8).rev() {
            lines.push_str(&format!(
                "{} ║{}\n",
                format!("{}", vec![1, 2, 3, 4, 5, 6, 7, 8][rank]),
                (rank * 8..rank * 8 + 8)
                    .map(|i| if self.0 & 1 << i != 0 {
                        String::from(" ").on_bright_white()
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

        println!(
            "Stringified bitboard in {} nanoseconds",
            now.elapsed().as_nanos()
        );

        write!(f, "{}", lines)
    }
}

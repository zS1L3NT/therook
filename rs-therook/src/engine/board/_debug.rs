use therook::timed;

use super::*;

impl std::fmt::Debug for Board {
    #[timed(Board)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = String::new();

        lines.push_str(&format!("Board: {}\n", "<FEN>"));
        lines.push_str(&"  ╔═══╦═══╦═══╦═══╦═══╦═══╦═══╦═══╗\n");
        for rank in (0..8).rev() {
            lines.push_str(&format!(
                "{} ║{}\n",
                format!("{}", vec![1, 2, 3, 4, 5, 6, 7, 8][rank]),
                (rank * 8..rank * 8 + 8)
                    .map(|i| self.squares[i].map(|p| p.into()).unwrap_or(' '))
                    .fold(String::new(), |acc, el| format!("{acc} {el} ║")),
            ));

            if rank != 0 {
                lines.push_str(&"  ╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣\n");
            }
        }
        lines.push_str(&"  ╚═══╩═══╩═══╩═══╩═══╩═══╩═══╩═══╝\n");
        lines.push_str(&"    A   B   C   D   E   F   G   H  \n");

        write!(f, "{}", lines)
    }
}

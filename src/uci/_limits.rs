use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GoLimits {
    pub depth: Option<u8>,
    pub movetime: Option<u64>,
    pub infinite: bool,
    pub wtime: Option<u64>,
    pub btime: Option<u64>,
    pub winc: u64,
    pub binc: u64,
    pub movestogo: Option<u64>,
}

pub fn parse_go_limits(tokens: &[&str]) -> GoLimits {
    let mut limits = GoLimits::default();
    let mut index = 0;
    while index < tokens.len() {
        match tokens[index] {
            "depth" if index + 1 < tokens.len() => {
                limits.depth = tokens[index + 1].parse::<u8>().ok();
                index += 2;
            }
            "movetime" if index + 1 < tokens.len() => {
                limits.movetime = tokens[index + 1].parse::<u64>().ok();
                index += 2;
            }
            "wtime" if index + 1 < tokens.len() => {
                limits.wtime = tokens[index + 1].parse::<u64>().ok();
                index += 2;
            }
            "btime" if index + 1 < tokens.len() => {
                limits.btime = tokens[index + 1].parse::<u64>().ok();
                index += 2;
            }
            "winc" if index + 1 < tokens.len() => {
                limits.winc = tokens[index + 1].parse::<u64>().unwrap_or(0);
                index += 2;
            }
            "binc" if index + 1 < tokens.len() => {
                limits.binc = tokens[index + 1].parse::<u64>().unwrap_or(0);
                index += 2;
            }
            "movestogo" if index + 1 < tokens.len() => {
                limits.movestogo = tokens[index + 1].parse::<u64>().ok().filter(|n| *n > 0);
                index += 2;
            }
            "infinite" => {
                limits.infinite = true;
                index += 1;
            }
            _ => index += 1,
        }
    }
    if limits.depth.is_some() || limits.movetime.is_some() {
        limits.infinite = false;
    }
    limits
}

pub fn allocate_clock_ms(time_ms: u64, increment_ms: u64, movestogo: Option<u64>) -> u64 {
    if time_ms == 0 {
        return 1;
    }
    let moves = movestogo.unwrap_or(30).max(1);
    let base = time_ms / moves;
    // Keep a reserve for later moves while allowing a modest increment bonus.
    base.saturating_add(increment_ms.min(time_ms / 10))
        .min(time_ms)
        .max(1)
}

pub fn clock_movetime(limits: GoLimits, turn: PieceColor) -> Option<u64> {
    if limits.movetime.is_some() || limits.depth.is_some() || limits.infinite {
        return limits.movetime;
    }
    let (time, increment) = match turn {
        PieceColor::White => (limits.wtime?, limits.winc),
        PieceColor::Black => (limits.btime?, limits.binc),
    };
    Some(allocate_clock_ms(time, increment, limits.movestogo))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn go_limits_choose_the_correct_clock_and_respect_priorities() {
        let limits = parse_go_limits(&[
            "wtime",
            "1200",
            "btime",
            "800",
            "winc",
            "100",
            "binc",
            "50",
            "movestogo",
            "10",
        ]);
        assert_eq!(clock_movetime(limits, PieceColor::White), Some(220));
        assert_eq!(clock_movetime(limits, PieceColor::Black), Some(130));

        let movetime = parse_go_limits(&["movetime", "17", "wtime", "999"]);
        assert_eq!(clock_movetime(movetime, PieceColor::White), Some(17));
        let depth = parse_go_limits(&["depth", "3", "btime", "999"]);
        assert_eq!(clock_movetime(depth, PieceColor::Black), None);
        assert!(parse_go_limits(&["infinite"]).infinite);
    }
}

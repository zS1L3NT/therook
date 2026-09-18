use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct Stockfish {
    process: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Stockfish {
    pub fn new() -> Self {
        let mut process = Command::new("../stockfish/stockfish")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn stockfish");
        let stdin = process.stdin.take().expect("Failed to open stdin");
        let stdout = BufReader::new(process.stdout.take().expect("Failed to open stdin"));

        let mut stockfish = Stockfish {
            process,
            stdin,
            stdout,
        };

        stockfish.write("uci".into());
        stockfish.read_until("uciok".into());

        stockfish
    }

    pub fn write(&mut self, text: String) {
        writeln!(self.stdin, "{}", text).expect("Failed to write to stdin");
    }

    pub fn read_until(&mut self, text: String) -> Vec<String> {
        let mut lines = vec![];

        for line in self.stdout.by_ref().lines() {
            let line = line.unwrap();
            if line.contains(&text) {
                break;
            }

            lines.push(line);
        }

        lines
    }
}

impl Drop for Stockfish {
    fn drop(&mut self) {
        self.process.kill().expect("Failed to kill stockfish");
    }
}

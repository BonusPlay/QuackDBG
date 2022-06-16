use std::process::{Command, Stdio, Child};
use color_eyre::eyre::Result;
use crossbeam::channel::{Sender, Receiver, unbounded};

pub struct GDB {
    proc: Child,
    stdin_tx: Sender<String>,
    stdin_rx: Receiver<String>,
    stdout_tx: Sender<String>,
    stdout_rx: Receiver<String>,
}

impl GDB {
    pub fn new() -> Result<Self> {
        let proc = Command::new("gdb")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;

        let (stdin_tx, stdin_rx) = unbounded();
        let (stdout_tx, stdout_rx) = unbounded();

        Ok(Self {
            proc,
            stdin_tx,
            stdin_rx,
            stdout_tx,
            stdout_rx,
        })
    }

    pub fn add_listener(&self) -> Receiver<String> {
        self.stdout_rx.clone()
    }

    pub fn add_transmitter(&self) -> Sender<String> {
        self.stdin_tx.clone()
    }

    pub fn start(&mut self) {
        let stdin = self.proc.stdin.take().unwrap();
        let stdout = self.proc.stdout.take().unwrap();
        let tx = self.stdout_tx.clone();
        let rx = self.stdin_rx.clone();

        // read from stdout
        std::thread::spawn(move || {
            use std::io::{BufRead, BufReader};
            let reader = BufReader::new(stdout);
            reader
                .lines()
                .filter_map(|line| line.ok())
                .for_each(|line| tx.send(line).unwrap());
        });

        // write to stdin
        std::thread::spawn(move || {
            // for some reason using BufWriter doesn't work here
            use std::io::{Write, BufWriter};
            let mut writer = BufWriter::new(stdin);
            for mut line in rx.iter() {
                line.push('\n');
                writer.write(line.as_bytes()).unwrap();
                writer.flush().unwrap();
            }
        });
    }
}

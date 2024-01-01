use actix::prelude::*;
use std::{
    io::{BufRead, BufReader, BufWriter, Error, Write},
    process::{ChildStdin, ChildStdout},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};

pub struct ServerHandler {
    pub stdin: Arc<Mutex<Option<BufWriter<ChildStdin>>>>,
    pub stdout: Arc<Mutex<Option<BufReader<ChildStdout>>>>,
}

impl ServerHandler {
    pub fn new() -> ServerHandler {
        let mut command = Command::new("java")
            .arg("-Xmx1024M")
            .arg("-Xms1024M")
            .arg("-jar")
            .arg("server.jar")
            .arg("nogui")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn process");

        let stdout = Arc::new(Mutex::new(Some(BufReader::new(
            command.stdout.take().unwrap(),
        ))));

        let stdin = Arc::new(Mutex::new(Some(BufWriter::new(
            command.stdin.take().unwrap(),
        ))));

        ServerHandler { stdin, stdout }
    }

    pub fn stop_server(&mut self) -> Result<&str, Error> {
        {
            let mut stdin_opt = self.stdin.lock().unwrap();

            if let Some(ref mut stdin) = *stdin_opt {
                let write_res = stdin
                    .write_all("/stop\n".as_bytes())
                    .map_err(|e| e.to_string());

                stdin.flush();
            } else {
                return Ok("Looks like something went wrong because I can't find stdin");
            }
        }

        let _ = self
            .wait_for("[Server thread/INFO]: ThreadedAnvilChunkStorage: All dimensions are saved");

        Ok("Everything's ok!")
    }

    pub fn wait_for(&mut self, str: &str) -> Result<(), ()> {
        let mut stdout_lock = self.stdout.lock().unwrap(); // Acquire the lock

        if let Some(ref mut stdout) = *stdout_lock {
            for line in stdout.lines() {
                let line = line.unwrap();
                println!("{}", line);

                if line.contains(str) {
                    return Ok(());
                }
            }

            return Ok(());
        } else {
            return Err(());
        }
    }
}

impl Actor for ServerHandler {
    type Context = Context<Self>;
}

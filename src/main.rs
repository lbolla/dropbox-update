// 1. start
// 2. poll until update finished
// 3. stop

use std::{thread, time};
use std::process::{Command, Stdio};

struct Dropbox();

#[derive(Debug, PartialEq)]
enum DropboxStatus {
    NotRunning,
    UpToDate,
    Starting,
    Connecting,
    Else(String)
}

impl Dropbox {
    fn run(&self, cmd: &str) -> String {
        let child = Command::new("dropbox")
            .arg(cmd)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to get dropbox status");

        let output = child.wait_with_output()
            .expect("Failed to wait on child");
        assert!(output.status.success());
        String::from_utf8(output.stdout)
            .expect("Failed to decode output").trim().to_string()
    }

    fn status(&self) -> DropboxStatus {
        match self.run("status").as_str() {
            s if s == "Dropbox isn't running!" => DropboxStatus::NotRunning,
            s if s == "Connecting..." => DropboxStatus::Connecting,
            s if s == "Starting..." => DropboxStatus::Starting,
            s if s == "Up to date" => DropboxStatus::UpToDate,
            s => DropboxStatus::Else(s.to_string())
        }
    }

    fn is_running(&self) -> bool {
        self.status() != DropboxStatus::NotRunning
    }

    fn start(&self) {
        self.run("start");
        assert!(self.is_running())
    }

    fn stop(&self) {
        self.run("stop");
        assert!(!self.is_running())
    }

}

fn main() {
    let db = Dropbox();
    println!("Starting...");
    db.start();
    let mut status = db.status();
    while status != DropboxStatus::UpToDate {
        println!("Syncing... [{:?}]", status);
        let t = time::Duration::from_millis(1000);
        thread::sleep(t);
        status = db.status();
    }
    println!("Stopping...");
    db.stop();
    println!("Done.");
}

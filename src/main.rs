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
            .expect("Failed to decode output")
    }

    fn status(&self) -> DropboxStatus {
        match self.run("status") {
            ref s if *s == String::from("Dropbox isn't running!\n") => DropboxStatus::NotRunning,
            ref s if *s == String::from("Up to date\n") => DropboxStatus::UpToDate,
            ref s if *s == String::from("Starting\n") => DropboxStatus::Starting,
            s => DropboxStatus::Else(s)
        }
    }

    fn is_running(&self) -> bool {
        self.status() != DropboxStatus::NotRunning
    }

    fn start(&self) -> bool {
        self.run("start");
        self.is_running()
    }

    fn stop(&self) -> bool {
        self.run("stop");
        !self.is_running()
    }

}

fn main() {
    let db = Dropbox();
    println!("Starting...");
    db.start();
    while db.status() != DropboxStatus::UpToDate {
        println!("Syncing...");
        let t = time::Duration::from_millis(1000);
        thread::sleep(t);
    }
    println!("Stopping...");
    db.stop();
    println!("Done.");
}

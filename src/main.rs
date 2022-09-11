use std::process::Command;
fn main() {
    //pull -> Push -> commit local changes
    let _ = Command::new("git").args(["pull"]).output();
    let _ = Command::new("git").args(["add", "."]).output();
    let _ = Command::new("git").args(["commit", "-m \"updated this garbage\""]).output();
    let _ = Command::new("git").args(["push"]).output();

}

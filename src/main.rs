use regex::Regex;
use std::fs;
use std::fs::FileType;
use std::io;
use std::path::Path;
use std::process::Command;
fn main() {
    let dir = ".git";
    let path = Path::new(dir);
    if path.exists() {
        println!("You have a initialized git repository")
    } else {
        let _ = Command::new("git")
            .arg("init")
            .status()
            .expect("Failed to initialize Git repository");
    }
    fs::write(".git/config", "").expect("Error writing");
    let _ = Command::new("git")
        .arg("add")
        .arg(".")
        .status()
        .expect("Failed to add files to Git repository");
    fs::write(".git/config", "").expect("Error writing");
    let _ = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("Initial commit")
        .status()
        .expect("failed to commit files");
    fs::write(".git/config", "").expect("Error writing");
    let _ = Command::new("git")
        .arg("branch")
        .arg("-M")
        .arg("main")
        .status()
        .expect("failed to create branch");
    fs::write(".git/config", "").expect("Error writing");
    let mut link = String::new();
    let configread = fs::read_to_string(".git/config").expect("Error reading");
    if configread.contains("https://github") {
        println!("Link found");
        let re = Regex::new(r"https://.*.git").unwrap();

        if let Some(captures) = re.captures(&configread) {
            let cap = &captures[0];
            println!("Link is: {}", cap);
            link = cap.to_string();
        }
    } else {
        fs::write(".git/config", "").expect("Error writing");

        println!("I need you to enter the link for your project");
        io::stdin()
            .read_line(&mut link)
            .expect("Failed to read line");
        link = link.replace('\n', "");
    }
    fs::write(".git/config", "").expect("Error writing");
    let _ = Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(link.as_str())
        .status()
        .expect("Failed to commit files");

    let _ = Command::new("git")
        .arg("push")
        .arg("-u")
        .arg("origin")
        .arg("main");
    println!("Finished")
}

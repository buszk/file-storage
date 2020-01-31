use std::process::{Command, Child};
use file_diff::diff;
use std::fs::{File, remove_file};
use std::io::prelude::*;

fn execute(cmd: String) -> Child{
    let child = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C", cmd.as_str()])
                .spawn()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .spawn()
                .expect("failed to execute process")
    };

    child
}

#[test]
fn test_server() {
    /* Start server */
    let mut p = Command::new("target/debug/file-storage")
                        .spawn()
                        .expect("failed to start server");
    
    /* Upload */
    remove_file_no_throw(String::from("files/gi"));
    execute(String::from("curl -sS --upload-file .gitignore 127.0.0.1:8000/upload/gi")).wait().expect("curl failed");
    assert!(diff(".gitignore", "files/gi"));
    
    /* Download */
    remove_file_no_throw(String::from("files/foo.txt"));
    remove_file_no_throw(String::from("temp"));
    let mut file = File::create("files/foo.txt").unwrap();
    file.write_all(b"Hello, world!").unwrap();
    execute(String::from("curl -sS 127.0.0.1:8000/file/foo.txt -o temp")).wait().expect("Unable to crul");
    assert!(diff("temp", "files/foo.txt"));
    remove_file_no_throw(String::from("files/foo.txt"));
    remove_file_no_throw(String::from("temp"));

    /* Stop server */
    p.kill().expect("!kill");
}

fn remove_file_no_throw(path: String) {
    match remove_file(path) {
        Ok(_) => {},
        Err(_) => {},
    }
}


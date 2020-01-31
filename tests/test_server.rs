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
    let dir = String::from("files");
    let upload_fname = String::from ("gi");
    let fname = String::from(".gitignore");
    /* Upload */
    remove_file_no_throw(format!("{}/{}", dir, upload_fname));
    
    execute(format!("curl -sS --upload-file {} 127.0.0.1:8000/upload/{}", fname, upload_fname))
            .wait()
            .expect("curl upload failed");
    assert!(diff(fname.as_str(), format!("{}/{}", dir, upload_fname).as_str()));
    
    /* Download */
    let upload_fname = String::from("foo.txt");
    remove_file_no_throw(format!("{}/{}", dir, upload_fname));
    remove_file_no_throw(String::from("temp"));

    let mut file = File::create(format!("{}/{}", dir, upload_fname)).unwrap();
    file.write_all(b"Hello, world!").unwrap();
    execute(format!("curl -sS 127.0.0.1:8000/file/{} -o temp", upload_fname))
            .wait()
            .expect("crul download failed");
    assert!(diff("temp", "files/foo.txt"));

    remove_file_no_throw(format!("{}/{}", dir, upload_fname));
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


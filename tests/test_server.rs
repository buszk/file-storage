use file_diff::diff;
use std::fs::remove_file;
use std::process::{Child, Command};

fn execute(cmd: String) -> Child {
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
    let mut p = Command::new("target/debug/file-storage")
        .arg("--addr=127.0.0.1:8080")
        .spawn()
        .expect("failed to start server");
    /* Stop server */
    p.kill().expect("!kill");
}

#[test]
fn test_upload() {
    /* Start server */
    let mut p = Command::new("target/debug/file-storage")
        .arg("--addr=127.0.0.1:8085")
        .spawn()
        .expect("failed to start server");
    let dir = String::from("files");
    let upload_fname = String::from("gi");
    let fname = String::from(".gitignore");
    /* Upload */
    remove_file_no_throw(format!("{}/{}", dir, upload_fname));

    execute(format!(
        "curl -sS --upload-file {} 127.0.0.1:8085/upload/{}",
        fname, upload_fname
    ))
    .wait()
    .expect("curl upload failed");
    assert!(diff(
        fname.as_str(),
        format!("{}/{}", dir, upload_fname).as_str()
    ));
    remove_file(format!("{}/{}", dir, upload_fname)).unwrap();

    /* Stop server */
    p.kill().expect("!kill");
}

#[test]
fn test_download() {
    /* Start server */
    let mut p = Command::new("target/debug/file-storage")
        .arg("--addr=127.0.0.1:8082")
        .spawn()
        .expect("failed to start server");
    let dir = String::from("files");
    /* Download */
    let upload_fname = String::from("test");
    remove_file_no_throw(String::from("temp"));

    execute(format!(
        "curl -sS 127.0.0.1:8082/file/{} -o temp",
        upload_fname
    ))
    .wait()
    .expect("crul download failed");
    execute(String::from("curl 127.0.0.1:8082/file/test"))
        .wait()
        .expect("cat failed");
    assert!(diff("temp", "files/test"));

    remove_file(String::from("temp")).unwrap();

    /* Stop server */
    p.kill().expect("!kill");
}

fn remove_file_no_throw(path: String) {
    match remove_file(path) {
        Ok(_) => {}
        Err(_) => {}
    }
}

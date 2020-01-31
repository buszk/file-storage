// #![allow(warnings)]
use std::io::Write;
use std::env::current_dir;
use std::fs::{File, create_dir_all, OpenOptions, remove_file};
use warp::{Buf, Filter};
use std::thread;

fn create_file_safe(uri: String) -> Option<File> {

    println!("uri: {}", uri);

    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(uri) {
            Err(e) => {
                println!("{}", e);
                return None
            }
            Ok(f) => {
                return Some(f)
            },
        };

}

fn full(fname: String, mut body: impl Buf) -> String {

    let files_dir: String = format!("{}/{}", current_dir().unwrap().display(), "files");
    let uri: String = format!("{}/{}", files_dir, fname);

    let mut temp: File;
    match create_file_safe(uri) {
        None => {
            return String::from("File exists!\n")
        }
        Some(f) => {
            temp = f;
        }
    }

    while body.has_remaining() {
        let bs = body.bytes();
        let cnt = bs.len();
        match temp.write_all(bs) {
            Ok(_) => {},
            Err(_) => {
                return String::from("Upload failed!\n");
            }
        };
        body.advance(cnt);
        // println!("read {} bytes", cnt);
    }
    String::from("File uploaded!\n")
}

#[tokio::main]
async fn main() {

    /* Create files directory if not exists */
    let files_dir: String = format!("{}/{}", current_dir().unwrap().display(), "files");
    create_dir_all(files_dir.clone()).unwrap();

    /* File server for download */
    let file_server = warp::path("file")
        .and(warp::fs::dir(files_dir));

    /* Upload server for upload */
    let upload_server = warp::path!("upload" / String)
        .and(warp::put())
        .and(warp::body::aggregate())
        .map(full);
    
    /* Wrong request */
    let no_server = warp::any().map(|| "Not found!\n");
    let all = file_server.or(upload_server).or(no_server);
    
    /* Spin up the server */
    warp::serve(all)
        .run(([127, 0, 0, 1], 8000))
        .await;
        
}

#[cfg(test)]
mod tests {
    /* Import symbols from outter scope */
    use super::*;
    fn remove_file_no_throw(path: String) {
        match remove_file(path) {
            Ok(_) => {},
            Err(_) => {},
        }
    }
    #[test]
    fn test_create_file() {
        remove_file_no_throw(String::from(".test_create"));
        create_file_safe(String::from(".test_create")).unwrap();
        remove_file(String::from(".test_create")).unwrap();
    }
    #[test]
    fn test_create_multiple_files() {
        remove_file_no_throw(String::from(".test_create_multiple"));
        let handle = thread::spawn(move || {
            match create_file_safe(String::from(".test_create_multiple")) {
                Some(_) => 1,
                None => 0,
            }
        });
        let a = match create_file_safe(String::from(".test_create_multiple")) {
            Some(_) => 1,
            None => 0,
        };
        remove_file(String::from(".test_create_multiple")).unwrap();
        let b = handle.join().unwrap();
        assert_ne!(a, b);
    }
    
}
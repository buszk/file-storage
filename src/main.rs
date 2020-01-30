// #![allow(warnings)]
use std::io::Write;
use std::env::current_dir;
use std::fs::{File, create_dir_all};
use std::path::Path;
use warp::{Buf, Filter};

fn full(fname: String, mut body: impl Buf) -> String {

    let files_dir: String = format!("{}/{}", current_dir().unwrap().display(), "files");
    let uri: String = format!("{}/{}", files_dir, fname);

    println!("uri: {}", uri);
    if Path::new(&uri).exists() {
        return String::from("Failed! File already exists.\n")
    }

    let mut temp = File::create(uri).unwrap();

    while body.has_remaining() {
        let bs = body.bytes();
        let cnt = bs.len();
        temp.write_all(bs).unwrap();
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
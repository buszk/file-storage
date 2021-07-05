// #![allow(warnings)]
use std::io::Write;
use std::env::current_dir;
use std::fs::{File, create_dir_all, OpenOptions};
use std::str::FromStr;
use warp::{Buf, Filter};
use clap::{Arg, App};
use std::net::SocketAddr;

fn create_file_safe(uri: &str) -> Result<File, std::io::Error> {

    println!("uri: {}", uri);

    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(uri)
}

fn full(fname: String, mut body: impl Buf) -> String {

    let files_dir: String = format!("{}/{}", current_dir().unwrap().display(), "files");
    let uri: String = format!("{}/{}", files_dir, fname);

    let mut temp = match create_file_safe(&uri) {
        Err(err) => return format!("Failed to create file with uri {}: {}", &uri, err),
        Ok(f) => f,
    };
    while body.has_remaining() {
        let bs = body.chunk();
        let cnt = bs.len();
        if let Err(err) = temp.write_all(bs) {
            return format!("Upload {} failed! Cannot write: {}", fname, err);
        }
        body.advance(cnt);
        // println!("read {} bytes", cnt);
    }
    String::from("Upload succeeded!\n")
}

#[tokio::main]
async fn main() {
    
    let matches = App::new("file storage")
        .version("0.1.0")
        .author("Zekun Shen <bruceshenzk@gmail.com>")
        .about("temporary file storage server")
        .arg(Arg::with_name("addr")
                 .long("addr")
                 .takes_value(true)
                 .help("Your ip address and port to bind"))
        .get_matches();
    
    let addr_str = matches.value_of("addr").unwrap_or("127.0.0.1:8000");


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
    let server = warp::serve(all);
    server.run(SocketAddr::from_str(addr_str).unwrap()).await;
    
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::fs::remove_file;
    /* Import symbols from outter scope */
    use super::*;

    #[test]
    fn test_create_file() -> Result<()> {
        remove_file(".test_create").ok();
        create_file_safe(".test_create")?;
        remove_file(".test_create")?;
        Ok(())
    }
}
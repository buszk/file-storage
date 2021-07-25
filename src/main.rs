// #![allow(warnings)]
use anyhow::{self, Context};
use clap::{App, Arg};
use futures::stream::StreamExt; // for `next`
use futures::Stream;
use futures_util;
use std::env::current_dir;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::net::SocketAddr;
use std::str::FromStr;
use warp::{Buf, Filter};

fn create_file_safe(uri: &str) -> Result<File, std::io::Error> {
    println!("uri: {}", uri);

    OpenOptions::new().write(true).create_new(true).open(uri)
}

async fn upload<S, B>(fname: String, stream: S) -> Result<impl warp::Reply, warp::Rejection>
where
    S: Stream<Item = Result<B, warp::Error>>,
    B: Buf,
{
    match upload_impl(fname, stream).await {
        Ok(reply) => Ok(reply),
        Err(err) => Ok(format!("Error: {:#?}", err)),
    }
}

async fn upload_impl<S, B>(fname: String, stream: S) -> Result<String, anyhow::Error>
where
    S: Stream<Item = Result<B, warp::Error>>,
    B: Buf,
{
    let files_dir: String = format!("{}/{}", current_dir().unwrap().display(), "files");
    let uri: String = format!("{}/{}", files_dir, fname);

    let mut temp = create_file_safe(&uri)
        .with_context(|| format!("Failed to create file with uri {}", &uri))?;

    futures_util::pin_mut!(stream);
    while let Some(data) = stream.next().await {
        let mut data = data.with_context(|| "Cannot get data from stream".to_string())?;
        while data.has_remaining() {
            let bs = data.chunk();
            let len = bs.len();
            temp.write(bs)
                .with_context(|| format!("Cannot write to file {}", &uri))?;
            data.advance(len);
        }
    }
    Ok(String::from("Upload succeeded!\n"))
}

#[tokio::main]
async fn main() {
    let matches = App::new("file storage")
        .version("0.1.0")
        .author("Zekun Shen <bruceshenzk@gmail.com>")
        .about("temporary file storage server")
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .takes_value(true)
                .help("Your ip address and port to bind"),
        )
        .get_matches();

    let addr_str = matches.value_of("addr").unwrap_or("127.0.0.1:8000");

    /* Create files directory if not exists */
    let files_dir: String = format!("{}/{}", current_dir().unwrap().display(), "files");
    create_dir_all(files_dir.clone()).unwrap();

    /* File server for download */
    let file_server = warp::path("file").and(warp::fs::dir(files_dir));

    let upload_server = warp::path!("upload" / String)
        .and(warp::put())
        .and(warp::body::stream())
        .and_then(|f, s| upload(f, s));

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

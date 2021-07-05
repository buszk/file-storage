# filestore [![Build Status](https://travis-ci.org/buszk/file-storage.svg?branch=master)](https://travis-ci.org/buszk/file-storage)

This project is created under the inspiration of [transfer.sh](https://github.com/dutchcoders/transfer.sh/). It is meant to be a practice project for writing in Rust(https://www.rust-lang.org/). 

## Introduction
Filestore is a light weight, simple file upload and download back-end service. The aim is to provide easy-to-use file transfer service while using command line. Filestore handles http protocol through [warp](https://github.com/seanmonstar/warp) crate. In this way, one can simply use `curl` to upload and download files from a filestore server.

## Usage

The parameter `-sS` is to quite curl's output. 

### Upload
```
curl -sS --upload-file test 127.0.0.1:8000/upload/test
```
### Download
```
curl -sS 127.0.0.1:8000/file/test -o test
```

## Service

### Start the service
By default, the service is runnning on port 8000 of localhost.
```
git clone https://github.com/buszk/file-storage
cd file-storage
cargo run
```

If you want to expose the service to external internet, be sure to bind to your external ip with the following command.
You can use `ipconfig` on Mac/Linux or `ifconfig` on Windows to find your external ip address.
```
cargo run -- --addr=<external-ip>:<port>
```

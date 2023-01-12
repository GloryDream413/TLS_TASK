use std::thread;
use std::fs;
use std::path::Path;
use std::io::{BufReader, Read, BufRead};
use std::os::unix::net::{UnixStream, UnixListener};
use std::fs::File;
use yaml_rust::{YamlLoader, YamlEmitter};
extern crate yaml_rust;

fn handle_client(stream: UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        println!("UDS data : {}", line.unwrap());
    }
}

fn main() {
    let path = Path::new("config.yaml");
    let mut file = match File::open(&path) {
        Err(_) => panic!("couldn't open"),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => panic!("couldn't read."),
        Ok(_)    => file,
    };
    let docs = YamlLoader::load_from_str(&s).unwrap();
    let doc = &docs[0];
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);
    emitter.dump(&doc["UDS"][0]).unwrap();

    let udspath:Vec<&str> = out_str.split(":").collect();
    let _udssocketpath = udspath[1].replace("\"", "");
    let socket = Path::new(&_udssocketpath);

    // Delete old socket if necessary
    if socket.exists() {
        if let Err(_e)  = fs::remove_file(&socket) {
            println!("Failed to remove the socket file.");
        }
    }

    // Bind to socket
    let stream = match UnixListener::bind(&socket) {
        Err(_) => panic!("failed to bind socket"),
        Ok(stream) => stream,
    };

    println!("Server started, waiting for clients");

    // Iterate over clients, blocks if no client available
    for client in stream.incoming() {
        match client {
            Ok(client) => {
                println!("New connection : UDS starting");
                thread::spawn(|| handle_client(client));
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
    drop(stream);
}
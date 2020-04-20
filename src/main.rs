use tokio::net::TcpStream;
use tokio::prelude::*;
use std::sync::{RwLock, Arc};
use std::sync::mpsc::{channel, Receiver, Sender};
use rand::Rng;
use std::{thread, time};
use std::io::{self, Read};
use std::thread::Thread;

fn main() {
    let (cmd_sender, cmd_receiver) = channel::<i32>();
    let (evt_sender, evt_receiver) = channel::<i32>();

    thread::spawn(move || {
        let mut input = String::new();
        println!("Input cmd: ");
        loop {
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    let cmd_str = &input[0..input.len() - 1];
                    match cmd_str.parse::<i32>() {
                        Ok(cmd) => {
                            println!("send cmd: {}", cmd);
                            cmd_sender.send(cmd).unwrap();
                            if cmd == 0 {
                                break;
                            }
                        }
                        Err(error) => {
                            println!("parse int error: {}", error.to_string());
                        }
                    }
                }
                Err(error) => println!("error: {}", error),
            }
            input.clear();
        }
    });

    thread::spawn(move || {
        loop {
            let evt = evt_receiver.recv().unwrap();
            println!("received evt: {}", evt);
            if evt == 0 {
                break;
            }
        }
    });

    let next_cmd = || -> i32 {
        let cmd = cmd_receiver.recv().unwrap();
        println!("received cmd: {}", cmd);
        cmd
    };

    let apply_evt = |evt:i32| {
        println!("send evt: {}", evt);
        evt_sender.clone().send(evt);
    };

    service(next_cmd, apply_evt);
}

async fn calc(cmd: i32) -> i32 {
    let dur = time::Duration::from_millis(10);
    thread::sleep(dur);
    println!("({}) calc {}", thread::current().name().unwrap(), cmd);
    cmd * 10
}

#[tokio::main]
async fn service(next_cmd_fn: impl Fn() -> i32, apply_evt_fn: impl Fn(i32)) {
    loop {
        let cmd = next_cmd_fn();
        let evt = calc(cmd).await;
        apply_evt_fn(evt);
        if evt == 0 {
            //thread::sleep(time::Duration::from_millis(1000));
            break;
        }
    }
}
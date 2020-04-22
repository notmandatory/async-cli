use crossbeam::channel::{Sender, Receiver, unbounded};
use once_cell::sync::Lazy;
use std::io;
use std::thread;
use std::time;
use tokio::time::delay_for;

static CMD_CHANNEL: Lazy<(Sender<i32>, Receiver<i32>)> = Lazy::new(unbounded);
static EVT_CHANNEL: Lazy<(Sender<i32>, Receiver<i32>)> = Lazy::new(unbounded);

fn main() {
    // user input thread
    thread::spawn(move || {
        let mut input = String::new();
        println!("Input cmd (integer or 0 to exit): ");
        loop {
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let cmd_str = &input[0..input.len() - 1];
                    match cmd_str.parse::<i32>() {
                        Ok(cmd) => {
                            CMD_CHANNEL.0.send(cmd).unwrap();
                            println!("send cmd: {}", cmd);
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

    // user output thread
    thread::spawn(move || {
        loop {
            let evt = EVT_CHANNEL.1.recv().unwrap();
            println!("received evt: {}", evt);
            if evt == 0 {
                break;
            }
        }
    });

    start();
}

// async processing thread
#[tokio::main]
async fn start() {
    loop {
        match CMD_CHANNEL.1.try_recv() {
            Err(_) => {
                //println!("waiting for cmd.");
                delay_for(time::Duration::from_millis(100)).await
            }
            Ok(cmd) => {
                println!("received cmd: {}", cmd);
                if cmd == 0 {
                    println!("quitting.");
                    break;
                }
                tokio::spawn(async move {
                    let evt = calc(cmd).await;
                    EVT_CHANNEL.0.send(evt).unwrap();
                    println!("sent evt: {}", evt);
                });
            }
        }
    }
}

async fn calc(cmd: i32) -> i32 {
    println!("({}) start calc for cmd {}", thread::current().name().unwrap(), cmd);
    let dur = time::Duration::from_millis(cmd as u64);
    delay_for(dur).await;
    let evt = cmd * 10;
    println!("({}) finish calc for cmd {} -> evt {}", thread::current().name().unwrap(), cmd, evt);
    evt
}
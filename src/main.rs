use core::num;
/*
    usage:
    ip_sniffer -h
    ip_sniffer -j thread_num ip_addr
    ip_sniffer ip_addr (uses def num of threads)

*/
use ::std::sync::mpsc::{channel, Sender};
use std::thread;
use std::{env, net::{IpAddr, TcpStream}, process, str::FromStr}; // include mantığı
use std::io::{self, Write};
const MAX: u16 = 65535;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!(
                    "usage: 
                ip_sniffer -h shows this message
                ip_sniffer -j thread_num ip_addr
                ip_sniffer ip_addr (uses def num of threads)
                "
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IP address"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number"),
                };
                return Ok(Arguments {
                    threads,
                    flag,
                    ipaddr,
                });
            } else {
                return Err("invalid syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1; 
    loop {
        match TcpStream::connect((addr,port)) {  // match try catch gibi birşey herhalde...
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            } Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect(); // collect arguments we passed to our program
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {} ", program, err);
            process::exit(0);
        }
    });

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is out", v);
    }
}

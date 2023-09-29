use std::env;
use std::io::{self,Write};
use std::io::read_to_string;
use std::net::{IpAddr,TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender,channel};
use std::thread;

const MAX:u16 = 65535;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String])-> Result<Arguments, &'static str>{
        if args.len()<2 {
            return Err("没有足够的参数");
        }else if args.len()>4 {
            return Err("参数过多");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguments {flag: String::from(""),ipaddr,threads:4});
        }else {
            let flag = args[1].clone();
            if flag.contains("-h")||flag.contains("-help") && args.len() ==2{
                println!("用法：-j 选择多少个线程，-h或者-help显示帮助提示");
                return Err("帮助");
            }else if flag.contains("-h")||flag.contains("-help") {
                return Err("太多参数");
            }else if flag.contains("-j"){
                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s)=>s,
                    Err(_)=> return Err("Ip验证不通过，必须使用ipv4格式或者ipv6格式")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s)=>s,
                    Err(_)=>return Err("failed to parse thread number")
                };
                return Ok(Arguments{threads,flag,ipaddr});
            } else {
                return Err("语法错误");
            }
        }
    }
}
fn scan(tx:Sender<u16>,start_port:u16,addr:IpAddr,num_threads:u16){
    let mut port:u16 = start_port+1;
    loop{
        match TcpStream::connect((addr,port)){
          Ok(_)=>{
               println!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_)=>{}
        };
        if (MAX-port)<= num_threads{
            break;
        }
        port+=num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();//获取输入
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err|{
            if err.contains("help"){
                process::exit(0);
            }else {
                eprintln!("{} problem parsing arguments: {}",program,err);
                process::exit(0);
            }
        }
    );
    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx,rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx,i,arguments.ipaddr,num_threads);
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
        println!("{} is open",v);
    }
    //  for i in &args {
    //      println!("输入了{}",i);
    //  }
    // println!("输入的数据{:?}",args)
    // let program = args[1].clone();
    // let threads = args[2].clone();
    // let ipaddr = args[3].clone();
}

// ip_sniffer.exe -h
// ip_sniffer.exe -j 100 192.168.1.1
// ip_sniffer.exe 192.168.1.1
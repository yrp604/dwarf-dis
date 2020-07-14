use std::env;
use std::fs;
use std::process;

use dwarf_dis::dis;

use flexi_logger::Logger;

fn main() {
    Logger::with_env_or_str("debug").start().unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <dwarf bytecode>", args[0]);
        process::exit(1);
    }

    let bytecode = fs::read(&args[1]).expect("Could not read bytecode");

    let res = dis(&bytecode[48..0x418]);

    for (addr, op) in res {
        println!("{:04x}: {:x?}", addr, op);
    }
}

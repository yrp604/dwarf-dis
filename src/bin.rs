use std::env;
use std::fs;
use std::process;

use dwarf_dis::decode;

use flexi_logger::Logger;

fn main() {
    Logger::with_env_or_str("warn").start().unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <dwarf bytecode>", args[0]);
        process::exit(1);
    }

    let bytecode = fs::read(&args[1]).expect("Could not read bytecode");

    let mut pc = 0;
    loop {
        if pc >= bytecode.len() {
            break;
        }

        let (sz, op) = decode(&bytecode[pc..]).unwrap();

        println!("{:04x}: {:x?}", pc, op);

        pc += sz;
    }
}

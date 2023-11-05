mod compile;
mod vm;
use compile::{to_node_bf, to_instruction_codes};

fn main() {
    let buf = std::fs::read(std::env::args().nth(1).unwrap())
                .expect("Should have been able to read the file");
    let a = to_node_bf(buf);
    let b = to_instruction_codes(a, 0);
    let mut bf = vm::VirtualMachine::new(b);
    loop {
        if bf.exec() == false {
            break;
        }
    }
    println!("");
}

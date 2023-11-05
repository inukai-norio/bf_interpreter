use std::io::Read;
use crate::compile;
use compile::Instruction;
pub struct VirtualMachine {
    code: Vec<Instruction>,
    buf: Vec<u8>,
    i: usize,
    p: u16,
}

impl VirtualMachine {
    pub fn new(code: Vec<Instruction>) -> Self {
        VirtualMachine {
            code,
            buf: vec![0u8; 65536],
            i: 0,
            p: 0,
        }
    }
/*
{ num: 1, arg: 1 },
            Instruction { num: 8, arg: 4 }
*/
    pub fn exec(&mut self) -> bool {
        let c = self.code.get(self.i);
        if c.is_none() {
            return false;
        }
        self.i += 1;

        let cc: &Instruction = c.unwrap();
        match cc.num {
            0 => { // NOP
                
            },
            1 => { // ADD
                self.buf[self.p as usize] = self.buf[self.p as usize].wrapping_add(cc.arg.to_be_bytes()[3]);
            },
            2 => { // SUB
                self.buf[self.p as usize] = self.buf[self.p as usize].wrapping_sub(cc.arg.to_be_bytes()[3]);
            },
            6 => { // IN
                self.buf[self.p as usize] = self.read_char();
            },
            5 => { // OUT
                print!("{}", char::from(self.buf[self.p as usize]));
            },
            3 => { // INC
                let l = cc.arg.to_be_bytes();
                self.p = self.p.wrapping_add(u16::from_be_bytes([l[2], l[3]]));
            },
            4 => { // DEC
                let l = cc.arg.to_be_bytes();
                self.p = self.p.wrapping_sub(u16::from_be_bytes([l[2], l[3]]));
            },
            8 => { // [
                if self.buf[self.p as usize] == 0 {
                    self.i = cc.arg as usize;
                }
            },
            7 => { // ]
                self.i = cc.arg as usize;
            },
            _ => {},
        }
        true
    }
    
    fn read_char(&mut self) -> u8 {
        let mut ret: [u8; 1] = [0];
        std::io::stdin().read_exact(&mut ret).unwrap_or_else(|_| std::process::exit(0));
        ret[0]
    }
}

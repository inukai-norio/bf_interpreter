use std::io::Read;
struct Bf<'a> {
    v: Vec<u8>,
    buf: Vec<u8>,
    i: usize,
    p: u16,
    read: std::io::StdinLock<'a>,
}

impl Bf<'_> {
    fn new() -> Self {
        Bf {
            v: std::fs::read(std::env::args().nth(1).unwrap())
                .expect("Should have been able to read the file"),
            buf: vec![0u8; 65536],
            i: 0,
            p: 0,
            read: std::io::stdin().lock(),
        }
    }
    fn read_char(&mut self) -> u8 {
        let mut ret: [u8; 1] = [0];
        self.read.read_exact(&mut ret).unwrap_or_else(|_| std::process::exit(0));
        ret[0]
    }
    pub fn exec(&mut self) -> bool {
        match self.v.get(self.i) {
            None => { return false; },
            Some(43) => { // +
                self.buf[self.p as usize] = self.buf[self.p as usize].wrapping_add(1);
            },
            Some(45) => { // -
                self.buf[self.p as usize] = self.buf[self.p as usize].wrapping_sub(1);
            },
            Some(44) => { // ,
                self.buf[self.p as usize] = self.read_char();
            },
            Some(46) => { // .
                print!("{}", char::from(self.buf[self.p as usize]));
            },
            Some(62) => { // >
                self.p = self.p.wrapping_add(1);
            },
            Some(60) => { // <
                self.p = self.p.wrapping_sub(1);
            },
            Some(91) => { // [
                if self.buf[self.p as usize] == 0 {
                    let mut s: usize = 0;
                    loop {
                        self.i += 1;
                        let m = self.v.get(self.i);
                        if m == Some(&93) { // ]
                            if s == 0 {
                                break;
                            }
                            s -= 1;
                        } else if m == Some(&91) { // [
                            s += 1;
                        }
                    }
                }
            },
            Some(93) => { // ]
                let mut s: usize = 0;
                loop {
                    self.i -= 1;
                    let m = self.v.get(self.i);
                    if m == Some(&91) { // [
                        if s == 0 {
                            break;
                        }
                        s -= 1;
                    } else if m == Some(&93) { // ]
                        s += 1;
                    }
                }
                self.i -= 1;
            },
            _ => {},
        }
        self.i += 1;
        true
    }
}

fn main() {
    let mut bf = Bf::new();
    loop {
        if bf.exec() == false {
            break;
        }
    }
    println!("");
}

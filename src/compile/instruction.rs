use super::nodebf::NodeBf;

#[derive(Clone)]
pub struct Instruction {
    pub num: u8,
    pub arg: u32,
}



pub fn to_nojump_instruction_codes(codes: Vec<u8>) -> Vec<Instruction> {
    let mut buf: Vec<Instruction> = Vec::new();
    let mut codes_iter = codes.iter().peekable();
    loop {
        let code_op = codes_iter.next();
        if code_op == None {
            break;
        }
        let code: &u8 = code_op.unwrap();
        match code {
            44 => { // ,
                buf.push(Instruction { num: 6, arg: 0 });
            },
            46 => { // .
                buf.push(Instruction { num: 5, arg: 0 }); 
            },
            _ => {
                let mut i = 1;
                loop {
                    if let Some(c) = codes_iter.peek() {
                        if code == *c {
                            let _ = codes_iter.next();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                    i += 1;
                }
                match code {
                    43 => { // +
                        buf.push(Instruction { num: 1, arg: i });
                    },
                    45 => { // -
                        buf.push(Instruction { num: 2, arg: i });
                    },
                    62 => { // >
                        buf.push(Instruction { num: 3, arg: i });
                    },
                    60 => { // <
                        buf.push(Instruction { num: 4, arg: i });
                    },
                    _ => {},
                }
            }
        }
        
    }
    buf
}

impl From<Instruction> for u32 {
    fn from(item: Instruction) -> Self {
        let arg = item.arg.to_be_bytes();
        u32::from_be_bytes([item.num, arg[1], arg[2], arg[3]])
    }
}

impl From<u32> for Instruction {
    fn from(item: u32) -> Self {
        let arg = item.to_be_bytes();
        Instruction { num: arg[0], arg: u32::from_be_bytes([0, arg[1], arg[2], arg[3]]) }
    }
}
impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        let s_arg = self.arg.to_be_bytes();
        let o_arg = other.arg.to_be_bytes();

        self.num == other.num &&
        s_arg[1] == o_arg[1] &&
        s_arg[2] == o_arg[2] &&
        s_arg[3] == o_arg[3]
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arg = &self.arg.to_be_bytes();
        f.debug_struct("Instruction")
         .field("num", &self.num)
         .field("arg", &[arg[1], arg[2], arg[3]])
         .finish()
    }
}

pub fn to_instruction_codes(bfs: Vec<Box<NodeBf>>, start: u32) -> Vec<Instruction> {
    let mut buf: Vec<Instruction> = Vec::new();
    for bf in bfs {
        let cbf = to_nojump_instruction_codes(bf.r);
        buf.extend(cbf);
        let c1 = buf.len() as u32 + start;
        if bf.l.is_empty() {
            continue;
        }
        let loop_buf: Vec<Instruction> = to_instruction_codes(bf.l, c1 + 1);
        let c2 = loop_buf.len() as u32 + 2;
        buf.push(Instruction { num: 8, arg: (c1 + c2) });
        buf.extend(loop_buf);
        buf.push(Instruction { num: 7, arg: (c1) });
    }
    buf
}



#[cfg(test)]
mod to_instruction_codes_test {
    use super::{super::nodebf::to_node_bf, Instruction, to_instruction_codes};

    #[test]
    fn t1() {
        let v: Vec<u8> = "+[+]".as_bytes().to_vec();
        let a = to_node_bf(v);
        let b = to_instruction_codes(a, 0);

        assert_eq!(b.len(), 4);

        assert_eq!(b, [
            Instruction { num: 1, arg: 1 },
            Instruction { num: 8, arg: 4 },
            Instruction { num: 1, arg: 1 },
            Instruction { num: 7, arg: 1 },
        ]);
    }

    #[test]
    fn t2() {
        let v: Vec<u8> = "+[+]+[+.]".as_bytes().to_vec();
        let a = to_node_bf(v);
        let b = to_instruction_codes(a, 0);

        assert_eq!(b.len(), 9);

        assert_eq!(b[0], Instruction { num: 1, arg: 1 });
        assert_eq!(b[1], Instruction { num: 8, arg: 4 });
        assert_eq!(b[2], Instruction { num: 1, arg: 1 });
        assert_eq!(b[3], Instruction { num: 7, arg: 1 });
        assert_eq!(b[4], Instruction { num: 1, arg: 1 });
        assert_eq!(b[5], Instruction { num: 8, arg: 9 });
        assert_eq!(b[6], Instruction { num: 1, arg: 1 });
        assert_eq!(b[7], Instruction { num: 5, arg: 0 });
        assert_eq!(b[8], Instruction { num: 7, arg: 5 });
    }
    #[test]
    fn t3() {
        let v: Vec<u8> = "+[>+[+]<+]".as_bytes().to_vec();
        let a = to_node_bf(v);
        let b = to_instruction_codes(a, 0);

        assert_eq!(b.len(), 10);

        assert_eq!(b[0], Instruction { num: 1, arg: 1 });
        assert_eq!(b[1], Instruction { num: 8, arg: 10 });
        assert_eq!(b[2], Instruction { num: 3, arg: 1 });
        assert_eq!(b[3], Instruction { num: 1, arg: 1 });
        assert_eq!(b[4], Instruction { num: 8, arg: 7 });
        assert_eq!(b[5], Instruction { num: 1, arg: 1 });
        assert_eq!(b[6], Instruction { num: 7, arg: 4 });
        assert_eq!(b[7], Instruction { num: 4, arg: 1 });
        assert_eq!(b[8], Instruction { num: 1, arg: 1 });
        assert_eq!(b[9], Instruction { num: 7, arg: 1 });
    }
    #[test]
    fn t4() {
        let v: Vec<u8> = "+[>+[>+[+]<+]<+]".as_bytes().to_vec();
        let a = to_node_bf(v);
        let b = to_instruction_codes(a, 0);

        assert_eq!(b.len(), 16);

        assert_eq!(b[0], Instruction { num: 1, arg: 1 });
        assert_eq!(b[1], Instruction { num: 8, arg: 16 });
        assert_eq!(b[2], Instruction { num: 3, arg: 1 });
        assert_eq!(b[3], Instruction { num: 1, arg: 1 });
        assert_eq!(b[4], Instruction { num: 8, arg: 13 });
        assert_eq!(b[5], Instruction { num: 3, arg: 1 });
        assert_eq!(b[6], Instruction { num: 1, arg: 1 });
        assert_eq!(b[7], Instruction { num: 8, arg: 10 });
        assert_eq!(b[8], Instruction { num: 1, arg: 1 });
        assert_eq!(b[9], Instruction { num: 7, arg: 7 });
        assert_eq!(b[10], Instruction { num: 4, arg: 1 });
        assert_eq!(b[11], Instruction { num: 1, arg: 1 });
        assert_eq!(b[12], Instruction { num: 7, arg: 4 });
        assert_eq!(b[13], Instruction { num: 4, arg: 1 });
        assert_eq!(b[14], Instruction { num: 1, arg: 1 });
        assert_eq!(b[15], Instruction { num: 7, arg: 1 });
    }

    #[test]
    fn t5() {
        let v: Vec<u8> = "+[>+[>+[+]-[+]<+]<+]".as_bytes().to_vec();
        let a = to_node_bf(v);
        let b = to_instruction_codes(a, 0);

        assert_eq!(b.len(), 20);

        assert_eq!(b[0], Instruction { num: 1, arg: 1 });
        assert_eq!(b[1], Instruction { num: 8, arg: 20 });
        assert_eq!(b[2], Instruction { num: 3, arg: 1 });
        assert_eq!(b[3], Instruction { num: 1, arg: 1 });
        assert_eq!(b[4], Instruction { num: 8, arg: 17 });
        assert_eq!(b[5], Instruction { num: 3, arg: 1 });
        assert_eq!(b[6], Instruction { num: 1, arg: 1 });
        assert_eq!(b[7], Instruction { num: 8, arg: 10 });
        assert_eq!(b[8], Instruction { num: 1, arg: 1 });
        assert_eq!(b[9], Instruction { num: 7, arg: 7 });
        assert_eq!(b[10], Instruction { num: 2, arg: 1 });
        assert_eq!(b[11], Instruction { num: 8, arg: 14 });
        assert_eq!(b[12], Instruction { num: 1, arg: 1 });
        assert_eq!(b[13], Instruction { num: 7, arg: 11 });
        assert_eq!(b[14], Instruction { num: 4, arg: 1 });
        assert_eq!(b[15], Instruction { num: 1, arg: 1 });
        assert_eq!(b[16], Instruction { num: 7, arg: 4 });
        assert_eq!(b[17], Instruction { num: 4, arg: 1 });
        assert_eq!(b[18], Instruction { num: 1, arg: 1 });
        assert_eq!(b[19], Instruction { num: 7, arg: 1 });
    }

    #[test]
    fn t6() {
        let v: Vec<u8> = "++++++++[>++++++++<-]>+.".as_bytes().to_vec();
        let a = to_node_bf(v);
        let b = to_instruction_codes(a, 0);

        assert_eq!(b.len(), 10);

        assert_eq!(b[0], Instruction { num: 1, arg: 8 });
        assert_eq!(b[1], Instruction { num: 8, arg: 7 });
        assert_eq!(b[2], Instruction { num: 3, arg: 1 });
        assert_eq!(b[3], Instruction { num: 1, arg: 8 });
        assert_eq!(b[4], Instruction { num: 4, arg: 1 });
        assert_eq!(b[5], Instruction { num: 2, arg: 1 });
        assert_eq!(b[6], Instruction { num: 7, arg: 1 });
        assert_eq!(b[7], Instruction { num: 3, arg: 1 });
        assert_eq!(b[8], Instruction { num: 1, arg: 1 });
        assert_eq!(b[9], Instruction { num: 5, arg: 0 });
    }
}

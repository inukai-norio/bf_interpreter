use std::io::Read;

fn read_char() -> u8 {
    let mut ret: [u8; 1] = [0];
    std::io::stdin().read_exact(&mut ret).unwrap_or_else(|_| std::process::exit(0));
    ret[0]
}

fn bf_open() -> Vec<u8> {
    let file_path: String = std::env::args().nth(1).unwrap();

    let contents = std::fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    contents.into_bytes()
}

fn bf_init() -> Vec<u8> {
    vec![0u8; 30000]
}

fn main() {
    let v: Vec<u8> = bf_open();
    let mut buf: Vec<u8> = bf_init();
    let mut i: usize = 0;
    let mut p: usize = 0;
    loop {
        match v.get(i) {
            None => { break; },
            Some(43) => { // +
                buf[p] = buf[p].wrapping_add(1);
            },
            Some(45) => { // -
                buf[p] = buf[p].wrapping_sub(1);
            },
            Some(44) => { // ,
                buf[p] = read_char();
            },
            Some(46) => { // .
                print!("{}", char::from(buf[p]));
            },
            Some(62) => { // >
                p = p.wrapping_add(1);
            },
            Some(60) => { // <
                p = p.wrapping_sub(1);
            },
            Some(91) => { // [
                if buf[p] == 0 {
                    let mut s: usize = 0;
                    loop {
                        i += 1;
                        let m = v.get(i);
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
                    i -= 1;
                    let m = v.get(i);
                    if m == Some(&91) { // [
                        if s == 0 {
                            break;
                        }
                        s -= 1;
                    } else if m == Some(&93) { // ]
                        s += 1;
                    }
                }
                i -= 1;
            },
            _ => {},
        }
        i += 1;
    }

    println!("");
}

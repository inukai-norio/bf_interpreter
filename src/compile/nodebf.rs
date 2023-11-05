

#[derive(Clone)]
pub struct NodeBf {
    pub r: Vec<u8>,
    pub l: Vec<Box<NodeBf>>,
}


pub fn to_node_bf(item: Vec<u8>) -> Vec<Box<NodeBf>> {
    fn to_nodebf(item: Vec<u8>) -> Option<(NodeBf, usize)> {
        let mut i: usize = 0;
        let mut j: usize = 0;
        loop {
            let n = item.get(i);
            if n == Some(&91) {
                let mut s: usize = 0;
                loop {
                    j += 1;
                    let m = item.get(i + j);
                    if m == Some(&93) { // ]
                        if s == 0 {
                            break;
                        }
                        s -= 1;
                    } else if m == Some(&91) { // [
                        s += 1;
                    }
                }
                break;
            } else if n == None {
                if i == 0 {
                    return None;
                }
                let n = NodeBf {
                    r: item[0..i].to_vec(),
                    l: Vec::new(),
                };
                return Some((n, i));
            }
            i += 1;
        }
        let n = NodeBf {
            r: item[0..i].to_vec(),
            l: to_vec_nodebf(item[(i+1)..(i+j)].to_vec()),
        };
        Some((n, i + j + 1))
    }
    fn to_vec_nodebf(item: Vec<u8>) -> Vec<Box<NodeBf>> {
        let mut i: usize = 0;
        let mut buf: Vec<Box<NodeBf>> = Vec::new();
        loop {
            let item_slice = item[i..].to_vec();
            let ret = to_nodebf(item_slice);
            match ret {
                Some((r, next)) => {
                    buf.push(Box::new(r));
                    i += next;
                },
                None => {
                    break;
                }
            }
        }
        buf
    }
    to_vec_nodebf(item)
}


#[cfg(test)]
mod bf_root_test {
    use super::to_node_bf;

    #[test]
    fn t1() {
        let v: Vec<u8> = "+[+]".as_bytes().to_vec();
        let a = to_node_bf(v);
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].r, [43]);
        assert_eq!(a[0].l.len(), 1);
        assert_eq!(a[0].l[0].r, [43]);
        assert_eq!(a[0].l[0].l.len(), 0);
    }

    #[test]
    fn t2() {
        let v: Vec<u8> = "+[+]+[+.]".as_bytes().to_vec();
        let a = to_node_bf(v);
        assert_eq!(a.len(), 2);

        assert_eq!(a[0].r, [43]);
        assert_eq!(a[0].l.len(), 1);
        assert_eq!(a[0].l[0].r, [43]);
        assert_eq!(a[0].l[0].l.len(), 0);

        assert_eq!(a[1].r, [43]);
        assert_eq!(a[1].l.len(), 1);
        assert_eq!(a[1].l[0].r, [43, 46]);
        assert_eq!(a[1].l[0].l.len(), 0);
    }
    #[test]
    fn t3() {
        let v: Vec<u8> = "+[>+[+]<+]".as_bytes().to_vec();
        let a = to_node_bf(v);
        assert_eq!(a.len(), 1);

        assert_eq!(a[0].r, [43]);
        assert_eq!(a[0].l.len(), 2);
        assert_eq!(a[0].l[0].r, [62, 43]);
        assert_eq!(a[0].l[0].l.len(), 1);
        assert_eq!(a[0].l[0].l[0].r, [43]);
        assert_eq!(a[0].l[0].l[0].l.len(), 0);
    }
}

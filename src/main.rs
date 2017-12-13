fn main() {
    let size: usize = std::env::args()
        .nth(1)
        .unwrap_or("5".to_string()).parse().unwrap();

    let l_str = std::env::args()
        .nth(2)
        .unwrap_or("3,4,1,5".to_string());

    let cycles: usize = std::env::args()
        .nth(3)
        .unwrap_or("1".to_string()).parse().unwrap();

    let extra_len = std::env::args()
        .nth(4)
        .map(parse_lengths)
        .unwrap_or_default();

    let ascii = std::env::args()
        .nth(5).unwrap_or_default() == "-a".to_string();

    let lenghts = if !ascii {
        parse_lengths(l_str)
    } else {
        convert_lengths(l_str)
    };

    let sparse = sparse_hash(size, lenghts, extra_len, cycles);

    println!("Result = {}", (sparse[0] as u32 * sparse[1] as u32));

    let hash_str = as_hex_string(dense_hash(sparse));

    println!("Hash = {}", hash_str);
}

fn sparse_hash(size: usize, mut lenghts: Vec<u8>, extra: Vec<u8>, cycles: usize) -> Vec<u8> {
    let mut ring = (0..size).map(|u| u as u8).collect::<Vec<_>>();
    {
        lenghts.extend(extra);

        let mut all = Vec::<u8>::with_capacity(lenghts.len() * cycles);

        for _ in 0..cycles {
            all.extend(lenghts.iter());
        }
        hash(&mut ring, &all);
    }
    ring
}

fn swap_slice<V: AsMut<[u8]>>(mut data: V, from: usize, to: usize) -> V {
    {
        let v = data.as_mut();
        let l = v.len();
        let from = from % l;
        let mut to = to % l;
        if to < from {
            to += v.len()
        }
        for i in 0..(to - from + 1) / 2 {
            v.swap((from + i) % l, (to - i) % l)
        }
    }
    data
}

fn hash<V: AsMut<[u8]>, W: AsRef<[u8]>>(mut ring: V, lengths: W) -> V {
    let mut pos = 0;
    let mut skip = 0;

    for l in lengths.as_ref() {
        let l = *l as usize;
        if l > 0 {
            swap_slice(ring.as_mut(), pos, pos + l - 1);
        }
        pos += l + skip;
        skip += 1;
    }
    ring
}

fn convert_lengths<S: AsRef<str>>(data: S) -> Vec<u8> {
    data.as_ref().as_bytes().iter().cloned().collect()
}

fn parse_lengths<S: AsRef<str>>(data: S) -> Vec<u8> {
    data.as_ref().split(',')
        .map(|t| t.parse::<u8>().unwrap())
        .collect()
}

fn xor_it<V: AsRef<[u8]>>(v: V) -> u8 {
    v.as_ref().iter().fold(0, |s, e| s ^ e)
}


fn dense_hash<V: AsRef<[u8]>>(v: V) -> Vec<u8> {
    v.as_ref()
        .chunks(16).map(xor_it)
        .collect()
}

fn as_hex_string<V: AsRef<[u8]>>(v: V) -> String {
    v.as_ref().iter().map(|u| format!("{:02x}", u)).collect::<Vec<_>>().join("")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn swap_slice_simple_odd() {
        assert_eq!(&[3, 2, 1, 4, 5], swap_slice(&mut [1, 2, 3, 4, 5], 0, 2))
    }

    #[test]
    fn swap_slice_simple_even() {
        assert_eq!(&[4, 3, 2, 1, 5], swap_slice(&mut [1, 2, 3, 4, 5], 0, 3))
    }

    #[test]
    fn swap_circular_odd() {
        assert_eq!(&[4, 2, 3, 1, 5], swap_slice(&mut [1, 2, 3, 4, 5], 3, 0))
    }

    #[test]
    fn swap_circular_even() {
        assert_eq!(&[5, 4, 3, 2, 1], swap_slice(&mut [1, 2, 3, 4, 5], 3, 1))
    }

    #[test]
    fn process_hash() {
        let mut ring = [0, 1, 2, 3, 4];
        let lengths = [3, 4, 1, 5];

        assert_eq!(&[3, 4, 2, 1, 0], hash(&mut ring, lengths));
    }

    #[test]
    fn process_hash_zero_length_should_do_nothing() {
        let mut orig = [0, 1, 2, 3, 4];
        let mut ring = orig.clone();
        let mut lengths = vec![3, 4, 1, 5];

        let expected = hash(&mut orig, &lengths);

        lengths.push(0);

        assert_eq!(expected, hash(&mut ring, lengths));
    }

    #[test]
    fn test_convert_lengths() {
        assert_eq!(vec![49, 44, 50, 44, 51], convert_lengths("1,2,3"))
    }

    #[test]
    fn dense_hash_simple() {
        assert_eq!(vec![64], dense_hash(vec![65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22]))
    }

    #[test]
    fn dense_hash_two() {
        assert_eq!(vec![64,41], dense_hash(vec![65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22,
                                                65, 27, 9, 1, 4, 3, 40, 0, 0, 7, 6, 0, 2, 5, 68, 22]))
    }

    #[test]
    fn test_as_hex_str() {
        assert_eq!("ae021f12".to_string(), as_hex_string(vec![0xae, 0x02, 0x1f, 0x12]))
    }

    fn process_puzzle(puzzle: &str) -> String {
        let lengths = convert_lengths(puzzle);
        let extra = vec![17, 31, 73, 47, 23];
        let size = 256;
        let cycle = 64;

        as_hex_string(dense_hash(sparse_hash(size, lengths, extra, cycle)))
    }

    #[test]
    fn integration_0() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", process_puzzle(""))
    }

    #[test]
    fn integration_1() {
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", process_puzzle("AoC 2017"))
    }

    #[test]
    fn integration_2() {
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", process_puzzle("1,2,3"))
    }

    #[test]
    fn integration_3() {
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", process_puzzle("1,2,4"))
    }
}

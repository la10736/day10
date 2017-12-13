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

    let ring = compute(size,lenghts, extra_len, cycles);

    println!("Result = {}", (ring[0] as u32 * ring[1] as u32));
}

fn compute(size: usize, mut lenghts: Vec<u8>, extra: Vec<u8>, cycles: usize) -> Vec<u8> {
    let mut ring = (0..size).map(|u| u as u8).collect::<Vec<_>>();
    {
        lenghts.extend(extra);

        for _ in 1..cycles {
            let c = lenghts.clone();
            lenghts.extend(c);
        }
        hash(&mut ring, &lenghts);
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
}

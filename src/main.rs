extern crate day10;

use day10::*;

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

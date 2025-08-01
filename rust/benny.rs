use std::time::SystemTime;

const RAW_DATA: &str = include_str!("../input.txt");
const DATA: &[u8] = RAW_DATA.as_bytes();

pub fn find_first_fourteen(input: &[u8]) -> Option<usize> {
  let mut filter = 0u32;
  input
    .iter()
    .take(14 - 1)
    .for_each(|c| filter ^= 1 << (c % 32));

  input.windows(14).position(|w| {
    let first = w[0];
    let last = w[w.len() - 1];
    filter ^= 1 << (last % 32);
    let res = filter.count_ones () == 14 as u32;
    filter ^= 1 << (first % 32);
    res
  })
}

fn main() {
  let before = SystemTime::now();
  let start = find_first_fourteen(DATA);
  let after = SystemTime::now();

  let time_taken = after.duration_since(before).unwrap();

  println!("find_first_fourteen found {:?} and took {} nanoseconds\n", Some(start), time_taken.as_nanos());
}

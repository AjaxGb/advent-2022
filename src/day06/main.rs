use std::time::Instant;

fn all_unique(buffer: &[char]) -> bool {
    let mut flags = 0u32;
    for &c in buffer {
        assert!(c.is_ascii_lowercase());
        let flag = 1 << (c as u32 - 'a' as u32);
        if flags & flag != 0 {
            return false;
        }
        flags |= flag;
    }
    true
}

fn message_start_index<const N: usize>(signal: &str) -> usize {
    let mut signal = signal.chars();
    let mut buffer: [char; N] = std::array::from_fn(|_| signal.next().unwrap());
    let mut pos = N;
    while !all_unique(&buffer) {
        let c = signal.next().unwrap();
        buffer[pos % N] = c;
        pos += 1;
    }
    pos
}

fn main() {
    let signal = include_str!("input.txt").trim();
    let time_a = Instant::now();
    let packet_start = message_start_index::<4>(signal);
    let time_b = Instant::now();
    let message_start = message_start_index::<14>(signal);
    let time_c = Instant::now();
    println!("Start of packet at index {packet_start}");
    println!("  Time: {:?}", time_b - time_a);
    println!("Start of message at index {message_start}");
    println!("  Time: {:?}", time_c - time_b);
}

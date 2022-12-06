fn all_unique(buffer: &[char]) -> bool {
    if let Some(last_index) = buffer.len().checked_sub(1) {
        for i in 0..last_index {
            if buffer[(i + 1)..].contains(&buffer[i]) {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut signal = include_str!("input.txt").trim().chars();
    let mut buffer: [char; 4] = std::array::from_fn(|_| signal.next().unwrap());
    let mut pos = buffer.len();
    while !all_unique(&buffer) {
        let c = signal.next().unwrap();
        buffer[pos % buffer.len()] = c;
        pos += 1;
    }
    println!("Start of message at index {pos}");
}

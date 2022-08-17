use std::mem;

pub fn allocate() {
    let mut bad_vec: Vec<char> = Vec::with_capacity(1024);
    for _ in 0..1024 {
        bad_vec.push('0');
    }
    mem::forget(bad_vec);
}

fn main() {
    allocate();
}

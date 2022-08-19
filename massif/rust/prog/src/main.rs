use std::mem;

//// This method does not work as expected in the `release` version of the program (allocates 2 peaks of memory instead of 3).

//// https://stackoverflow.com/questions/58003899/heap-size-for-rust-programs-very-large-when-measured-with-valgrind-using-massif
//fn alloc_and_free_bytes_using_vec_reserve(bytes: usize) {
//    let mut vec: Vec<u8> = vec![];
//    vec.reserve(bytes);
//}
//
//// This method does not work as expected in the `release` version of the program (allocates 2 peaks of memory instead of 3).
//// https://stackoverflow.com/questions/27175685/how-to-allocate-space-for-a-vect-in-rust
//fn alloc_and_free_bytes_using_vec_of_i32_with_capacity(bytes: usize) {
//    let _v1: Vec<i32> = Vec::with_capacity(get_i32_elements_count_for_desired_bytes_size(bytes));
//}

fn get_i32_elements_count_for_desired_bytes_size(bytes: usize) -> usize {
    let i32_size_as_bytes = mem::size_of::<i32>();
    bytes / i32_size_as_bytes
}

fn alloc_and_free_bytes_using_vec_of_i32_of_0(bytes: usize) {
    let _v1 = vec![0; get_i32_elements_count_for_desired_bytes_size(bytes)];
}

fn main() {
    let one_gb = 1_000_000_000;
    alloc_and_free_bytes_using_vec_of_i32_of_0(one_gb);
    alloc_and_free_bytes_using_vec_of_i32_of_0(one_gb);
    alloc_and_free_bytes_using_vec_of_i32_of_0(one_gb);
}

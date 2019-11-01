mod index_builder;

fn main() {
    index_builder::create_column_store("src/sample.csv", "output_col", 3);
    let mut arr: [u32; 128] = [0; 128];
    for i in 0..64 {
        arr[i as usize] = i+1;
    }
    let mut j:usize = 64;
    for i in 0..64 {
        arr[j] = i+1;
        j += 1;
    }
    index_builder::create_byte_code(&arr);
}

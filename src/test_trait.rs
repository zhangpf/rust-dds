fn main() {
    let mut big_array = [0; 10240000000];
    big_array[0] = 1;
    println!("big_array[0] = {}", big_array[0]);
}
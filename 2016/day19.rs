fn main() {
    const N: usize = 3014387;
    let mut winner = 0;
    for n in 1..(N + 1) {
        winner += 2;
        winner %= n;
    }
    println!("Winner: #{}", winner + 1);
}

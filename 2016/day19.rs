fn main() {
    //const N: usize = 5;
    //const N: usize = 9;
    const N: usize = 3014387;
    println!("For N = {}", N);
    let mut winner = 0;
    for n in 1..(N + 1) {
        winner += 2;
        winner %= n;
    }
    println!("Part 1: Winner: #{}", winner + 1);

    winner = 0;
    for n in 1..(N + 1) {
        winner += 1;
        if winner + 1 >= n {
            winner = 0;
        } else if (winner + 1) * 2 > n {
            winner += 1;
        }
    }
    println!("Part 2: Winner: #{}", winner + 1);
}

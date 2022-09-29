fn main() {
    let index = 10;

    let (mut v, mut next_v) = (1, 1);
    for _i in 0..index {
        println!("{v}");
        (v, next_v) = (next_v, v + next_v);
    }
}

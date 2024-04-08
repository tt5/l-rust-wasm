fn main() {
    let width = std::env::args()
        .nth(1)
        .expect("no width given")
        .parse::<u32>()
        .expect("argument 1 is not a number.");

    let length = std::env::args()
        .nth(2)
        .expect("no length given")
        .parse::<u32>()
        .expect("argument 2 is not a number.");

    println!("Area: {}", width * length);
}

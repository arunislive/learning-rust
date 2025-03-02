// for loop in rust

fn main() {
    let mut num = 120;
    // increment(num);
    num += 1380;
    println!("Enter a Number to calculate increment:{num}");
    // scanf("%d", &num);
    let _i = 1;
    for i in 0..num {
        if (i % 2) == 0 {
            println!("{} is even ", i);
        } else {
            println!("{} is odd ", i);
        }
    }
}

fn main() {
    let mut number = 0;
    loop {
        println!("{number}");
        number += 1;

        if number == 3 {
            break;
        }
    }
}

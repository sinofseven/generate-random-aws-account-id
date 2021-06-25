use rand::{Rng, thread_rng};

const NUMBER_OF_DIGITS: usize = 12;

fn main() {
    let mut rng = thread_rng();
    let mut list: Vec<String> = Vec::new();
    for _ in 0..NUMBER_OF_DIGITS {
        let num = rng.gen_range(0..10);
        let char = format!("{}", num);
        list.push(char);
    }
    let result = list.join("");
    println!("{}", result);
}

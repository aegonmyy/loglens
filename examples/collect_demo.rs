use std::collections::HashSet;

fn main() {
    // ===== Experiment 1: same numbers, different containers =====
    let as_vec: Vec<i32> = [1, 1, 2, 3, 5].into_iter().collect();
    let as_set: HashSet<i32> = [1, 1, 2, 3, 5].into_iter().collect();

    println!("Vec:     {:?}", as_vec); // keeps duplicates AND order
    println!("HashSet: {:?}", as_set); // dedupes, order is arbitrary

    // ===== Experiment 2: chars poured into a String =====
    let joined: String = ['l', 'o', 'g', 's'].into_iter().collect();
    println!("String:  {:?}", joined);

    // ===== Experiment 3: turbofish (same result, different syntax) =====
    let turbo = [1, 2, 3].into_iter().collect::<Vec<i32>>();
    println!("Turbofish: {:?}", turbo);
}

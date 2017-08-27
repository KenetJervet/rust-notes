use std::collections::HashMap;

pub fn hashmaps() {
    // creating hashmaps
    let mut hm: HashMap<String, u32> = HashMap::new();

    // mutation
    hm.insert(String::from("Yukio"), 95);
    hm.insert(String::from("Unknown"), 100);
    hm.entry(String::from("Nigel")).or_insert(90);  // like `setdefault` in Python

    println!("Hashmap is {:?}", hm);

    // creating hashmaps from data
    let students: Vec<String> = vec![
        String::from("Yukio"),
        String::from("Unknown"),
        String::from("Nigel"),
    ];
    let scores: Vec<u32> = vec![95, 100, 90];

    // A collection type must implement the FromIterator trait to support `collect()`
    let mut hm: HashMap<_, _> = students.iter().zip(scores).collect();
    println!("Hashmap is {:?}", hm);

    // Accessing values in a hashmap
    let score_of_yukio = hm[&String::from("Yukio")];
    // let score_of_eva = hm[&String::from("Eva")];  // Panic
    println!("The score of Yukio is {}", score_of_yukio);
    let maybe_score_of_eva = hm.get(&String::from("Eva"));
    println!("Maybe the score of Eva is {:?}", maybe_score_of_eva);

    for (key, value) in &hm {
        println!("{}: {}", key, value);
    }

    // Updating values based on existing ones
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);  // returns a &mut
        *count += 1;  // dereferencing
    }
}

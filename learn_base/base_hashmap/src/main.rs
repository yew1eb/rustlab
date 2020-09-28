use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let keys = vec![String::from("Blue"), String::from("Red")];
    let values = vec![10, 20];
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();


    let k = String::from("Blue");
    if let Some(v) = scores.get(&k) {
        println!("v = {}", v);
    }


    let mut ss1 = HashMap::new();
    ss1.insert(String::from("one"), 1);
    ss1.insert(String::from("two"), 2);
    //key不存在才才插入
    ss1.entry(String::from("one")).or_insert(3);
    println!("ss1 = {:?}", ss1);
    ss1.entry(String::from("three")).or_insert(3);
    println!("ss1 = {:?}", ss1);

    //根据旧值来更新一个值
    let text = "hello world wonderful    world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map = {:?}", map);

}

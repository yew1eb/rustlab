use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot be completed as dialed. 
            Please hang up and try again.",
        "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?",
        _ => "Hi! Who is this again?"
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base() {
        let mut contacts = HashMap::new();

        contacts.insert("Daniel", "798-1364");
        contacts.insert("Ashley", "645-7689");
        contacts.insert("Katie", "435-8291");
        contacts.insert("Robert", "956-1745");

        // 接受一个引用并返回 Option<&V>
        match contacts.get(&"Daniel") {
            Some(&number) => println!("Calling Daniel: {}", call(number)),
            _ => println!("Don't have Daniel's number."),
        }

        // 如果被插入的值为新内容，那么 `HashMap::insert()` 返回 `None`，
        // 否则返回 `Some(value)`
        contacts.insert("Daniel", "164-6743");

        match contacts.get(&"Ashley") {
            Some(&number) => println!("Calling Ashley: {}", call(number)),
            _ => println!("Don't have Ashley's number."),
        }

        contacts.remove(&("Ashley"));

        // `HashMap::iter()` 返回一个迭代器，该迭代器获得
        // 任意顺序的 (&'a key, &'a value) 对。
        // （原文：`HashMap::iter()` returns an iterator that yields
        // (&'a key, &'a value) pairs in arbitrary order.）
        for (contact, &number) in contacts.iter() {
            println!("Calling {}: {}", contact, call(number));
        }
    }

    #[test]
    fn tmain() {
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
}

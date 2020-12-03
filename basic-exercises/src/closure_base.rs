fn counter(i: i32) -> Box<Fn(i32) -> i32> {
    Box::new(move |n: i32| n + i)
}

/**
FnOnce调用参数为self,这意味着它会转移方法接收者的所有权。
换句话说，就是这种方法调用只能被调用一次。

FnMut调用参数为&mut self,这意味着它会对方法接收者进行可变借用。

Fn调用为&self,这意味着它会对方法接收者进行不可变借用，
也就是说，这种方法调用可以被调用多次。
*/

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn tmain() {
        let f = counter(3);
        assert_eq!(4, f(1));

        let add = |a: i32, b: i32| -> i32 { a + b };
        assert_eq!(add(1, 2), 3);

        let number_one = 6;
        let number_two = 10;

        let my_closure = |x: i32| println!("{}", number_one + number_two + x);
        my_closure(5);
    }

    #[test]
    fn t_iter_collection_to_hashmap() {
        let some_numbers = vec![0, 1, 2, 3, 4, 5]; // a Vec<i32>
        let some_words = vec!["zero", "one", "two", "three", "four", "five"]; // a Vec<&str>

        let number_word_hashmap = some_numbers
            .clone()
            .into_iter()
            .zip(some_words.clone().into_iter())
            .collect::<HashMap<_, _>>();
        println!(
            "For key {} we get {}.",
            2,
            number_word_hashmap.get(&2).unwrap()
        );

        let number_word_hashmap2: HashMap<_, _> = some_numbers
            .into_iter()
            .zip(some_words.into_iter())
            .collect();
        println!(
            "For key {} we get {}.",
            2,
            number_word_hashmap.get(&2).unwrap()
        );
    }
}

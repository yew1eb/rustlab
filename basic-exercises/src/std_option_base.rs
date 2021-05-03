/**
enum Option<T> {
    None,
    Some(T),
}
/*None 和 Some 不是类型，而是 Option<T> 类型的变体。 这表示在其他功能中，
函数不能使用 Some 或 None 作为参数，而只能使用 Option<T> 作为参数。
*/

**/
#[cfg(test)]
mod test {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(x) => Some(x + 1),
        }
    }

    #[test]
    fn tmain() {
        let some_number = Some(5);
        let some_string = Some(String::from("a string"));
        let absent_number: Option<i32> = None;

        let x: i32 = 5;
        let y: Option<i32> = Some(5);
        let mut temp = 0;
        match y {
            Some(i) => {
                temp = i;
            }
            None => println!("do nothing!"),
        }
        let sum = x + temp;
        println!("sum = {}", sum);

        let result = plus_one(y);
        match result {
            Some(i) => println!("result = {}", i),
            None => println!("do nothing!"),
        }

        if let Some(value) = plus_one(y) {
            println!("value = {}", value)
        } else {
            println!("do nothing!")
        }

        println!("Hello, world!");
    }

    #[test]
    fn t_vector_option_and_match() {
        let fruits = vec!["banana", "apple", "cocounut", "orange", "strawberry"];
        let first = fruits.get(0);
        println!("{:?}", first);

        let third = fruits.get(2);
        println!("{:?}", third);

        let non_existent = fruits.get(99);
        println!("{:?}", non_existent);

        for &index in [0, 2, 99].iter() {
            match fruits.get(index) {
                Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
                None => println!("There is no fruit! :("),
            }
        }

        for &index in [0, 2, 99].iter() {
            match fruits.get(index) {
                Some(&"coconut") => println!("Coconuts are awesome!!!"),
                Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
                None => println!("There is no fruit! :("),
            }
        }
    }

    #[test]
    #[should_panic]
    fn t_unwrap() {
        //unwrap 方法直接访问 Option 类型的内部值。 但是要小心，因为如果变体是 None，则此方法将会 panic。
        let gift = Some("candy");
        assert_eq!(gift.unwrap(), "candy");

        let empty_gift: Option<&str> = None;
        assert_eq!(empty_gift.unwrap(), "candy");
    }

    #[test]
    #[should_panic]
    fn t_expect() {
        let a = Some("value");
        assert_eq!(a.expect("fruits are healthy"), "value");

        let b: Option<&str> = None;
        b.expect("fruits are healthy");
    }

    #[test]
    fn t_expect_or() {
        assert_eq!(Some("dog").unwrap_or("cat"), "dog");
        assert_eq!(None.unwrap_or("cat"), "cat");
    }
}

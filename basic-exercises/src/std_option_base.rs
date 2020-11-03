#[cfg(test)]
mod test {

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

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(x) => Some(x + 1),
        }
    }
}

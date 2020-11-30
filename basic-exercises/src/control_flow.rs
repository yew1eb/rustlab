#[cfg(test)]
mod test {
    #[test]
    fn t_if() {
        let my_number = 5;
        if my_number % 2 == 1 && my_number > 0 {
            println!("It's a positive odd number");
        } else if my_number == 6 {
            println!("It's six")
        }
    }

    #[test]
    fn t_match() {
        let my_number = 5;
        match my_number {
            0 => println!("it's zero"),
            1 => println!("it's one"),
            _ => println!("It's some other number"),
        }

        let sky = "cloudy";
        let temperature = "warm";
        match (sky, temperature) {
            ("cloudy", "cold") => println!("It's dark and unpleasant today"),
            ("clear", "warm") => println!("It's a nice day"),
            _ => println!("Not sure what the weather is."),
        }

        let children = 5;
        let married = true;
        match (children, married) {
            (children, married) if married == false => {
                println!("Not married with {} children", children)
            }
            (children, married) if children == 0 && married == true => {
                println!("Married but no children")
            }
            _ => println!("Married? {}. Number of children: {}.", married, children),
        }

        match_number(50);
        match_number(13);
        match_number(4);
    }

    fn match_number(input: i32) {
        //use @ to give a name to the value of a match expression
        match input {
            number @ 4 => println!("{} is an unlucky number in China!", number),
            number @ 13 => println!("{} is an unlucky number in North America!", number),
            _ => println!("looks like a normal number"),
        }
    }
}

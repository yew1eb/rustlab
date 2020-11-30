#[cfg(test)]
mod test {
    #[test]
    fn t_loop() {
        let mut counter = 0;
        let mut counter2 = 0;

        'first_loop: loop {
            counter += 1;
            println!("The counter is now: {}", counter);
            if counter > 9 {
                println!("Now entering the second loop.");

                'second_loop: loop {
                    println!("The second counter is now: {}", counter2);
                    counter2 += 1;
                    if counter2 == 3 {
                        break 'first_loop;
                    }
                }
            }
        }
    }

    #[test]
    fn t_range_loop() {
        for number in 0..3 {
            //[0,3)
            println!("The number is: {}", number);
        }

        for number in 0..=3 {
            //[0,3]
            println!("The next number is: {}", number);
        }

        for _number in 0..3 {
            //[0,3)
            println!("Printing the same thing three times");
        }
    }

    #[test]
    fn t_return_loop() {
        let mut counter = 5;
        let my_number = loop {
            counter += 1;
            if counter % 32 == 3 {
                break counter;
            }
        };

        println!("{}", my_number);
    }
}

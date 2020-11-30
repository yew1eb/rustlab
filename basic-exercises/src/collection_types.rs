#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_arrays() {
        // [] Arrays, The type of an array is: [type; number]
        // must not change their size
        // must only contain the same type
        let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let three_to_five = &array_of_ten[2..5];
        let start_at_two = &array_of_ten[1..];
        let end_at_five = &array_of_ten[..5];
        let everything = &array_of_ten[..];

        println!(
            "Three to five: {:?}, start at two: {:?}, end at five: {:?}, everything: {:?}",
            three_to_five, start_at_two, end_at_five, everything
        );
    }

    #[test]
    fn t_vectors() {
        let vect_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let three_to_five = &vect_of_ten[2..5];
        let start_at_two = &vect_of_ten[1..];
        let end_at_five = &vect_of_ten[..5];
        let everything = &vect_of_ten[..];

        println!(
            "Three to five: {:?}, start_at_two: {:?} end at five: {:?} everything: {:?}",
            three_to_five, start_at_two, end_at_five, everything
        )
    }

    #[test]
    fn t_vectors_capacity() {
        let mut num_vec = Vec::new();
        println!("{}", num_vec.capacity());
        num_vec.push("a");
        println!("{}", num_vec.capacity()); //Vecs with 1 item always start with capacity 4
        num_vec.push("b");
        num_vec.push("c");
        num_vec.push("d");
        println!("{}", num_vec.capacity()); //elements: still prints 4
        num_vec.push("e");
        println!("{}", num_vec.capacity()); //prints 8. We have 5 elements. but it doubled 4 to 8 to make space

        let mut num_vec2 = Vec::with_capacity(8); // Give it capacity 8
        num_vec2.push("x");
        println!("{}", num_vec2.capacity());
    }

    #[test]
    fn t_tuples() {
        let str_vec = vec!["one", "two", "three"];
        let (a, b, _) = (str_vec[0], str_vec[1], str_vec[2]);
        println!("{:?}", b)
    }
}

use std::collections::VecDeque;

fn check_remaining(input: &VecDeque<(&str, bool)>) {
    for item in input {
        if item.1 == false {
            println!("You must: {}", item.0);
        }
    }
}

fn done(input: &mut VecDeque<(&str, bool)>) {
    let mut task_done = input.pop_back().unwrap(); // pop off the back
    task_done.1 = true; // now it's done - mark as true
    input.push_front(task_done); // put it at the front now
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_vecdeque() {
        let mut my_vecdeque = VecDeque::new();
        let things_to_do = vec![
            "send email to customer",
            "add new product to list",
            "phone Loki back",
        ];

        for thing in things_to_do {
            my_vecdeque.push_front((thing, false));
        }

        done(&mut my_vecdeque);
        done(&mut my_vecdeque);

        check_remaining(&my_vecdeque);

        for task in my_vecdeque {
            println!("{:?} ", task);
        }
    }
}

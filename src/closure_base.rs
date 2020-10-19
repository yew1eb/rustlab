fn counter(i: i32) -> Box<Fn(i32) -> i32> {
    Box::new(move |n: i32| n + i)
}

/**
FnOnce调用参数为self,这意味着它会转移方法接收者的所有权。换句话说，就是这种方法调用只能被调用一次。
FnMut调用参数为&mut self,这意味着它会对方法接收者进行可变借用。
Fn调用为&self,这意味着它会对方法接收者进行不可变借用，也就是说，这种方法调用可以被调用多次。
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tmain() {
        let f = counter(3);
        assert_eq!(4, f(1));

        let add = |a: i32, b: i32| -> i32 { a + b };
        assert_eq!(add(1, 2), 3);
    }
}

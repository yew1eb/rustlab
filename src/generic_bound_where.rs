/*
限定也可以使用 where 从句来表达，这样可以让限定写在 { 紧邻的前面，而不需写在类型第一次提到的位置上。
    另外 where 从句可以用于任意类型的限定，而不局限于类型参量。
*/

use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`
// 或使用另一种间接的方法。
impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn tmain() {
        let vec = vec![1, 2, 3];

        vec.print_in_option();
    }
}

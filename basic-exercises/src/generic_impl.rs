struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let x = Val { val: 3.0 };
        let y = GenVal { gen_val: 3i32 };

        println!("{}, {}", x.value(), y.value());

        let z = 4;
        assert_eq!(2 + 2, z);
    }

    #[test]
    #[should_panic]
    fn failing_test() {
        assert!(1i32 == 2i32);
    }
}

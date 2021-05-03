/*
你可以继续使用自有类型，使用克隆等，如果你想暂时避免它们。

很多时候，当编译器想要lifetime的时候，你只要在这里和那里写上<'a>就可以了。
这只是一种 "别担心，我不会给你任何不够长寿的东西 "的说法。

你可以每次只探索一下生命期。写一些拥有值的代码，然后把一个代码变成一个引用。
编译器会开始抱怨，但也会给出一些建议。如果它变得太复杂，你可以撤销它，下次再试。
*/
#[cfg(test)]
mod test {
    #[test]
    // 下面使用连线来标注各个变量的生命周期的创建和销毁。
    // `i` 的生命周期最长，因为它的作用域完全覆盖了 `borrow1` 和
    // `borrow2` 两者。`borrow1` 和 `borrow2` 的周期没有关联，
    // 因为它们各不相交。
    fn base() {
        let i = 3; // `i` 的生命周期开始。─────────────────────┐
                   //                                        │
        {
            //                                        │
            //                                                   │
            let borrow1 = &i; // `borrow1` 的生命周期开始。 ──┐│
                              //                                                ││
            println!("borrow1: {}", borrow1); //              ││
        } // `borrow1` 结束。─────────────────────────────────┘│
          //                                                     │
          //                                                     │
        {
            //                                                     │
            //                                                   │
            let borrow2 = &i; // `borrow2` 生命周期开始。─────┐│
                              //                                ││
            println!("borrow2: {}", borrow2); //                ││
        } // `borrow2` 结束。─────────────────────────────────┘│
          //                                                     │
    } // `i` 生命周期结束。 ─────────────────────────────────────┘

    #[test]
    fn explicit() {
        // 生命周期 `'a` 和 `'b`。这两个生命周期都必须至少要和 `print_refs`
        // 函数的一样长。
        fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
            println!("x is {} and y is {}", x, y);
        }

        // 不带参量的函数，不过有一个生命周期参量 `'a`。
        fn failed_borrow<'a>() {
            let _x = 12;

            // 报错：`_x` 存活时间长度不够（`_x` does not live long enough）
            //let y: &'a i32 = &_x;
            // 尝试使用生命周期 `'a` 作为函数内部的显式类型标注将导致失败，因为
            // `&_x` 的生命周期比 `y` 的短。短生命周期不能强制转换成长生命周期。
        }

        // 创建变量，给下面代码借用。
        let (four, nine) = (4, 9);

        // 两个变量的借用（`&`）都传进函数。
        print_refs(&four, &nine);
        // 任何借用得来的输入量都必须比借入者“活”得更长。
        // 也就是说，`four` 和 `nine` 的生命周期都必须比 `print_refs`
        // 的长。

        failed_borrow();
        // `failed_borrow` 未包含引用来迫使 `'a` 长于函数的生命周期，
        // 但 `'a` 寿命更长。因为该生命周期从未被约束，所以默认为 `'static`。
    }

    #[test]
    fn t_lifttime_error() {
        let magic1 = String::from("abracadabra!");
        let result = &magic1;
        {
            let magic2 = String::from("shazam!");
            //result = longest_word(&magic1, &magic2);
            //`magic2` does not live long enough
            //longest_word 函数返回的引用的生存期与传入的引用的生存期中的较小者相同
        }
        println!("The longest magic word is {}", result);
    }

    fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    #[test]
    fn t_struct_lifttime() {
        #[derive(Debug)]
        struct Highlight<'document>(&'document str);
        //我们使用名为 'document 的生存期对我们的结构进行了批注。
        //此批注是一个提醒，它提醒 Highlight 结构的生存期不能超过它借用的 &str 的源（一个假定的文档）的生存期。

        fn erase(_: String) {}

        let text = String::from("The quick brown fox jumps over the lazy dog.");
        let fox = Highlight(&text[4..19]);
        let dog = Highlight(&text[35..43]);

        erase(text);

        //println!("{:?}", fox);
        //println!("{:?}", dog);
    }
}

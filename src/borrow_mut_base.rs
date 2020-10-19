#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` 是一个指向分配在只读内存区的字符串的引用
    author: &'static str,
    title: &'static str,
    year: u32,
}
// 此函数接受一个指向图书 Book 的引用
fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

// 此函数接受一个指向可变的图书 Book 的引用，同时把年份 `year` 改为 2004 年
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tmain() {
        // 创建一个名为 `immutabook` 的不可变的图书 Book
        let immutabook = Book {
            // 字符串字面量拥有 `&'static str` 类型
            author: "Douglas Hofstadter",
            title: "Gödel, Escher, Bach",
            year: 1979,
        };
        // 创建一个 `immutabook` 的可变拷贝，命名为 `mutabook`
        let mut mutabook = immutabook;

        // 不可变地借用一个不可变对象
        borrow_book(&immutabook);

        // 不可变地借用一个可变对象
        borrow_book(&mutabook);

        // 借用一个可变对象作为可变类型
        new_edition(&mut mutabook);

        // 报错！不能借用一个不可变对象来充当可变类型
        //new_edition(&mut immutabook);
        // 改正 ^ 注释掉此行
    }

    #[test]
    fn borrow_freeze() {
        //当数据被不可变地借用时，它还会冻结（freeze）。已冻结（frozen）数据无法通过原始对象来修改，直到指向这些数据的所有引用离开作用域为止。
        let mut _mutable_integer = 7i32;

        {
            let _large_integer = &_mutable_integer;
            // 报错！`_mutable_integer` 在本作用域被冻结
            _mutable_integer = 50;
            // 改正 ^ 注释掉此行
        }

        _mutable_integer = 3;
    }

    #[test]
    fn borrow_alias() {
        //数据可以进行多次不可变借用，但是在不可变借用的期间，原始数据不可进行可变借用。
        //也就是说，在同一段时间内只允许单独一个可变借用。原始数据在可变引用离开作用域之后可再次被借用。
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let mut point = Point { x: 0, y: 0, z: 0 };

        {
            let borrowed_point = &point;
            let another_borrow = &point;

            // 通过引用和原始所有者来访问数据
            println!(
                "Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z
            );

            // 报错！不能借用 `point` 作为可变内容，因为目前已被借用成为
            // 不可变内容。
            //let mutable_borrow = &mut point;
            // 动手试一试 ^ 将此行注释去掉。

            // 不可变引用离开作用域
        }

        {
            let mutable_borrow = &mut point;

            // 通过可变引用来改变数据
            mutable_borrow.x = 5;
            mutable_borrow.y = 2;
            mutable_borrow.z = 1;

            // 报错！不能借用 `point` 作为不可变内容，因为目前它已被借用成为
            // 可变内容。
            //let y = &point.y;
            // 动手试一试 ^ 将此行注释去掉。

            // 报错！不能打印，因为 `println!` 接受了一个不可变引用。
            //println!("Point Z coordinate is {}", point.z);
            // 动手试一试 ^ 将此行注释去掉。

            // 好！可变引用可以作为不可变的传给 `println!`。
            println!(
                "Point has coordinates: ({}, {}, {})",
                mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
            );

            // 可变引用离开作用域
        }

        // `point` 的不可变引用再次可用。
        let borrowed_point = &point;
        println!(
            "Point now has coordinates: ({}, {}, {})",
            borrowed_point.x, borrowed_point.y, borrowed_point.z
        );
    }

    #[test]
    fn borrow_ref() {
        //在通过 let 绑定来进行模式匹配或解构时，ref 关键字可用来接受结构体/元组的字段的引用。
        #[derive(Clone, Copy)]
        struct Point {
            x: i32,
            y: i32,
        }

        let c = 'Q';

        // 赋值语句中左边的 `ref` 关键字等价右边的 `&` 符号。
        let ref ref_c1 = c;
        let ref_c2 = &c;

        println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

        let point = Point { x: 0, y: 0 };

        // 在解构一个结构体时 `ref` 同样有效。
        let _copy_of_x = {
            // `ref_to_x` 是一个指向 `point` 的 `x` 字段的引用。
            let Point {
                x: ref ref_to_x,
                y: _,
            } = point;
            // 返回一个 `point` 的 `x` 字段的拷贝。
            *ref_to_x
        };

        // `point` 的可变拷贝
        let mut mutable_point = point;
        {
            // `ref` 可以结合 `mut` 来接受可变引用。
            let Point {
                x: _,
                y: ref mut mut_ref_to_y,
            } = mutable_point;

            // 通过可变引用来改变 `mutable_point` 的字段 `y`。
            *mut_ref_to_y = 1;
        }
        println!("point is ({}, {})", point.x, point.y);
        println!(
            "mutable_point is ({}, {})",
            mutable_point.x, mutable_point.y
        );

        // 包含一个指针的可变元组
        let mut mutable_tuple = (Box::new(5u32), 3u32);
        {
            // 解构 `mutable_tuple` 来改变 `last` 的值。
            let (_, ref mut last) = mutable_tuple;
            *last = 2u32;
        }

        println!("tuple is {:?}", mutable_tuple);
    }
}

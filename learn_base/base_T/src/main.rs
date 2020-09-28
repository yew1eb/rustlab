fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: PartialOrd + Copy> (list: &[T]) -> T{
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}



enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}


//-----------在方法中使用泛型--------------
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn creat_point<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    let number_list = vec![1, 2, 23, 34, 8, 100];
    let max_number = largest(&number_list);
    println!("max_number = {}", max_number);


    let integer = Point{x: 1, y: 2};
    println!("{:#?}", integer);

    let p = Point{x:1.1, y: 2.2};
    println!("x = {}", p.get_x());
    println!("y = {}", p.get_y());

    let p1 = Point2{x: 5, y: 1.1};
    let p2 = Point2{x: "hello", y: 'c'};
    let p3 = p1.creat_point(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}



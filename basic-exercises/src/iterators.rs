/*
.iter() for an iterator of references
.iter_mut() for an iterator of mutable references
.into_iter() for an iterator of values (not references)

A for loop is actually just an iterator that uses .iter_mut().
That's why you can change the values when you use it.
*/

#[derive(Debug, Clone)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>,
}

#[derive(Debug, Clone)]
enum LibraryType {
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }

    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            books: Vec::new(),
        }
    }
}
impl Iterator for Library {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.books.pop() {
            Some(book) => Some(book + " is found!"), // Rust allows String + &str
            None => None,
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_iterators() {
        let vector1 = vec![1, 2, 3];
        let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
        let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();

        let mut vector2 = vec![10, 20, 30];
        vector2.iter_mut().for_each(|x| *x += 100);

        println!("{:?}", vector1_a);
        println!("{:?}", vector2);
        println!("{:?}", vector1_b);
        /*
        The first two we used a method called .map().
        This method lets you do something to every item, then pass it on.
        The last one we used is one called .for_each().
        This method just lets you do something to every item.
        */
    }

    #[test]
    fn t_impl_iterator() {
        let mut my_library = Library::new();
        my_library.add_book("The xa");
        my_library.add_book("The xb");
        my_library.add_book("中华");
        my_library.add_book("xxx");

        for item in my_library.clone() {
            println!("{}", item);
        }
    }
}

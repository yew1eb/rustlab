struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

struct City {
    name: String,
    name_before: String,
    population: u32,
    date_founded: u32,
}

impl City {
    fn new(name: String, name_before: String, population: u32, date_founded: u32) -> Self {
        Self {
            name,
            name_before,
            population,
            date_founded,
        }
    }
}

fn process_city_values(city: &City) {
    let City {
        name,
        name_before,
        population,
        date_founded,
    } = city;

    let two_names = vec![name, name_before];
    println!("The city's two names are {:?}", two_names);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_destructuring() {
        let papa_doc = Person {
            name: "Papa Dco".to_string(),
            real_name: "Clarence".to_string(),
            height: 170,
            happiness: false,
        };

        let Person {
            // destructure papa_doc
            name: a,
            real_name: b,
            height: c,
            happiness: d,
        } = papa_doc;

        println!(
            "They call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
            a, b, c, d
        );
    }

    #[test]
    fn t_destructuring_function() {
        let tallinn = City::new("Tallinn".to_string(), "Reval".to_string(), 426_538, 1219);
        process_city_values(&tallinn);
    }
}

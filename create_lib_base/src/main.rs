use mylib::factory::produce_refrigerator as pr;
use mylib::factory::produce_washing_machine;

//use mylib::factory::*;

fn main() {
    mod_practice1();

    mod_practice2();

    mod_practice3();
}

fn mod_practice1() {
    mylib::factory::produce_refrigerator::produce_re();
    pr::produce_re();

    produce_washing_machine::produce_re();
}

fn mod_practice2() {
    mod modA {
        #[derive(Debug)]
        pub struct A {
            pub number: i32,
            name: String,
        }

        impl A {
            pub fn new_a() -> A {
                A {
                    number: 1,
                    name: String::from("A"),
                }
            }

            pub fn print_a(&self) {
                println!("number: {}, name: {}", self.number, self.name);
            }
        }

        pub mod modB {
            pub fn print_B() {
                println!("B");
            }

            pub mod modC {
                pub fn print_C() {
                    println!("C");
                    super::print_B();
                }
            }
        }
    }

    use modA::A as A1;
    //let a = modA::A::new_a();
    //let a = A::new_a();
    let a = A1::new_a();
    a.print_a();

    let number = a.number;
    //let name = a.name;

    println!("+++++++++++++");
    modA::modB::modC::print_C();
    println!("Hello, world!");

}


use crypto::digest::Digest;
use crypto::sha3::Sha3;
fn mod_practice3() {
    let mut hasher = Sha3::sha3_256();
    hasher.inpu_str("hello, world");
    let result = hasher.result_str();
    println!("hash = {}", result);
}
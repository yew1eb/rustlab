#![allow(dead_code)]
#![warn(unused_variables)]
#![warn(unused_imports)]

mod basic_type;
mod closure_base;
mod enum_base;
mod error_base;
mod define_error_type;
mod borrow_base;
mod borrow_mut_base;
mod fn_base;
mod generic_base;
mod lifetime_base;
mod macros_base;


mod struct_base;
mod trait_base;
mod generic_bound;
mod generic_bound_where;
mod generic_impl;
mod override_add_operator;


mod std_box_base;
mod std_vector_base;
mod std_string_base;
mod std_option_base;
mod std_result_base;
mod std_hashmap_base;
mod std_hashset_base;

fn main() {
    println!("Hello World!")
}

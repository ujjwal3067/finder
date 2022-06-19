#![allow(dead_code)]
mod core; 
mod parser; 


fn main() {
    let arguments = parser::parse_arguments();
    println!("arguments are : {:?}" , arguments);
}

mod countries;
mod animals;

use countries::{print_africa, print_asia};
use animals::mammal::print_domestic;

fn main() {
    println!("Hello, world!");
    print_africa();
    print_asia();
    print_domestic();
}

// mod countries;
mod animals;

use animals::mammals::{print_domestic_animal, print_wild_animal};

fn main() {
    print_domestic_animal();
    print_wild_animal();
}

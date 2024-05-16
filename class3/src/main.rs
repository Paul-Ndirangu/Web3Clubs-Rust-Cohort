// mod countries;
mod animals;
mod world;

use animals::mammals::{print_domestic_animal, print_wild_animal};
use world::countries::{print_africa, print_asia};

fn main() {
    print_domestic_animal();
    print_wild_animal();
    print_africa();
    print_asia();
}

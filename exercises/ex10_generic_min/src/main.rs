use traits_and_generics_a_generic_min_function::min_value;

mod pair;

use pair::Pair;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let int_min = min_value(&number_list);
    match int_min {
        Some(min) => println!("Min: {}", min),
        None => println!("No minimum found")
    }

    let fp_list = vec![673.345, 425.642, 134.654];
    let fp_min = min_value(&fp_list);
    match fp_min {
        Some(min) => println!("Min: {}", min),
        None => println!("No minimum found")
    }

    let pair_one = Pair::new(5, 4);
    let pair_two = Pair::new(22,5);
    let pair_three = Pair::new(13, 6);

    let pair_list = vec![pair_one, pair_two, pair_three];
    let pair_min = min_value(&pair_list);
    match pair_min {
        Some(min) => println!("Min: {}", min),
        None => println!("No minimum found")
    }

}

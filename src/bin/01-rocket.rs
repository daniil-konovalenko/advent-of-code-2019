use std::fs;

fn main() {
    let module_masses = read_numbers("inputs/01-rocket.txt");
    let total_fuel_mass: i64 = module_masses
        .into_iter()
        .map(|m| calc_fuel_with_fuel(m))
        .sum();

    dbg!(total_fuel_mass);
}

fn calc_fuel(module_mass: i64) -> i64 {
    module_mass / 3 - 2
}

fn calc_fuel_with_fuel(module_mass: i64) -> i64 {
    let module_fuel = calc_fuel(module_mass);
    let mut total_fuel = module_fuel;

    let mut fuel_for_fuel = calc_fuel(module_fuel);
    while fuel_for_fuel > 0 {
        total_fuel += fuel_for_fuel;
        fuel_for_fuel = calc_fuel(fuel_for_fuel);
    }

    total_fuel
}

fn read_numbers(filename: &str) -> Vec<i64> {
    let contents = fs::read_to_string(filename).expect("error reading file");
    let numbers: Vec<_> = contents
        .split_whitespace()
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .collect();
    return numbers;
}

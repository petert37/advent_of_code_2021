use std::fs;

const INPUT_SIZE: i32 = 12;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect::<Vec<u16>>();

    let mut gamma_rate: u16 = 0;

    for i in 0..INPUT_SIZE {
        let bits = count_bits(&input, i);
        let bit = (if bits.0 > bits.1 { 0x0 } else { 0x1 }) << i;
        gamma_rate |= bit;
    }

    let epsilon_rate = !gamma_rate & 0x0fff;

    println!(
        "Gamma rate: {0:016b}={0}, Epsilon rate: {1:016b}={1}, Result: {2}",
        gamma_rate,
        epsilon_rate,
        gamma_rate as u32 * epsilon_rate as u32
    );

    let oxygen_generator_rating = filter_oxygen_generator_rating(&input);
    let co2_scrubber_rating = filter_co2_scrubber_rating(&input);

    println!(
        "Oxygen generator rating: {0:012b}={0}, CO2 scrubber rating: {1:012b}={1}, Life support rating: {2}",
        oxygen_generator_rating,
        co2_scrubber_rating,
        oxygen_generator_rating as u32 * co2_scrubber_rating as u32
    );
}

fn count_bits(input: &[u16], n: i32) -> (i32, i32) {
    let mask = 0x01 << n;
    let mut one_bits = 0;
    let mut zero_bits = 0;
    for num in input {
        let zero = *num & mask == 0;
        if zero {
            zero_bits += 1;
        } else {
            one_bits += 1
        }
    }
    return (zero_bits, one_bits);
}

fn filter_oxygen_generator_rating(input: &Vec<u16>) -> u16 {
    filter_rating(input, |bits| if bits.0 > bits.1 { 0 } else { 1 })
}

fn filter_co2_scrubber_rating(input: &Vec<u16>) -> u16 {
    filter_rating(input, |bits| if bits.0 > bits.1 { 1 } else { 0 })
}

fn filter_rating(input: &Vec<u16>, bit_selector: fn((i32, i32)) -> i32) -> u16 {
    let mut numbers = input.clone();

    for i in (0..INPUT_SIZE).rev() {
        let bits = count_bits(&numbers, i);
        let bit = bit_selector(bits);
        let mask = 0x01 << i;
        numbers = numbers
            .iter()
            .copied()
            .filter(|num| (if *num & mask == 0 { 0 } else { 1 }) == bit)
            .collect();
        if numbers.len() == 1 {
            return numbers[0];
        }
    }
    panic!("Result not found");
}

use aoc03::{input, BinNumberList};

fn get_life_support_rating(input: &str) -> u32 {
    let mut list: BinNumberList = input.parse().unwrap();
    let mut list2 = list.clone();
    let number_len = list[0].len();

    for index in 0..number_len {
        let most_common = list.get_most_common();
        list.retain(|bin_number| bin_number[index] == most_common[index]);
        if list.len() == 1 {
            break;
        }
    }

    for index in 0..number_len {
        let least_common = list2.get_most_common().not();
        list2.retain(|bin_number| bin_number[index] == least_common[index]);
        if list2.len() == 1 {
            break;
        }
    }

    let oxygen_generator_rating = list[0].to_decimal();
    let co2_scrubber_rating = list2[0].to_decimal();

    oxygen_generator_rating * co2_scrubber_rating
}

fn main() {
    println!("{}", get_life_support_rating(input::USER));
}

#[test]
fn test_example() {
    assert_eq!(230, get_life_support_rating(input::EXAMPLE));
}

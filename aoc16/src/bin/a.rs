use aoc16::{input, BitIterator, Packet};

fn sum_versions(input: &'static str) -> usize {
    let mut bit_iter = BitIterator::from(input);
    let packet = Packet::from(&mut bit_iter);
    packet.sum_versions()
}

fn main() {
    println!("{}", sum_versions(input::USER));
}

#[test]
fn test_example1() {
    assert_eq!(16, sum_versions("8A004A801A8002F478"));
}

#[test]
fn test_example2() {
    assert_eq!(12, sum_versions("620080001611562C8802118E34"));
}

#[test]
fn test_example3() {
    assert_eq!(23, sum_versions("C0015000016115A2E0802F182340"));
}

#[test]
fn test_example4() {
    assert_eq!(31, sum_versions("A0016C880162017C3686B18A3D4780"));
}

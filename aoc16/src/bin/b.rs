use aoc16::{input, BitIterator, Packet};

fn evaluate(input: &'static str) -> usize {
    let mut bit_iter = BitIterator::from(input);
    let packet = Packet::from(&mut bit_iter);
    packet.evaluate()
}

fn main() {
    println!("{}", evaluate(input::USER));
}

#[test]
fn test_example1() {
    assert_eq!(3, evaluate("C200B40A82"));
}

#[test]
fn test_example2() {
    assert_eq!(54, evaluate("04005AC33890"));
}

#[test]
fn test_example3() {
    assert_eq!(7, evaluate("880086C3E88112"));
}

#[test]
fn test_example4() {
    assert_eq!(9, evaluate("CE00C43D881120"));
}

#[test]
fn test_example5() {
    assert_eq!(1, evaluate("D8005AC2A8F0"));
}

#[test]
fn test_example6() {
    assert_eq!(0, evaluate("F600BC2D8F"));
}

#[test]
fn test_example7() {
    assert_eq!(0, evaluate("9C005AC2F8F0"));
}

#[test]
fn test_example8() {
    assert_eq!(1, evaluate("9C0141080250320F1802104A08"));
}

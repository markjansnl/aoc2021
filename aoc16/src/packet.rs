use crate::BitIterator;

#[derive(Debug)]
pub struct Packet {
    version: usize,
    type_id: usize,
    content: PacketContent,
}

#[derive(Debug)]
pub enum PacketContent {
    Literal(usize),
    SubPackets(Vec<Packet>),
}

impl Packet {
    pub fn from(bit_iter: &mut BitIterator) -> Self {
        let version = bit_iter.take_number(3);
        let type_id = bit_iter.take_number(3);
        let content = PacketContent::from(bit_iter, type_id);

        Packet {
            version,
            type_id,
            content,
        }
    }

    pub fn sum_versions(&self) -> usize {
        match &self.content {
            PacketContent::Literal(_) => self.version,
            PacketContent::SubPackets(sub_packets) => {
                self.version
                    + sub_packets
                        .iter()
                        .map(|sub_packet| sub_packet.sum_versions())
                        .sum::<usize>()
            }
        }
    }

    pub fn evaluate(&self) -> usize {
        match &self.content {
            PacketContent::Literal(literal) => *literal,
            PacketContent::SubPackets(sub_packets) => match self.type_id {
                0 => sub_packets
                    .iter()
                    .map(|sub_packet| sub_packet.evaluate())
                    .sum(),
                1 => sub_packets
                    .iter()
                    .map(|sub_packet| sub_packet.evaluate())
                    .product(),
                2 => sub_packets
                    .iter()
                    .map(|sub_packet| sub_packet.evaluate())
                    .min()
                    .unwrap(),
                3 => sub_packets
                    .iter()
                    .map(|sub_packet| sub_packet.evaluate())
                    .max()
                    .unwrap(),
                5 => {
                    if sub_packets[0].evaluate() > sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if sub_packets[0].evaluate() < sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if sub_packets[0].evaluate() == sub_packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

impl PacketContent {
    fn from(bit_iter: &mut BitIterator, type_id: usize) -> Self {
        match type_id {
            4 => {
                let mut literal = 0;
                let mut has_more = true;
                while has_more {
                    has_more = bit_iter.next().unwrap();
                    literal = (literal << 4) + bit_iter.take_number(4);
                }
                PacketContent::Literal(literal)
            }
            _ => {
                let mut sub_packets = Vec::new();
                if bit_iter.next().unwrap() {
                    let number_of_sub_packets = bit_iter.take_number(11);
                    for _ in 0..number_of_sub_packets {
                        sub_packets.push(Packet::from(bit_iter));
                    }
                } else {
                    let total_length_in_bits = bit_iter.take_number(15);
                    let end = bit_iter.bits_read() + total_length_in_bits;
                    while bit_iter.bits_read() < end {
                        sub_packets.push(Packet::from(bit_iter));
                    }
                }
                PacketContent::SubPackets(sub_packets)
            }
        }
    }
}

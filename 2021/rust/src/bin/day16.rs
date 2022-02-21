use std::convert::TryFrom;
use std::str::FromStr;

use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;

use aoc::aoc_error::AOCError;
use aoc::utils::get_input;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = get_input()?;
    let packet = input.trim().parse()?;

    println!("Part 1: {}", part1(&packet));
    println!("Part 2: {}", part2(&packet));
    Ok(())
}

fn part1(packet: &Packet) -> u64 {
    version_number_sum(packet)
}

fn part2(packet: &Packet) -> u64 {
    packet.value()
}

fn version_number_sum(packet: &Packet) -> u64 {
    let mut result = packet.version as u64;
    for child in packet.children() {
        result += version_number_sum(child);
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    version: PacketVersion,
    packet_type: PacketType,
    payload: PacketPayload,
}

impl Packet {
    fn from_bits_with_length(bits: &BitSlice<u8, Msb0>) -> Result<(Self, usize)> {
        let version = bits[..3].load_be::<u8>();
        let packet_type = PacketType::try_from(bits[3..6].load_be::<u8>())?;
        let (payload, payload_length) = match packet_type {
            PacketType::Literal => PacketPayload::literal_from_bits(&bits[6..])?,
            _ => PacketPayload::operator_from_bits(&bits[6..])?,
        };

        Ok((
            Packet {
                version,
                packet_type,
                payload,
            },
            payload_length + 6,
        ))
    }

    fn children(&self) -> Vec<&Self> {
        match &self.payload {
            PacketPayload::Literal(_) => vec![],
            PacketPayload::Operator(packets) => packets.iter().collect(),
        }
    }

    fn value(&self) -> u64 {
        let values: Vec<_> = self.children().iter().map(|child| child.value()).collect();
        match self.packet_type {
            PacketType::Sum => values.into_iter().sum(),
            PacketType::Product => values.into_iter().product(),
            PacketType::Min => values.into_iter().min().unwrap(),
            PacketType::Max => values.into_iter().max().unwrap(),
            PacketType::Literal => match self.payload {
                PacketPayload::Literal(value) => value,
                PacketPayload::Operator(_) => unreachable!("Operator payload on Literal"),
            },
            PacketType::GreaterThan => (values[0] > values[1]) as u64,
            PacketType::LessThan => (values[0] < values[1]) as u64,
            PacketType::EqualTo => (values[0] == values[1]) as u64,
        }
    }
}

impl FromStr for Packet {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let is_hex = s
            .chars()
            .all(|c| ('0'..='9').contains(&c) || ('A'..='F').contains(&c));
        if !is_hex || s.len() % 2 != 0 {
            return Err(Box::new(AOCError::new("Invalid input.")));
        }
        let v: Vec<u8> = s
            .as_bytes()
            .chunks(2)
            .map(|s| {
                let s = std::str::from_utf8(s).unwrap();
                u8::from_str_radix(s, 16).unwrap()
            })
            .collect();

        let bits = bitvec::vec::BitVec::from_vec(v);
        Ok(Self::from_bits_with_length(&bits)?.0)
    }
}

type PacketVersion = u8;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketPayload {
    Literal(u64),
    Operator(Vec<Packet>),
}

impl PacketPayload {
    fn literal_from_bits(bits: &BitSlice<u8, Msb0>) -> Result<(Self, usize)> {
        let mut value: BitVec<u64, Msb0> = BitVec::new();
        for (i, chunk) in bits.chunks(5).enumerate() {
            if chunk.len() != 5 {
                return Err(Box::new(AOCError::new("Invalid Literal bit length")));
            }
            value.extend_from_bitslice(&chunk[1..]);
            if !chunk[0] {
                let value = value.load::<u64>();
                return Ok((PacketPayload::Literal(value), (i + 1) * 5));
            }
        }
        panic!("Unexpected end of Literal");
    }

    fn operator_from_bits(bits: &BitSlice<u8, Msb0>) -> Result<(Self, usize)> {
        let mut packets = vec![];
        match bits.get(0).map(|bitref| *bitref) {
            Some(true) => {
                // next 11 bits are number of subpackets
                if bits.len() < 12 {
                    return Err(Box::new(AOCError::new("Invalid operator")));
                }
                let packet_count = bits[1..12].load_be::<u16>();
                let mut bit_length = 12;
                for _ in 0..packet_count {
                    let (packet, packet_length) =
                        Packet::from_bits_with_length(&bits[bit_length..])?;
                    bit_length += packet_length;
                    packets.push(packet);
                }
                Ok((PacketPayload::Operator(packets), bit_length))
            }
            Some(false) => {
                // next 15 bits are bit length of subpackets
                if bits.len() < 16 {
                    return Err(Box::new(AOCError::new("Invalid operator")));
                }
                let packets_length = bits[1..16].load_be::<usize>();
                let mut bit_length = 16;
                while bit_length < packets_length + 16 {
                    let (packet, packet_length) =
                        Packet::from_bits_with_length(&bits[bit_length..])?;
                    bit_length += packet_length;
                    packets.push(packet);
                }
                if bit_length != packets_length + 16 {
                    return Err(Box::new(AOCError::new("Incorrect packet length")));
                }
                Ok((PacketPayload::Operator(packets), packets_length + 16))
            }
            None => Err(Box::new(AOCError::new("Empty operator payload"))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketType {
    Sum,
    Product,
    Min,
    Max,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl TryFrom<u8> for PacketType {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(PacketType::Sum),
            1 => Ok(PacketType::Product),
            2 => Ok(PacketType::Min),
            3 => Ok(PacketType::Max),
            4 => Ok(PacketType::Literal),
            5 => Ok(PacketType::GreaterThan),
            6 => Ok(PacketType::LessThan),
            7 => Ok(PacketType::EqualTo),
            _ => Err(Box::new(AOCError::new("Invalid packet type"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_sum_1() {
        let packet = "8A004A801A8002F478".parse().unwrap();
        let sum = version_number_sum(&packet);

        assert_eq!(sum, 16);
    }

    #[test]
    fn version_sum_2() {
        let packet = "620080001611562C8802118E34".parse().unwrap();
        let sum = version_number_sum(&packet);

        assert_eq!(sum, 12);
    }

    #[test]
    fn version_sum_3() {
        let packet = "C0015000016115A2E0802F182340".parse().unwrap();
        let sum = version_number_sum(&packet);

        assert_eq!(sum, 23);
    }

    #[test]
    fn version_sum_4() {
        let packet = "A0016C880162017C3686B18A3D4780".parse().unwrap();
        let sum = version_number_sum(&packet);

        assert_eq!(sum, 31);
    }

    #[test]
    fn value_1() {
        let packet: Packet = "C200B40A82".parse().unwrap();
        assert_eq!(packet.value(), 3);
    }

    #[test]
    fn value_2() {
        let packet: Packet = "04005AC33890".parse().unwrap();
        assert_eq!(packet.value(), 54);
    }

    #[test]
    fn value_3() {
        let packet: Packet = "880086C3E88112".parse().unwrap();
        assert_eq!(packet.value(), 7);
    }

    #[test]
    fn value_4() {
        let packet: Packet = "CE00C43D881120".parse().unwrap();
        assert_eq!(packet.value(), 9);
    }

    #[test]
    fn value_5() {
        let packet: Packet = "D8005AC2A8F0".parse().unwrap();
        assert_eq!(packet.value(), 1);
    }

    #[test]
    fn value_6() {
        let packet: Packet = "F600BC2D8F".parse().unwrap();
        assert_eq!(packet.value(), 0);
    }

    #[test]
    fn value_7() {
        let packet: Packet = "9C005AC2F8F0".parse().unwrap();
        assert_eq!(packet.value(), 0);
    }

    #[test]
    fn value_8() {
        let packet: Packet = "9C0141080250320F1802104A08".parse().unwrap();
        assert_eq!(packet.value(), 1);
    }
}

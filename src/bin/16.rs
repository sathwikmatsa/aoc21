use nom::bits::complete::take;
use nom::combinator::map;
use nom::sequence::tuple;
use std::path::Path;

type Input<'a> = (&'a [u8], usize);
type Result<'a, T> = nom::IResult<Input<'a>, T, ()>;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Header {
    version: u8,
    type_id: u8,
}

impl Header {
    fn new(version: u8, type_id: u8) -> Self {
        Self { version, type_id }
    }

    fn parse(i: Input) -> Result<Self> {
        map(tuple((take(3_usize), take(3_usize))), |(version, id)| {
            Self::new(version, id)
        })(i)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Packet {
    header: Header,
    length: usize,
    kind: PacketVariant,
}

impl Packet {
    fn parse(i: Input) -> Result<Self> {
        let initial_offset = i.1;
        let (input_ah, header) = Header::parse(i)?;
        let (input_pv, packet) = PacketVariant::parse(input_ah, header.type_id)?;
        let final_offset = (i.0.len() - input_pv.0.len()) * 8 + input_pv.1;

        Ok((
            input_pv,
            Self {
                header,
                length: final_offset - initial_offset,
                kind: packet,
            },
        ))
    }

    fn decode(&self) -> usize {
        match &self.kind {
            PacketVariant::Literal(x) => *x,
            PacketVariant::Operator(packets) => {
                let decoded = packets.iter().map(|p| p.decode()).collect::<Vec<_>>();
                match self.header.type_id {
                    0 => decoded.iter().sum::<usize>(),
                    1 => decoded.iter().product::<usize>(),
                    2 => *decoded.iter().min().unwrap(),
                    3 => *decoded.iter().max().unwrap(),
                    5 => match decoded[0] > decoded[1] {
                        true => 1,
                        false => 0,
                    },
                    6 => match decoded[0] < decoded[1] {
                        true => 1,
                        false => 0,
                    },
                    7 => match decoded[0] == decoded[1] {
                        true => 1,
                        false => 0,
                    },
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum PacketVariant {
    Literal(usize),
    Operator(Vec<Packet>),
}

impl PacketVariant {
    fn parse_literal(i: Input) -> Result<Self> {
        let mut groups = Vec::new();
        let mut input = i;
        loop {
            let out = take::<_, u8, _, _>(5_usize)(input)?;
            input = out.0;
            let five_bits = out.1;

            let group = five_bits & 0b1111;
            groups.push(group);
            if five_bits & (1 << 4) == 0 {
                return Ok((
                    input,
                    Self::Literal(
                        groups
                            .iter()
                            .rev()
                            .enumerate()
                            .fold(0, |acc, (idx, g)| acc | (*g as usize) << (4 * idx)),
                    ),
                ));
            }
        }
    }

    fn parse_operator(i: Input) -> Result<Self> {
        let (mut input, length_type_id) = take::<_, u8, _, _>(1_usize)(i)?;
        if length_type_id == 0 {
            let out_lt = take::<_, usize, _, _>(15_usize)(input)?;
            input = out_lt.0;
            let total_sub_packets_len = out_lt.1;
            let mut packets = Vec::new();
            loop {
                let out_p = Packet::parse(input)?;
                input = out_p.0;
                let packet = out_p.1;
                packets.push(packet);
                if packets.iter().map(|p| p.length).sum::<usize>() == total_sub_packets_len {
                    return Ok((input, Self::Operator(packets)));
                }
            }
        } else {
            let out_lt = take::<_, usize, _, _>(11_usize)(input)?;
            input = out_lt.0;
            let total_sub_packets = out_lt.1;
            let mut packets = Vec::new();
            loop {
                let out_p = Packet::parse(input)?;
                input = out_p.0;
                let packet = out_p.1;
                packets.push(packet);
                if packets.len() == total_sub_packets {
                    return Ok((input, Self::Operator(packets)));
                }
            }
        }
    }

    fn parse(input: Input, id: u8) -> Result<Self> {
        match id {
            4 => Self::parse_literal(input),
            _ => Self::parse_operator(input),
        }
    }
}

fn main() {
    let bytes = read_hex_file("input/16.txt");
    let (_, top_packet) = Packet::parse((&bytes, 0)).unwrap();
    println!("part 1: {}", add_versions(&top_packet));
    println!("part 2: {}", top_packet.decode());
}

fn add_versions(packet: &Packet) -> usize {
    let mut sum = packet.header.version as usize;
    if let PacketVariant::Operator(sub_packets) = &packet.kind {
        sum += sub_packets.iter().map(|p| add_versions(p)).sum::<usize>();
    }
    sum
}

fn hex_string_to_bytes(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

fn read_hex_file(path: impl AsRef<Path>) -> Vec<u8> {
    let s = std::fs::read_to_string(path).unwrap();
    hex_string_to_bytes(s.as_str())
}

#[allow(dead_code)]
fn bytes_to_binary_string(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:08b}", byte)).collect()
}

#[cfg(test)]
mod problem16 {
    use super::*;

    #[test]
    fn hex() {
        let bytes = hex_string_to_bytes("D2FE28");
        assert_eq!(bytes_to_binary_string(&bytes), "110100101111111000101000");
    }

    #[test]
    fn literal() {
        let bytes = hex_string_to_bytes("D2FE28");
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(packet.header.version, 6);
        assert_eq!(packet.header.type_id, 4);
        assert_eq!(packet.length, 21);
        assert_eq!(packet.kind, PacketVariant::Literal(2021));
    }

    #[test]
    fn operator_0() {
        let bytes = hex_string_to_bytes("38006F45291200");
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(packet.header.version, 1);
        assert_eq!(packet.header.type_id, 6);
        assert_eq!(packet.length, 49);
        if let PacketVariant::Operator(packets) = packet.kind {
            assert_eq!(packets.len(), 2);
            assert_eq!(packets[0].kind, PacketVariant::Literal(10));
            assert_eq!(packets[1].kind, PacketVariant::Literal(20));
        } else {
            panic!("not operator variant");
        }
    }

    #[test]
    fn operator_1() {
        let bytes = hex_string_to_bytes("EE00D40C823060");
        let (_, packet) = Packet::parse((&bytes, 0)).unwrap();
        assert_eq!(packet.header.version, 7);
        assert_eq!(packet.header.type_id, 3);
        assert_eq!(packet.length, 51);
        if let PacketVariant::Operator(packets) = packet.kind {
            assert_eq!(packets.len(), 3);
            assert_eq!(packets[0].kind, PacketVariant::Literal(1));
            assert_eq!(packets[1].kind, PacketVariant::Literal(2));
            assert_eq!(packets[2].kind, PacketVariant::Literal(3));
        } else {
            panic!("not operator variant");
        }
    }

    #[test]
    fn part1() {
        macro_rules! add_versions_test {
            ($hex:expr, $sum:expr) => {
                let bytes = hex_string_to_bytes($hex);
                let (_, top_packet) = Packet::parse((&bytes, 0)).unwrap();
                assert_eq!($sum, add_versions(&top_packet));
            };
        }

        add_versions_test!("A0016C880162017C3686B18A3D4780", 31);
        add_versions_test!("8A004A801A8002F478", 16);
        add_versions_test!("620080001611562C8802118E34", 12);
        add_versions_test!("C0015000016115A2E0802F182340", 23);
    }

    #[test]
    fn part2() {
        macro_rules! decode_test {
            ($hex:expr, $val:expr) => {
                let bytes = hex_string_to_bytes($hex);
                let (_, top_packet) = Packet::parse((&bytes, 0)).unwrap();
                assert_eq!($val, top_packet.decode());
            };
        }

        decode_test!("C200B40A82", 3);
        decode_test!("04005AC33890", 54);
        decode_test!("880086C3E88112", 7);
        decode_test!("CE00C43D881120", 9);
        decode_test!("D8005AC2A8F0", 1);
        decode_test!("F600BC2D8F", 0);
        decode_test!("9C005AC2F8F0", 0);
        decode_test!("9C0141080250320F1802104A08", 1);
    }
}

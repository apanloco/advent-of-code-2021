use crate::error;

#[derive(PartialEq, Debug)]
pub enum TypeId {
    Literal,
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl TypeId {
    fn from_type_id(type_id: usize) -> Self {
        match type_id {
            0 => TypeId::Sum,
            1 => TypeId::Product,
            2 => TypeId::Minimum,
            3 => TypeId::Maximum,
            4 => TypeId::Literal,
            5 => TypeId::GreaterThan,
            6 => TypeId::LessThan,
            7 => TypeId::EqualTo,
            _ => {
                panic!("invalid type id: {}", type_id);
            }
        }
    }
}

#[derive(Debug)]
pub struct Transmission {
    pub digits: String,
    left: Vec<char>,
}

#[derive(Debug)]
pub struct Packet {
    pub version: usize,
    type_id: TypeId,
    value: usize,
    num_sub_packet_bits: usize,
    num_sub_packets: usize,
}

impl std::str::FromStr for Transmission {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits: String = s
            .trim_start()
            .trim_end()
            .chars()
            .map(|c| format!("{:04b}", u64::from_str_radix(&format!("{}", c), 16).unwrap()))
            .collect();

        Ok(Transmission {
            digits: digits.to_string(),
            left: digits.chars().collect(),
        })
    }
}

fn process_operation(packet: &Packet, value_packets: &Vec<Packet>) -> Packet {
    let values: Vec<usize> = value_packets.iter().map(|p| p.value).collect();

    let result = match packet.type_id {
        TypeId::Sum => values.iter().sum(),
        TypeId::Product => values.iter().product(),
        TypeId::Minimum => *values.iter().min().unwrap(),
        TypeId::Maximum => *values.iter().max().unwrap(),
        TypeId::GreaterThan => {
            if values[0] > values[1] {
                1
            } else {
                0
            }
        }
        TypeId::LessThan => {
            if values[0] < values[1] {
                1
            } else {
                0
            }
        }
        TypeId::EqualTo => {
            if values[0] == values[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("invalid operation: {:?}", packet.type_id),
    };

    let mut num_sub_packet_bits = value_packets.iter().map(|p| p.num_sub_packet_bits).sum::<usize>() + 3 + 3 + 1;
    if packet.num_sub_packet_bits > 0 {
        num_sub_packet_bits += 15;
    } else {
        num_sub_packet_bits += 11;
    }

    Packet {
        version: 0,
        type_id: TypeId::Literal,
        value: result,
        num_sub_packet_bits,
        num_sub_packets: 1,
    }
}

pub fn process_packets(mut packets: Vec<Packet>) -> usize {
    let mut stack: Vec<Packet> = Vec::new();

    loop {
        let packet = packets.pop().unwrap();
        if packet.type_id == TypeId::Literal {
            stack.push(packet)
        } else {
            let mut operation_values: Vec<Packet> = Vec::new();
            loop {
                if (packet.num_sub_packet_bits > 0 && operation_values.iter().map(|p| p.num_sub_packet_bits).sum::<usize>() == packet.num_sub_packet_bits)
                    || (packet.num_sub_packets > 0 && operation_values.iter().map(|p| p.num_sub_packets).sum::<usize>() == packet.num_sub_packets)
                {
                    break;
                }
                operation_values.push(stack.pop().unwrap());
            }
            stack.push(process_operation(&packet, &operation_values));
        }

        if packets.is_empty() {
            break;
        }
    }

    if stack.len() != 1 {
        panic!("problem with algorithm");
    }

    stack[0].value
}

impl Transmission {
    fn consume_bits_to_int(&mut self, num_bits: usize) -> Option<usize> {
        self.consume_bits_to_string(num_bits).map(|binary_string| usize::from_str_radix(&binary_string, 2).unwrap())
    }

    fn consume_bits_to_string(&mut self, num_bits: usize) -> Option<String> {
        if self.left.len() < num_bits {
            return None;
        }

        let substring = self.left.drain(0..num_bits).collect();

        Some(substring)
    }

    fn consume_packet_type_operator(&mut self, packet: &mut Packet) {
        let length_type_id = self.consume_bits_to_int(1).unwrap();
        match length_type_id {
            0 => {
                packet.num_sub_packet_bits = self.consume_bits_to_int(15).unwrap();
            }
            1 => {
                packet.num_sub_packets = self.consume_bits_to_int(11).unwrap();
            }
            _ => {
                panic!("invalid length type id: {}", length_type_id);
            }
        }
    }

    fn consume_packet_type_literal(&mut self, packet: &mut Packet) {
        let mut binary_string = String::new();

        loop {
            let not_last_bit = self.consume_bits_to_int(1).unwrap();
            binary_string += &self.consume_bits_to_string(4).unwrap();
            if not_last_bit == 0 {
                break;
            }
        }

        packet.num_sub_packets = 1;
        packet.num_sub_packet_bits = ((binary_string.len() / 4) * 5) + 6;
        packet.value = usize::from_str_radix(&binary_string, 2).unwrap();
    }
}

impl Iterator for Transmission {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left.len() < 8 {
            return None;
        }

        let version = self.consume_bits_to_int(3)?;
        let type_id = self.consume_bits_to_int(3)?;

        let mut packet = Packet {
            version,
            type_id: TypeId::from_type_id(type_id),
            value: 0,
            num_sub_packet_bits: 0,
            num_sub_packets: 0,
        };

        match packet.type_id {
            TypeId::Literal => self.consume_packet_type_literal(&mut packet),
            _ => self.consume_packet_type_operator(&mut packet),
        }

        Some(packet)
    }
}

#[test]
fn test_day16_utils() {
    assert_eq!(format!("{:04b}", 7), "0111");
}

#[test]
fn test_day16_part1() -> Result<(), error::Error> {
    let transmission: Transmission = "D2FE28".parse()?;
    assert_eq!(transmission.digits, "110100101111111000101000");
    let packets: Vec<Packet> = transmission.collect();
    assert_eq!(packets.len(), 1);
    assert_eq!(packets[0].version, 6);
    assert_eq!(packets[0].type_id, TypeId::Literal);
    assert_eq!(packets[0].value, 2021);

    let transmission: Transmission = "38006F45291200".parse()?;
    assert_eq!(transmission.digits, "00111000000000000110111101000101001010010001001000000000");
    let packets: Vec<Packet> = transmission.collect();
    assert_eq!(packets.len(), 3);

    let transmission: Transmission = "EE00D40C823060".parse()?;
    assert_eq!(transmission.digits, "11101110000000001101010000001100100000100011000001100000");
    let packets: Vec<Packet> = transmission.collect();
    assert_eq!(packets.len(), 4);

    let transmission: Transmission = "8A004A801A8002F478".parse()?;
    let packets: Vec<Packet> = transmission.collect();
    assert_eq!(packets.len(), 4);
    assert_eq!(packets.iter().map(|p| p.version).sum::<usize>(), 16);

    let transmission: Transmission = "620080001611562C8802118E34".parse()?;
    let packets: Vec<Packet> = transmission.collect();
    assert_eq!(packets.len(), 7);
    assert_eq!(packets.iter().map(|p| p.version).sum::<usize>(), 12);

    let transmission: Transmission = "C0015000016115A2E0802F182340".parse()?;
    let packets: Vec<Packet> = transmission.collect();
    assert_eq!(packets.len(), 7);
    assert_eq!(packets.iter().map(|p| p.version).sum::<usize>(), 23);

    let transmission: Transmission = "A0016C880162017C3686B18A3D4780".parse()?;
    let packets: Vec<Packet> = transmission.collect();
    assert_eq!(packets.len(), 8);
    assert_eq!(packets.iter().map(|p| p.version).sum::<usize>(), 31);

    let transmission: Transmission = std::fs::read_to_string("input_day16")?.parse()?;
    let packets: Vec<Packet> = transmission.collect();
    assert_eq!(packets.len(), 268);
    assert_eq!(packets.iter().map(|p| p.version).sum::<usize>(), 999);

    Ok(())
}

#[test]
fn test_day16_part2() -> Result<(), error::Error> {
    let transmission: Transmission = "D2FE28".parse()?;
    assert_eq!(process_packets(transmission.collect()), 2021);

    let transmission: Transmission = "EE00D40C823060".parse()?;
    assert_eq!(process_packets(transmission.collect()), 3);

    let transmission: Transmission = "620080001611562C8802118E34".parse()?;
    assert_eq!(process_packets(transmission.collect()), 46);

    let transmission: Transmission = "C200B40A82".parse()?;
    assert_eq!(process_packets(transmission.collect()), 3);

    let transmission: Transmission = "04005AC33890".parse()?;
    assert_eq!(process_packets(transmission.collect()), 54);

    let transmission: Transmission = "880086C3E88112".parse()?;
    assert_eq!(process_packets(transmission.collect()), 7);

    let transmission: Transmission = "CE00C43D881120".parse()?;
    assert_eq!(process_packets(transmission.collect()), 9);

    let transmission: Transmission = "D8005AC2A8F0".parse()?;
    assert_eq!(process_packets(transmission.collect()), 1);

    let transmission: Transmission = "F600BC2D8F".parse()?;
    assert_eq!(process_packets(transmission.collect()), 0);

    let transmission: Transmission = "9C005AC2F8F0".parse()?;
    assert_eq!(process_packets(transmission.collect()), 0);

    let transmission: Transmission = "9C0141080250320F1802104A08".parse()?;
    assert_eq!(process_packets(transmission.collect()), 1);

    let transmission: Transmission = std::fs::read_to_string("input_day16")?.parse()?;
    assert_eq!(process_packets(transmission.collect()), 3408662834145);

    Ok(())
}

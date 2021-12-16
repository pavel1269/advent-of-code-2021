use self::binary_reader::BinaryReader;

pub mod binary_reader;

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let packet = Packet::from(input);
    let result = packet.version_sum();
    return result;
}

struct Packet {
    version: u8,
    sub_packets: Vec<Packet>,
}

impl Packet {
    pub fn version_sum(&self) -> i64 {
        let sum: i64 = self.sub_packets.iter().map(|p| p.version_sum()).sum();
        return sum + self.version as i64;
    }

    pub fn from(input: &str) -> Packet {
        let mut binary_reader = binary_reader::BinaryReader::from(input);
        let packet = Packet::parse_packet(&mut binary_reader);
        return packet;
    }

    fn parse_packet(binary_reader: &mut BinaryReader) -> Packet {
        let version = binary_reader.take::<u8>(3);
        let type_id = binary_reader.take::<u8>(3);

        let mut sub_packets = Vec::new();
        match type_id {
            4 => {
                let mut is_not_last = true;
                while is_not_last {
                    is_not_last = binary_reader.take::<u8>(1) == 1;

                    // ignore number
                    binary_reader.take::<usize>(4);
                }
            },
            _ => {
                let number_of_packets = binary_reader.take::<u8>(1) == 1;
                if number_of_packets {
                    let size = binary_reader.take::<usize>(11);
                    
                    for _index in 0..size {
                        let sub_packet = Packet::parse_packet(binary_reader);
                        sub_packets.push(sub_packet);
                    }
                } else {
                    let size = binary_reader.take::<usize>(15);
                    let mut read = binary_reader.get_read_bits();
                    let boundary = read + size;
                    while read < boundary {
                        let sub_packet = Packet::parse_packet(binary_reader);
                        sub_packets.push(sub_packet);
                        read = binary_reader.get_read_bits();
                    }
                }
            },
        }

        return Packet {
            version: version,
            sub_packets: sub_packets,
        };
    }
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_1_input() -> &'static str {
        return "D2FE28";
    }

    fn get_example_2_input() -> &'static str {
        return "38006F45291200";
    }

    fn get_example_3_input() -> &'static str {
        return "EE00D40C823060";
    }

    fn get_example_4_input() -> &'static str {
        return "8A004A801A8002F478";
    }

    fn get_example_5_input() -> &'static str {
        return "620080001611562C8802118E34";
    }

    fn get_example_6_input() -> &'static str {
        return "C0015000016115A2E0802F182340";
    }

    fn get_example_7_input() -> &'static str {
        return "A0016C880162017C3686B18A3D4780";
    }

    #[test]
    fn example1_part1_correct_result() {
        let packet = Packet::from(get_example_1_input());
        let result = packet.version_sum();
        assert_eq!(6, result);
    }

    #[test]
    fn example2_part1_correct_result() {
        let packet = Packet::from(get_example_2_input());
        let result = packet.version_sum();
        assert_eq!(9, result);
    }

    #[test]
    fn example3_part1_correct_result() {
        let packet = Packet::from(get_example_3_input());
        let result = packet.version_sum();
        assert_eq!(14, result);
    }

    #[test]
    fn example4_part1_correct_result() {
        let packet = Packet::from(get_example_4_input());
        let result = packet.version_sum();
        assert_eq!(16, result);
    }

    #[test]
    fn example5_part1_correct_result() {
        let packet = Packet::from(get_example_5_input());
        let result = packet.version_sum();
        assert_eq!(12, result);
    }

    #[test]
    fn example6_part1_correct_result() {
        let packet = Packet::from(get_example_6_input());
        let result = packet.version_sum();
        assert_eq!(23, result);
    }

    #[test]
    fn example7_part1_correct_result() {
        let packet = Packet::from(get_example_7_input());
        let result = packet.version_sum();
        assert_eq!(31, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(936, result);
    }
}

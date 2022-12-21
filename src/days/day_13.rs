use std::{iter::Peekable, str::Chars};

use super::DaySolver;

#[derive(Debug, Clone)]
enum PacketData {
    Number(i32),
    Packet(Vec<PacketData>),
}

impl PartialEq<PacketData> for &PacketData {
    fn eq(&self, other: &PacketData) -> bool {
        (*self).eq(other)
    }
}

impl PartialEq<&PacketData> for PacketData {
    fn eq(&self, other: &&PacketData) -> bool {
        self.eq(*other)
    }
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PacketData::Number(num1), PacketData::Number(num2)) => num1.eq(num2),
            (PacketData::Number(_), PacketData::Packet(v2)) => vec![self].eq(v2),
            (PacketData::Packet(v1), PacketData::Packet(v2)) => v1.eq(v2),
            (PacketData::Packet(v1), PacketData::Number(_)) => v1.eq(&vec![other]),
        }
    }
}

impl PartialOrd<PacketData> for &PacketData {
    fn partial_cmp(&self, other: &PacketData) -> Option<std::cmp::Ordering> {
        (*self).partial_cmp(other)
    }
}

impl PartialOrd<&PacketData> for PacketData {
    fn partial_cmp(&self, other: &&PacketData) -> Option<std::cmp::Ordering> {
        self.partial_cmp(*other)
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (PacketData::Number(num1), PacketData::Number(num2)) => num1.partial_cmp(num2),
            (PacketData::Number(_), PacketData::Packet(_)) => {
                PacketData::Packet(vec![self.clone()]).partial_cmp(other)
            }
            (PacketData::Packet(_), PacketData::Number(_)) => {
                self.partial_cmp(&PacketData::Packet(vec![other.clone()]))
            }
            (PacketData::Packet(v1), PacketData::Packet(v2)) => v1.partial_cmp(v2),
        }
    }
}

struct Packet {
    data: PacketData,
}

fn parse_data_packets(iter: &mut Peekable<Chars>) -> Vec<PacketData> {
    let mut data = Vec::new();
    while let Some(next) = iter.next() {
        match next {
            ',' => continue,
            '[' => data.push(PacketData::Packet(parse_data_packets(iter))),
            ']' => return data,
            c if c.is_numeric() => {
                let mut s = String::new();
                s.push(c);
                while let Some(c) = iter.peek() {
                    if c.is_numeric() {
                        s.push(*c);
                        iter.next();
                    } else {
                        break;
                    }
                }
                data.push(PacketData::Number(s.parse::<i32>().unwrap()));
            }
            _ => panic!("OOF"),
        }
    }
    panic!("Did not find ending bracket")
}

impl Packet {
    fn parse(s: &str) -> Self {
        let mut iter = s.chars().peekable();
        // skip hight level '['
        iter.next();
        Self {
            data: PacketData::Packet(parse_data_packets(&mut iter)),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl Eq for Packet {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Solver;

impl DaySolver for Solver {
    fn q2(&self, input_txt: &str) -> String {
        let mut lines = input_txt.lines().peekable();
        let mut packets = Vec::new();
        loop {
            let first_packet = Packet::parse(lines.next().unwrap());
            let second_packet = Packet::parse(lines.next().unwrap());
            packets.push(first_packet);
            packets.push(second_packet);
            if lines.peek().is_some() {
                lines.next();
            } else {
                break;
            }
        }
        packets.push(Packet::parse("[[2]]"));
        packets.push(Packet::parse("[[6]]"));
        packets.sort();
        let idx1 = packets.iter().enumerate().find(|p| {
            if let PacketData::Packet(v) = &p.1.data {
                if v.len() != 1 {
                    return false;
                }
                if let PacketData::Packet(v) = &v[0] {
                    if v.len() != 1 {
                        return false;
                    }
                    if let PacketData::Number(n) = &v[0] {
                        if *n == 2 {
                            return true;
                        }
                    }
                }
            }
            false
        });
        let idx2 = packets.iter().enumerate().find(|p| {
            if let PacketData::Packet(v) = &p.1.data {
                if v.len() != 1 {
                    return false;
                }
                if let PacketData::Packet(v) = &v[0] {
                    if v.len() != 1 {
                        return false;
                    }
                    if let PacketData::Number(n) = &v[0] {
                        if *n == 6 {
                            return true;
                        }
                    }
                }
            }
            false
        });

        ((idx1.unwrap().0 + 1) * (idx2.unwrap().0 + 1)).to_string()
    }

    fn q1(&self, input_txt: &str) -> String {
        let mut lines = input_txt.lines().peekable();
        let mut packets = Vec::new();
        loop {
            let first_packet = Packet::parse(lines.next().unwrap());
            let second_packet = Packet::parse(lines.next().unwrap());
            packets.push((first_packet, second_packet));
            if lines.peek().is_some() {
                lines.next();
            } else {
                break;
            }
        }
        packets
            .iter()
            .enumerate()
            .filter(|(_, (p1, p2))| p1 <= p2)
            .fold(0, |acc, (idx, _)| acc + (idx + 1))
            .to_string()
    }
}

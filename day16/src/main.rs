use hex;

fn main() {
    let mut bs = BitStream::new(include_str!("input.txt").trim());
    let p = bs.packet();
    println!(
        "sum_version: {}, eval:{}",
        p.sum_version_numbers(),
        p.eval()
    );
}

type Number = u64;
type Position = usize;
type Version = Number;

#[derive(Debug, PartialEq)]
struct Packet {
    version: Version,
    payload: Payload,
}

#[derive(Debug, PartialEq)]
enum Payload {
    Literal(Number),
    Operator(Vec<Packet>, Operation),
}
use Payload::*;

impl Packet {
    fn sum_version_numbers(&self) -> Number {
        self.version
            + if let Operator(ps, _) = &self.payload {
                ps.iter().map(|p| p.sum_version_numbers()).sum()
            } else {
                0
            }
    }
    fn eval(&self) -> Number {
        match &self.payload {
            Literal(n) => *n,
            Operator(ps, o) => o.eval(ps.iter().map(|p| p.eval()).collect::<Vec<Number>>()),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}
use Operation::*;

impl Operation {
    fn from(n: Number) -> Operation {
        match n {
            0 => Sum,
            1 => Product,
            2 => Minimum,
            3 => Maximum,
            5 => GreaterThan,
            6 => LessThan,
            7 => EqualTo,
            _ => unreachable!(),
        }
    }
    fn eval(&self, ns: Vec<Number>) -> Number {
        match self {
            Sum => ns.iter().sum(),
            Product => ns.iter().product(),
            Minimum => *ns.iter().min().unwrap(),
            Maximum => *ns.iter().max().unwrap(),
            GreaterThan => bool2bit(ns[0] > ns[1]),
            LessThan => bool2bit(ns[0] < ns[1]),
            EqualTo => bool2bit(ns[0] == ns[1]),
        }
    }
}

fn bool2bit(b: bool) -> Number {
    if b {
        1
    } else {
        0
    }
}

#[derive(Debug)]
struct BitStream {
    pos: Position,
    chars: Vec<char>,
}

impl BitStream {
    fn packet(&mut self) -> Packet {
        Packet {
            version: self.n(3),
            payload: match self.n(3) {
                4 => self.literal(),
                pt => {
                    let children = self.children();
                    Operator(children, Operation::from(pt))
                }
            },
        }
    }

    fn literal(&mut self) -> Payload {
        let mut value = 0;
        loop {
            let n = self.n(5);
            value = value << 4 | n & 0b1111;
            if n & 0b10000 == 0 {
                break;
            }
        }
        Literal(value)
    }

    fn children(&mut self) -> Vec<Packet> {
        let mut packets = Vec::new();
        if self.flag() {
            let packet_count = self.n(11);
            self.children_packets(&mut packets, packet_count);
        } else {
            let bits = self.n(15);
            self.children_bits(&mut packets, bits as usize);
        }
        packets
    }

    fn children_bits(&mut self, packets: &mut Vec<Packet>, bits: usize) {
        let end = self.pos + bits;
        while self.pos < end {
            packets.push(self.packet());
        }
    }

    fn children_packets(&mut self, packets: &mut Vec<Packet>, n: Number) {
        for _ in 0..n {
            packets.push(self.packet());
        }
    }

    fn flag(&mut self) -> bool {
        self.n(1) != 0
    }

    fn new(s: &str) -> Self {
        Self {
            pos: 0,
            chars: hex::decode(s)
                .unwrap()
                .iter()
                .map(|n| format!("{:08b}", n))
                .collect::<String>()
                .chars()
                .collect(),
        }
    }

    // Read next n bits as number
    fn n(&mut self, bits: usize) -> Number {
        let chunk = &self.chars[self.pos..self.pos + bits];
        self.pos += bits;
        let s: String = chunk.iter().collect();
        let n = Number::from_str_radix(&s, 2).unwrap();
        n
    }
}

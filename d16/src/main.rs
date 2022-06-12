#![feature(io_read_to_string)]
static INPUT: &'static str ="C20D718021600ACDC372CD8DE7A057252A49C940239D68978F7970194EA7CCB310088760088803304A0AC1B100721EC298D3307440041CD8B8005D12DFD27CBEEF27D94A4E9B033006A45FE71D665ACC0259C689B1F99679F717003225900465800804E39CE38CE161007E52F1AEF5EE6EC33600BCC29CFFA3D8291006A92CA7E00B4A8F497E16A675EFB6B0058F2D0BD7AE1371DA34E730F66009443C00A566BFDBE643135FEDF321D000C6269EA66545899739ADEAF0EB6C3A200B6F40179DE31CB7B277392FA1C0A95F6E3983A100993801B800021B0722243D00042E0DC7383D332443004E463295176801F29EDDAA853DBB5508802859F2E9D2A9308924F9F31700AA4F39F720C733A669EC7356AC7D8E85C95E123799D4C44C0109C0AF00427E3CC678873F1E633C4020085E60D340109E3196023006040188C910A3A80021B1763FC620004321B4138E52D75A20096E4718D3E50016B19E0BA802325E858762D1802B28AD401A9880310E61041400043E2AC7E8A4800434DB24A384A4019401C92C154B43595B830002BC497ED9CC27CE686A6A43925B8A9CFFE3A9616E5793447004A4BBB749841500B26C5E6E306899C5B4C70924B77EF254B48688041CD004A726ED3FAECBDB2295AEBD984E08E0065C101812E006380126005A80124048CB010D4C03DC900E16A007200B98E00580091EE004B006902004B00410000AF00015933223100688010985116A311803D05E3CC4B300660BC7283C00081CF26491049F3D690E9802739661E00D400010A8B91F2118803310A2F43396699D533005E37E8023311A4BB9961524A4E2C027EC8C6F5952C2528B333FA4AD386C0A56F39C7DB77200C92801019E799E7B96EC6F8B7558C014977BD00480010D89D106240803518E31C4230052C01786F272FF354C8D4D437DF52BC2C300567066550A2A900427E0084C254739FB8E080111E0";

use bitvec::prelude::*;
use hex::{FromHexError, decode};
use either::Either;

type Hexa = BitVec<u8, Msb0>;
type HexaSlice = BitSlice<u8, Msb0>;

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: usize,
    type_id: u8,
    containts: Either<Vec<Packet>, usize>,
}

impl Packet {
    fn new_non_4(version: usize, type_id: u8) -> Self {
        Self {
            version,
            type_id,
            containts: Either::Left(Vec::new()),
        }
    }

    fn new_4(version: usize, type_id: u8, value: usize) -> Self {
        Self {
            version,
            type_id,
            containts: Either::Right(value),
        }
    }

    fn extend(&mut self, packet: Vec<Packet>) {
        match &mut self.containts {
            Either::Left(v) => v.extend(packet),
            Either::Right(_) => unreachable!("A packet with value 4 should not be pushed."),
        }
    }

    fn version_sum(&self) -> usize {
        self.version + match &self.containts {
            Either::Left(packets) => packets.iter().map(Packet::version_sum).sum(),
            Either::Right(_) => 0,
        }
    }

    fn calculate(&self) -> usize {
        match &self.containts {
            Either::Left(packets) => match self.type_id {
                0 => packets.iter().map(Packet::calculate).sum(),
                1 => packets.iter().map(Packet::calculate).product(),
                2 => packets.iter().map(Packet::calculate).min().unwrap(),
                3 => packets.iter().map(Packet::calculate).max().unwrap(),
                5 => (packets[0].calculate() > packets[1].calculate()) as usize,
                6 => (packets[0].calculate() < packets[1].calculate()) as usize,
                7 => (packets[0].calculate() == packets[1].calculate()) as usize,
                n => unreachable!("Packet id: {}, is unknown.", n)
            },
            Either::Right(value) => *value,
        }
    }
}

fn parse(s: &str) -> Result<Hexa, FromHexError> {
    let decoded = decode(s)?;
    let bv = Hexa::from_slice(&decoded);
    Ok(bv)
}

fn header_version(message: &mut &HexaSlice) -> Option<(usize, u8)> {
    if message.any() {
        let header;
        (header, *message) = message.split_at(6);
        let (version, type_id) = header.split_at(3);
        Some((version.load_be(), type_id.load_be()))
    } else {
        None
    }
}

fn packet_id_4(message: &mut &HexaSlice) -> usize {
    let mut packet = BitVec::<u8, Msb0>::new();
    let mut part;
    let mut cont;
    loop {
        (part, *message) = message.split_at(5);
        (cont, part) = part.split_at(1);
        packet.extend_from_bitslice(part);
        if !cont.any() {
            break;
        }
    }
    packet.load_be()
}

fn operator_packets(message: &mut &HexaSlice) -> Vec<Packet> {
    let length_type_id;
    (length_type_id, *message) = message.split_at(1);
    match length_type_id.any() {
        true => {
            let sub_packets;
            (sub_packets, *message) = message.split_at(11);
            let nr_sub_packets: usize = sub_packets.load_be();
            let mut sub_packets = Vec::new();
            for _ in 0..nr_sub_packets {
                sub_packets.push(packets(message));
            }
            sub_packets
        },
        false => {
            let packet_len;
            (packet_len, *message) = message.split_at(15);
            let packet_len: usize = packet_len.load_be();
            let len = message.len();
            let mut sub_packets = Vec::new();
            while len - message.len() < packet_len {
                sub_packets.push(packets(message));
            }
            sub_packets
        },
    }
}

fn packets(message: &mut &HexaSlice) -> Packet {
    if let Some((version, type_id)) = header_version(message) {
        match type_id {
            4 => Packet::new_4(version, type_id, packet_id_4(message)),
            _ => {
                let mut packet = Packet::new_non_4(version, type_id);
                packet.extend(operator_packets(message));
                packet
            },
        }
    } else {
        unreachable!()
    }

}

fn part1(message: &mut &HexaSlice) -> usize {
    packets(message).version_sum()
}

fn part2(message: &mut &HexaSlice) -> usize {
    packets(message).calculate()
}


fn main() -> Result<(), FromHexError> {
    let task = parse(INPUT)?;
    let mut p1 = task.as_bitslice();
    println!("Part 1: {}", part1(&mut p1));
    let mut p2 = task.as_bitslice();
    println!("Part 2: {}", part2(&mut p2));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static T_1: &'static str = "D2FE28";

    fn parsed() -> Hexa {
        parse(T_1).unwrap()
    }


    #[test]
    fn test_parse_input() {
        assert!(parse(INPUT).is_ok())
    }

    #[test]
    fn test_parse_t_1() {
        assert!(parse(T_1).is_ok())
    }

    #[test]
    fn test_parse_t_1_correct() {
        let check = bitvec![u8, Msb0; 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0];
        assert_eq!(parse(T_1), Ok(check));
    }

    #[test]
    fn test_headers() {
        let parsed = parsed();
        let mut p = parsed.as_bitslice();
        let check = bitvec![u8, Msb0; 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0];
        let actual = header_version(&mut p).unwrap();
        let expected = (6usize, 4u8, check.as_bitslice());
        assert_eq!((actual.0, actual.1, p), expected);
    }

    #[test]
    fn test_id_4() {
        let parsed = parsed();
        let mut p = &parsed.as_bitslice()[6..];
        assert_eq!(packet_id_4(&mut p), 2021)
    }

    #[test]
    fn test_name() {
        let tests = [
            ("8A004A801A8002F478", 16),  //represents an operator packet (version 4) which contains an operator packet (version 1) which contains an operator packet (version 5) which contains a literal value (version 6); this packet has a version sum of 16.
            ("620080001611562C8802118E34", 12),  //represents an operator packet (version 3) which contains two sub-packets; each sub-packet is an operator packet that contains two literal values. This packet has a version sum of 12.
            ("C0015000016115A2E0802F182340", 23),  //has the same structure as the previous example, but the outermost packet uses a different length type ID. This packet has a version sum of 23.
            ("A0016C880162017C3686B18A3D4780", 31)  //is an operator packet that contains an operator packet that contains an operator packet that contains five literal values; it has a version sum of 31
        ];
        for (hex_string, expexted) in tests {
            let parsed = dbg!(parse(hex_string).unwrap());
            let mut r = parsed.as_bitslice();
            assert_eq!(part1(&mut r), expexted);
        }
    }
}

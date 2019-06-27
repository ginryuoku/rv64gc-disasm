enum InstFormat {
    R, I, S, U,
}

struct InstR {
    opcode: u8,
    funct3: u8,
    funct7: u8,
    rs1: u8,
    rs2: u8,
    rd: u8,
}

struct InstI {
    opcode: u8,
    funct3: u8,
    rs1: u8,
    rd: u8,
    immediate: i32,
}

struct InstS {
    opcode: u8,
    funct3: u8,
    rs1: u8,
    rs2: u8,
    immediate: i32,
}

struct InstU {
    opcode: u8,
    rd: u8,
    immediate: i32
}

enum ImmedFormat {
    I, S, B, U, J
}

struct ImmI {
}

fn get_opcode(word: u32) -> u8 {
    (word & 0b111_1111) as u8
}

fn main() {
    let word = 0x0000297;
    println!("Opcode: 0x{:x}", get_opcode(word));
}

use crate::FunctionOutput;
use crate::FunctionOutput::StringPair;

#[derive(Debug)]
struct StrangeDevice {
    a: i64,
    b: i64,
    c: i64,
}

impl StrangeDevice {
    fn combo(&self, operand: &i64) -> i64 {
        self.resolve_combo(operand.try_into().unwrap())
    }
    fn resolve_combo(&self, combo: Combo) -> i64 {
        match combo {
            Combo::Literal(val) => val,
            Combo::Register(register) => match register {
                Register::A => self.a,
                Register::B => self.b,
                Register::C => self.c,
            },
        }
    }

    fn divide(&self, operand: &i64) -> i64 {
        let denominator = 2_i64.pow(self.combo(operand) as u32);
        self.a / denominator
    }
}

enum Combo {
    Literal(i64),
    Register(Register),
}
impl TryFrom<&i64> for Combo {
    type Error = &'static str;

    fn try_from(value: &i64) -> Result<Self, Self::Error> {
        match value {
            0 | 1 | 2 | 3 => Ok(Combo::Literal(*value)),
            4 => Ok(Combo::Register(Register::A)),
            5 => Ok(Combo::Register(Register::B)),
            6 => Ok(Combo::Register(Register::C)),
            _ => Err("Combo must be between 0 and 6 inclusive"),
        }
    }
}
enum Register {
    A,
    B,
    C,
}

pub(crate) fn day17(lines: Vec<String>) -> FunctionOutput {
    let (mut device, instructions) = parse(lines);
    println!("{:?}", instructions);

    let mut output = Vec::new();
    let mut instruction_pointer: i64 = 0;
    while let (Some(opcode), Some(operand)) = (
        instructions.get(instruction_pointer as usize),
        instructions.get((instruction_pointer + 1) as usize),
    ) {
        let mut jump_to = instruction_pointer + 2;
        match opcode {
            0 => device.a = device.divide(operand),
            1 => device.b = device.b ^ operand,
            2 => device.b = device.combo(operand) % 8,
            3 => {
                if device.a != 0 {
                    jump_to = *operand
                }
            }
            4 => device.b = device.b ^ device.c,
            5 => output.push(device.combo(operand) % 8),
            6 => device.b = device.divide(operand),
            7 => device.c = device.divide(operand),
            _ => panic!(),
        }
        instruction_pointer = jump_to;
    }
    let output: Vec<_> = output.iter().map(|x| x.to_string()).collect();

    let partone = output.join(",");
    let parttwo = String::new();
    StringPair(partone, parttwo)
}

fn parse(lines: Vec<String>) -> (StrangeDevice, Vec<i64>) {
    let device = StrangeDevice {
        a: lines[0][12..].parse().unwrap(),
        b: lines[1][12..].parse().unwrap(),
        c: lines[2][12..].parse().unwrap(),
    };
    let instructions: Vec<i64> = lines[4][9..]
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect();
    (device, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_testfile;

    #[test]
    fn test_day_17_small() {
        let lines = read_testfile("day17test.txt");
        assert_eq!(
            day17(lines),
            StringPair("4,6,3,5,6,3,5,2,1,0".to_string(), "".to_string())
        );
    }

    #[test]
    fn test_day_17() {
        let lines = read_testfile("day17.txt");
        assert_eq!(
            day17(lines),
            StringPair("2,1,0,4,6,2,4,2,0".to_string(), "".to_string())
        );
    }
}

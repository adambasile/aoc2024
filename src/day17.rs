use crate::FunctionOutput;
use crate::FunctionOutput::StringPair;

#[derive(Debug, Copy, Clone)]
struct StrangeDevice {
    a: i64,
    b: i64,
    c: i64,
}

impl StrangeDevice {
    fn new_with_a(&self, a: i64) -> Self {
        Self {
            a,
            b: self.b,
            c: self.c,
        }
    }
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
    let (device, instructions) = parse(lines);

    let output = compute(device, &instructions, false).unwrap();

    let partone = output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");

    // let parttwo = find_quine(device, &instructions);
    let parttwo = "".to_string();
    StringPair(partone, parttwo)
}

fn find_quine(device: StrangeDevice, instructions: &Vec<i64>) -> String {
    todo!("doesn't work");
    // one jump to the start and it's at the end
    assert_eq!(
        vec![instructions.len() - 2],
        instructions
            .iter()
            .enumerate()
            .filter(|(i, &v)| ((i % 2) == 0) && v == 3)
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
    );
    assert_eq!(instructions.last().unwrap().clone(), 0);

    let outputs_per_loop = instructions
        .iter()
        .enumerate()
        .filter(|(i, &v)| ((i % 2) == 0) && v == 5)
        .count();

    let divides_per_loop = instructions
        .iter()
        .enumerate()
        .filter(|(i, &v)| ((i % 2) == 0) && v == 0)
        .count();

    let max_power_of_8 = (instructions.len() * divides_per_loop) / outputs_per_loop;

    let parttwo = (8_i64.pow((max_power_of_8 - 1) as u32)..8_i64.pow((max_power_of_8) as u32))
        .filter(|&a| {
            let new_device = device.new_with_a(a);
            let output = compute(new_device, &instructions, true);
            match output {
                Some(_) => true,
                None => false,
            }
        })
        .next()
        .unwrap()
        .to_string();
    parttwo
}

fn compute(device: StrangeDevice, instructions: &Vec<i64>, try_to_match: bool) -> Option<Vec<i64>> {
    let mut device = device.clone();
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
            5 => {
                let val = device.combo(operand) % 8;
                if try_to_match && (Some(&val) != instructions.get(output.len())) {
                    return None;
                }
                output.push(val);
            }
            6 => device.b = device.divide(operand),
            7 => device.c = device.divide(operand),
            _ => panic!(),
        }
        instruction_pointer = jump_to;
    }
    if try_to_match && (output.len() != instructions.len()) {
        return None;
    }
    Some(output)
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
    fn test_day_17_small1() {
        let lines = read_testfile("day17test1.txt");
        assert_eq!(
            day17(lines),
            StringPair("4,6,3,5,6,3,5,2,1,0".to_string(), "117440".to_string()),
        );
    }
    #[test]
    fn test_day_17_small2() {
        let lines = read_testfile("day17test2.txt");
        assert_eq!(
            day17(lines),
            StringPair("5,7,3,0".to_string(), "117440".to_string()),
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

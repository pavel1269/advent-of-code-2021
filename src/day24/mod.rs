use std::collections::HashSet;

pub fn get_solution_part1() -> i64 {
    let _input = get_input();
    process_instructions(_input);
    return -1;
}

#[derive(Debug, PartialEq)]
enum Value {
    Variable(usize),
    Number(i64),
}

#[derive(Debug)]
enum Instructions {
    Inp(usize),
    Add(usize, Value),
    Mul(usize, Value),
    Div(usize, Value),
    Mod(usize, Value),
    Eql(usize, Value),
}

#[derive(Debug, Clone, PartialEq)]
enum PossibleValue {
    Zero,
    Value(i64),
    Range(i64, i64, HashSet<usize>),
}

fn process_instructions(input: &str) {
    let instructions = parse_input(input);

    let mut registers = vec![PossibleValue::Zero; 4];
    let mut input_index = 0;
    let mut useful_instruction = Vec::new();
    for instruction in instructions.iter() {
        let registers_backup = registers.clone();
        let result = match instruction {
            Instructions::Inp(register) => {
                let mut inputs = HashSet::new();
                inputs.insert(input_index);
                input_index += 1;
                Some((*register, PossibleValue::Range(1, 9, inputs)))
            }
            Instructions::Add(register, value) => {
                if *value == Value::Number(0) {
                    None
                } else if let Value::Number(number) = value {
                    Some((
                        *register,
                        match &registers[*register] {
                            PossibleValue::Zero => PossibleValue::Value(*number),
                            PossibleValue::Value(a) => PossibleValue::Value(a + number),
                            PossibleValue::Range(a, b, inputs) => {
                                PossibleValue::Range(a + number, b + number, inputs.clone())
                            }
                        },
                    ))
                } else if let Value::Variable(register2) = value {
                    let value = &registers[*register];
                    let value2 = &registers[*register2];
                    match value2 {
                        PossibleValue::Zero => None,
                        PossibleValue::Value(a) => Some((
                            *register,
                            match value {
                                PossibleValue::Zero => value2.clone(),
                                PossibleValue::Value(b) => PossibleValue::Value(a + b),
                                PossibleValue::Range(b, c, inputs) => {
                                    PossibleValue::Range(a + b, a + c, inputs.clone())
                                }
                            },
                        )),
                        PossibleValue::Range(a, b, inputs) => Some((
                            *register,
                            match value {
                                PossibleValue::Zero => value2.clone(),
                                PossibleValue::Value(c) => {
                                    PossibleValue::Range(a + c, b + c, inputs.clone())
                                }
                                PossibleValue::Range(c, d, inputs2) => {
                                    let mut inputs = inputs.clone();
                                    inputs.extend(inputs2);
                                    PossibleValue::Range(a + c, b + d, inputs)
                                }
                            },
                        )),
                    }
                } else {
                    panic!();
                }
            }
            Instructions::Mul(register, value) => {
                if PossibleValue::Zero == registers[*register] {
                    None
                } else if *value == Value::Number(1) {
                    None
                } else if *value == Value::Number(0) {
                    Some((*register, PossibleValue::Zero))
                } else if let Value::Number(number) = value {
                    Some((
                        *register,
                        match &registers[*register] {
                            PossibleValue::Zero => panic!(),
                            PossibleValue::Value(a) => PossibleValue::Value(a * number),
                            PossibleValue::Range(a, b, inputs) => {
                                PossibleValue::Range(a * number, b * number, inputs.clone())
                            }
                        },
                    ))
                } else if let Value::Variable(register2) = value {
                    let value = &registers[*register];
                    let value2 = &registers[*register2];
                    match value2 {
                        PossibleValue::Zero => Some((*register, PossibleValue::Zero)),
                        PossibleValue::Value(a) => Some((
                            *register,
                            match value {
                                PossibleValue::Zero => value2.clone(),
                                PossibleValue::Value(b) => PossibleValue::Value(a * b),
                                PossibleValue::Range(b, c, inputs) => {
                                    PossibleValue::Range(a * b, a * c, inputs.clone())
                                }
                            },
                        )),
                        PossibleValue::Range(a, b, inputs) => Some((
                            *register,
                            match value {
                                PossibleValue::Zero => value2.clone(),
                                PossibleValue::Value(c) => {
                                    PossibleValue::Range(a * c, b * c, inputs.clone())
                                }
                                PossibleValue::Range(c, d, inputs2) => {
                                    let mut inputs = inputs.clone();
                                    inputs.extend(inputs2);
                                    PossibleValue::Range(a * c, b * d, inputs)
                                }
                            },
                        )),
                    }
                } else {
                    panic!();
                }
            }
            Instructions::Div(register, value) => {
                if PossibleValue::Zero == registers[*register] {
                    None
                } else if let Value::Number(number) = value {
                    Some((
                        *register,
                        match &registers[*register] {
                            PossibleValue::Zero => panic!(),
                            PossibleValue::Value(a) => PossibleValue::Value(a / number),
                            PossibleValue::Range(a, b, inputs) => {
                                PossibleValue::Range(a / number, b / number, inputs.clone())
                            }
                        },
                    ))
                } else {
                    todo!();
                }
            }
            Instructions::Mod(register, value) => {
                if PossibleValue::Zero == registers[*register] {
                    None
                } else if let Value::Number(number) = value {
                    Some((
                        *register,
                        match &registers[*register] {
                            PossibleValue::Zero => panic!(),
                            PossibleValue::Value(a) => PossibleValue::Value(a % number),
                            PossibleValue::Range(a, b, inputs) => {
                                if b < number {
                                    PossibleValue::Range(*a, *b, inputs.clone())
                                } else {
                                    PossibleValue::Range(0, number - 1, inputs.clone())
                                }
                            }
                        },
                    ))
                } else {
                    todo!();
                }
            }
            Instructions::Eql(register, value) => match value {
                Value::Number(a) => Some((
                    *register,
                    match &registers[*register] {
                        PossibleValue::Zero => {
                            if *a == 0 {
                                PossibleValue::Value(1)
                            } else {
                                PossibleValue::Zero
                            }
                        }
                        PossibleValue::Value(b) => {
                            if *a == *b {
                                PossibleValue::Value(1)
                            } else {
                                PossibleValue::Zero
                            }
                        }
                        PossibleValue::Range(b, c, inputs) => {
                            if *a >= *b && *a <= *c {
                                PossibleValue::Range(0, 1, inputs.clone())
                            } else {
                                PossibleValue::Zero
                            }
                        }
                    },
                )),
                Value::Variable(register2) => {
                    let value = &registers[*register];
                    let value2 = &registers[*register2];
                    Some((
                        *register,
                        match value2 {
                            PossibleValue::Zero => match value {
                                PossibleValue::Zero => PossibleValue::Value(1),
                                PossibleValue::Value(a) => {
                                    if *a == 0 {
                                        PossibleValue::Value(1)
                                    } else {
                                        PossibleValue::Zero
                                    }
                                }
                                PossibleValue::Range(a, b, inputs) => {
                                    if *a <= 0 && *b >= 0 {
                                        PossibleValue::Range(0, 1, inputs.clone())
                                    } else {
                                        PossibleValue::Zero
                                    }
                                }
                            },
                            PossibleValue::Value(a) => match value {
                                PossibleValue::Zero => {
                                    if *a == 0 {
                                        PossibleValue::Value(1)
                                    } else {
                                        PossibleValue::Zero
                                    }
                                }
                                PossibleValue::Value(b) => {
                                    if *a == *b {
                                        PossibleValue::Value(1)
                                    } else {
                                        PossibleValue::Zero
                                    }
                                }
                                PossibleValue::Range(b, c, inputs) => {
                                    if *b <= *a && *c >= *a {
                                        PossibleValue::Range(0, 1, inputs.clone())
                                    } else {
                                        PossibleValue::Zero
                                    }
                                }
                            },
                            PossibleValue::Range(a, b, inputs) => match value {
                                PossibleValue::Zero => {
                                    if *a <= 0 && *b >= 0 {
                                        PossibleValue::Range(0, 1, inputs.clone())
                                    } else {
                                        PossibleValue::Zero
                                    }
                                }
                                PossibleValue::Value(c) => {
                                    if *a <= *c && *b >= *c {
                                        PossibleValue::Range(0, 1, inputs.clone())
                                    } else {
                                        PossibleValue::Zero
                                    }
                                }
                                PossibleValue::Range(c, d, inputs2) => {
                                    if *a <= *d && *b >= *c {
                                        let mut inputs = inputs.clone();
                                        inputs.extend(inputs2);
                                        PossibleValue::Range(0, 1, inputs)
                                    } else {
                                        PossibleValue::Zero
                                    }
                                }
                            },
                        },
                    ))
                }
            },
        };

        match result {
            Some((register, possible_value)) => {
                *registers.get_mut(register).unwrap() = possible_value;
                useful_instruction.push((instruction.clone(), registers_backup));
            }
            None => (),
        }
    }

    println!("Program:");
    for inst in useful_instruction.iter() {
        let registers = inst
            .1
            .iter()
            .map(|val| {
                if let PossibleValue::Range(a, b, _) = val {
                    PossibleValue::Range(*a, *b, HashSet::new())
                } else {
                    val.clone()
                }
            })
            .collect::<Vec<_>>();
        println!("{:?} {:?}", inst.0, registers);
    }
    println!();

    todo!();
}

fn parse_input(input: &str) -> Vec<Instructions> {
    input
        .lines()
        .map(|input| {
            let (operation, rest) = input.split_at(3);
            let operand1 = (rest.chars().nth(1).unwrap() as u8 - 'w' as u8) as usize;

            if operation == "inp" {
                return Instructions::Inp(operand1);
            }

            let (_, operand2) = rest.split_at(3);
            let operand2_val = operand2.chars().nth(0).unwrap() as u8;
            let operand2 = if operand2_val >= 'w' as u8 && operand2_val <= 'z' as u8 {
                Value::Variable((operand2_val - 'w' as u8) as usize)
            } else {
                Value::Number(operand2.parse().unwrap())
            };

            match operation {
                "add" => Instructions::Add(operand1, operand2),
                "mul" => Instructions::Mul(operand1, operand2),
                "div" => Instructions::Div(operand1, operand2),
                "mod" => Instructions::Mod(operand1, operand2),
                "eql" => Instructions::Eql(operand1, operand2),
                _ => todo!(),
            }
        })
        .collect()
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        process_instructions(get_input());
        assert!(false);
    }
}

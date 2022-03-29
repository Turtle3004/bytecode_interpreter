use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader};
use std::str::from_utf8;

// Type of instructions supported and the expected value(optional) associated with them.
enum ByteCodeInstruction {
    LOAD_VAL(i32),
    WRITE_VAR(String),
    READ_VAR(String),
    ADD,
    SUB,
    MULTIPLY,
    DIVIDE,
    CMP(i32),
    JMP(String),
    JMP_LE(String),
    RETURN_VALUE,
}

struct byte_code {
    // Mapping of all the instructions with the optional values/variables they are associated with.
    instruction_sets: Vec<ByteCodeInstruction>,

    // Mapping of the LABEL to the number of line it should jump.
    label_to_instruction: HashMap<String, usize>,
}

impl byte_code {
    fn new() -> byte_code {
        byte_code {
            instruction_sets: Vec::<ByteCodeInstruction>::new(),
            label_to_instruction: HashMap::<String, usize>::new(),
        }
    }
    // Checking the length of the Bytecode Instructions given in the file.
    fn check_length(
        buffer: &Vec<String>,
        expected_length: usize,
    ) -> std::result::Result<(), String> {
        if buffer.len() != expected_length {
            return Err(format!(
                "Should have exactly {} parameter in {:?}",
                expected_length - 1,
                buffer
            ));
        }
        Ok(())
    }
    // Converting from any result returned from libraries to Result<T, E>
    fn convert_to_result<T, E: std::fmt::Debug>(
        res: std::result::Result<T, E>,
        message: String,
    ) -> std::result::Result<T, String> {
        match res {
            Ok(s) => Ok(s),
            Err(err) => return Err(format!("{}, Error: {:?}", message, err)),
        }
    }

    // Reading the file into byte_code -> instruction_sets and label_to_instruction.
    fn read_file_to_bytecode(&mut self, filename: String) -> std::result::Result<(), String> {
        let file = byte_code::convert_to_result(
            fs::File::open(&filename),
            format!("Unable to open file {}", filename),
        )?;
        let reader = BufReader::new(file);

        // inst_count -> For keeping the log of instruction number for the LABEL(to which line number it should jump to).
        let mut inst_count: usize = 0;
        for line in reader.lines() {
            let buffer: Vec<String> =
                byte_code::convert_to_result(line, String::from("Error in reading file"))?
                    .split_whitespace()
                    .map(str::to_string)
                    .collect();
            if buffer.is_empty() {
                continue;
            }
            let inst = buffer[0].as_str();
            inst_count = inst_count + 1;
            match inst {
                // Checking LOAD_VAL instruction:- it's length and storing the value with it in instruction_sets.
                "LOAD_VAL" => {
                    byte_code::check_length(&buffer, 2)?;

                    let value = byte_code::convert_to_result(
                        buffer[1].parse::<i32>(),
                        format!("Unable to parse {:?}", buffer),
                    )?;

                    self.instruction_sets
                        .push(ByteCodeInstruction::LOAD_VAL(value));
                }
                // Checking WRITE_VAR instruction:- it's length and storing the value(in this case variable) with it in instruction_sets.
                "WRITE_VAR" => {
                    byte_code::check_length(&buffer, 2)?;

                    let value = byte_code::convert_to_result(
                        valid_variable(buffer[1].clone()),
                        format!("Unable to parse"),
                    )?;
                    self.instruction_sets
                        .push(ByteCodeInstruction::WRITE_VAR(value));
                }
                // Checking READ_VAR instruction:- it's length and storing the value(in this case variable) with it in instruction_sets.
                "READ_VAR" => {
                    byte_code::check_length(&buffer, 2)?;

                    let value = byte_code::convert_to_result(
                        valid_variable(buffer[1].clone()),
                        format!("Unable to parse"),
                    )?;
                    self.instruction_sets
                        .push(ByteCodeInstruction::READ_VAR(value));
                }
                // Checking CMP instruction:- it's length and storing the value(in this case variable) with it in instruction_sets.
                "CMP" => {
                    byte_code::check_length(&buffer, 2)?;

                    let value = byte_code::convert_to_result(
                        buffer[1].parse::<i32>(),
                        format!("Unable to parse {:?}", buffer),
                    )?;
                    self.instruction_sets.push(ByteCodeInstruction::CMP(value));
                }
                // Checking JMP instruction:- it's length and storing the value(in this case Label) with it in instruction_sets.
                "JMP" => {
                    byte_code::check_length(&buffer, 2)?;

                    let value = byte_code::convert_to_result(
                        buffer[1].parse::<String>(),
                        format!("Unable to parse {:?}", buffer),
                    )?;
                    self.instruction_sets.push(ByteCodeInstruction::JMP(value));
                }
                // Checking JMP_LE instruction:- it's length and storing the value(in this case Label) with it in instruction_sets.
                "JMP_LE" => {
                    byte_code::check_length(&buffer, 2)?;

                    let value = byte_code::convert_to_result(
                        buffer[1].parse::<String>(),
                        format!("Unable to parse {:?}", buffer),
                    )?;
                    self.instruction_sets
                        .push(ByteCodeInstruction::JMP_LE(value));
                }

                // Storing ADD, SUB, MULTIPLY, DIVIDE and RETURN_value in instruction_sets.
                "ADD" => {
                    byte_code::check_length(&buffer, 1)?;
                    self.instruction_sets.push(ByteCodeInstruction::ADD);
                }
                "SUB" => {
                    byte_code::check_length(&buffer, 1)?;
                    self.instruction_sets.push(ByteCodeInstruction::SUB);
                }
                "MULTIPLY" => {
                    byte_code::check_length(&buffer, 1)?;
                    self.instruction_sets.push(ByteCodeInstruction::MULTIPLY);
                }
                "DIVIDE" => {
                    byte_code::check_length(&buffer, 1)?;
                    self.instruction_sets.push(ByteCodeInstruction::DIVIDE);
                }
                "RETURN_VALUE" => {
                    byte_code::check_length(&buffer, 1)?;
                    self.instruction_sets
                        .push(ByteCodeInstruction::RETURN_VALUE);
                }
                // Checking LABEL instruction:- it's length and storing the value(in this inst_count) with it in instruction_sets.
                "LABEL" => {
                    byte_code::check_length(&buffer, 2)?;

                    let value = byte_code::convert_to_result(
                        buffer[1].parse::<String>(),
                        format!("Unable to parse {:?}", buffer),
                    )?;
                    self.label_to_instruction.insert(value, inst_count);
                    inst_count = inst_count - 1;
                }
                _ => {
                    return Err(format!("Undefined Instruction! {}", inst));
                }
            }
        }

        // Check for only one RETURN_VALUE should be find in the end.
        if !self
            .instruction_sets
            .iter()
            .position(|x| match x {
                ByteCodeInstruction::RETURN_VALUE => true,
                _ => false,
            })
            .ok_or(format!("Unexpected: unable to get RETURN_VALUE!"))?
            == self.instruction_sets.len() - 1
        {
            return Err(format!("Should have exactly one RETURN_VALUE and that should be in the end of the bytecode!"));
        }
        return Ok(());
    }

    // Executing the instructions.
    fn execute(&self) -> Result<i32, String> {
        // Stack -> Given Bytecode language is stack based.
        let mut v: Vec<i32> = Vec::new();

        // Mapping the variable with its corresponding value.
        let mut variable_to_value = HashMap::<String, i32>::new();
        let mut res = 0;
        let mut inst_num: usize = 0;
        while inst_num < self.instruction_sets.len() {
            //println!("{:?}", v);
            let instruction = self.instruction_sets.get(inst_num).ok_or(format!(
                "Unexpected: unable to get the instruction for the instruction number: {}!",
                inst_num
            ))?;
            match instruction {
                // For LOAD_VAL pushing the value on the stack.
                ByteCodeInstruction::LOAD_VAL(value) => {
                    v.push(value.clone());
                }
                // For WRITE_VAR poping the value from the stack assigning it to the variable.
                ByteCodeInstruction::WRITE_VAR(value) => {
                    variable_to_value.insert(
                        value.clone(),
                        v.pop().ok_or(format!(
                            "Unexpected: unable to get the value {} for WRITE_VAR instruction!",
                            value
                        ))?,
                    );
                }
                // For READ_VAR reading the variables value and pushing it on the stack.
                ByteCodeInstruction::READ_VAR(value) => {
                    let x = variable_to_value.get(&value.clone()).ok_or(format!(
                        "Unexpected: unable to get the value from variable {}!",
                        value
                    ))?;
                    v.push(*x);
                }
                // For CMP, get the data from the stack and compare it with value.
                ByteCodeInstruction::CMP(value) => {
                    let x = v.pop().ok_or(format!(
                        "Unexpected: unable to get the value: {} for CMP instruction!",
                        value
                    ))?;
                    v.push(x - value);
                }
                // For JMP, get the instruction number.
                ByteCodeInstruction::JMP(value) => {
                    inst_num = *self
                        .label_to_instruction
                        .get(value)
                        .ok_or(format!("Unexpected: unable to get the instruction number!"))?;
                    inst_num -= 1;
                    continue;
                }
                // For JMP, get the instruction number.
                ByteCodeInstruction::JMP_LE(value) => {
                    if v.pop()
                        .ok_or(format!("Unexpected: unable to get the value!"))?
                        < 0
                    {
                        inst_num = *self
                            .label_to_instruction
                            .get(value)
                            .ok_or(format!("Unexpected: unable to get the instruction number!"))?;
                        inst_num -= 1;
                        continue;
                    }
                }
                // For ADD, SUB, MULTIPLY, DIVIDE pop two values from the stack perform +, -, *, / respectively
                // and push the result on the stack.
                ByteCodeInstruction::ADD => {
                    let x = v
                        .pop()
                        .ok_or(format!("Unexpected: unable to get the value!"))?;
                    let y = v
                        .pop()
                        .ok_or(format!("Unexpected: unable to get the value!"))?;
                    v.push(x + y);
                }
                ByteCodeInstruction::MULTIPLY => {
                    let x = v
                        .pop()
                        .ok_or(format!("Unexpected: unable to get the value!"))?;
                    let y = v
                        .pop()
                        .ok_or(format!("Unexpected: unable to get the value!"))?;
                    v.push(x * y);
                }
                ByteCodeInstruction::SUB => {
                    let x = v
                        .pop()
                        .ok_or(format!("Unexpected: unable to get the value!"))?;
                    let y = v
                        .pop()
                        .ok_or(format!("Unexpected: unable to get the value!"))?;
                    v.push(y - x);
                }
                ByteCodeInstruction::DIVIDE => {
                    let x = v
                        .pop()
                        .ok_or(format!("Unexpected: unable to get the value!"))?;
                    if x == 0 {
                        return Err(format!("Divide by 0 error!"));
                    }
                    let y = v
                        .pop()
                        .ok_or(format!("Unexpected: unable to get the value!"))?;
                    v.push(y / x);
                }
                // For RETURN_VALUE, pop the value from the stack.
                ByteCodeInstruction::RETURN_VALUE => {
                    res = v
                        .pop()
                        .ok_or(format!("Unexpected: unable to get the result!"))?;
                }
            }
            inst_num = inst_num + 1;
        }
        return Ok(res);
    }
}

// Check for a valid variable (e.g. 'x')
fn valid_variable(mut variable: String) -> Result<String, String> {
    // Length should be at least 3
    if variable.len() < 3 {
        return Err(format!("Variable length is less than 3!"));
    }
    // Considering the ASCII. (NOTE: For the sake of simplicity stright quotes is being used instead of curly)
    if !variable.is_ascii() {
        return Err(format!("Variable is not in ASCII!"));
    }
    // Removing the first and last characters of the variable and checking is they are '
    let x = variable.remove(0);
    let y = variable.pop().ok_or(format!(
        "Unexpected: unable to get the character from the variable: {}",
        variable
    ))?;
    if (x != '\'') || (y != '\'') {
        return Err(format!("Invalid character!"));
    }

    // First character of the variable
    let first_char = variable.chars().nth(0).ok_or(format!(
        "Unexpected: Unable to get first character from {}!",
        variable
    ))?;
    // first_char should be either an alphabet or _
    if !first_char.is_alphabetic() && first_char != '_' {
        return Err(format!("Variable should start with an alphabet or _ !"));
    }
    // All the characters of the variable should be either alphanumeric or _
    if !(variable.chars().all(|x| x.is_alphanumeric() || x == '_')) {
        return Err(format!("Variable should contain alphabets, numbers or _"));
    }
    variable.make_ascii_lowercase();
    return Ok(variable);
}

fn main() {
    let mut b: byte_code = byte_code::new();
    // For task 1: use "byteCode.txt" Should get answer 2.
    // For task 2(Loops): use "byteCode_loop.txt" Should get answer 45.
    match b.read_file_to_bytecode(String::from("byteCode.txt")) {
        Ok(()) => println!("Parsing successful!"),
        Err(err) => {
            println!("Unable to parse with error: {}", err);
            return;
        }
    };
    match b.execute() {
        Ok(res) => println!("Result: {}.", res),
        Err(err) => {
            println!("Error {:?}", err);
            return;
        }
    }
}

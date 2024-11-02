extern crate capstone;
use arch::x86::X86OpMem;
use arch::x86::X86Reg::{
    X86_REG_RSP,
    X86_REG_RBP,
    X86_REG_ESP,
    X86_REG_EBP,
    X86_REG_RIP,
};
use capstone::prelude::*;
use capstone::arch::x86::{
    X86Insn,
    X86OperandType
};
use capstone::arch::ArchOperand;
use capstone::{
    Insn,
    InsnId,
    Instructions
};
use std::io::{
    Error,
    ErrorKind
};
use std::collections::{
    BTreeMap, HashSet, VecDeque
};
use crate::models::block::Block;
use crate::models::function::Function;
use crate::models::binary::Binary;
use crate::models::binary::BinaryArchitecture;
use crate::models::instruction::Instruction;
use crate::models::debug::Debug;
use crate::models::cfg::CFG;

#[derive(Clone)]
pub struct DisassemblerOptions {
    pub enable_minhash: bool,
    pub minhash_maximum_byte_size: usize,
    pub minhash_number_of_hashes: usize,
    pub minhash_shingle_size: usize,
    pub minhash_seed: u64,
    pub enable_tlsh: bool,
    pub enable_sha256: bool,
    pub enable_entropy: bool,
    pub enable_feature: bool,
    pub tlsh_mininum_byte_size: usize,
    pub tags: Vec<String>,
    //pub symbols: HashMap<u64, HashSet<String>>,
}

pub struct Disassembler {
    cs: Capstone,
    image: Vec<u8>,
    machine: BinaryArchitecture,
    executable_address_ranges: BTreeMap<u64, u64>,
    options: DisassemblerOptions,
}

impl Disassembler {

    pub fn new(machine: BinaryArchitecture, image: Vec<u8>, executable_address_ranges: BTreeMap<u64, u64>, options: DisassemblerOptions) -> Result<Self, Error> {
        let cs = match Disassembler::cs_new(machine, true) {
            Ok(cs) => cs,
            Err(error) => return Err(error),
        };
        Ok(Self{
            cs: cs,
            image: image,
            machine: machine,
            executable_address_ranges: executable_address_ranges,
            options: options,
        })
    }

    pub fn is_executable_address(&self, address: u64) -> bool {
        self.executable_address_ranges
            .iter()
            .any(|(start, end)| address >= *start && address <= *end)
    }

    #[allow(dead_code)]
    pub fn disassemble_linear_pass(&self, valid_jump_threshold: usize, valid_instruction_threshold: usize) -> HashSet<u64> {

        let mut functions = HashSet::<u64>::new();
        for (start, end) in self.executable_address_ranges.clone() {
            let mut pc = start;
            let mut valid_instructions = 0;
            let mut valid_jumps = 0;
            while pc < end {
                let instructions = match self.disassemble_instructions(pc, 1) {
                    Ok(instructions) => instructions,
                    Err(_) => {
                        pc += 1;
                        valid_instructions = 0;
                        valid_jumps = 0;
                        continue;
                    },
                };
                let instruction = instructions.iter().next().unwrap();
                if Disassembler::is_privilege_instruction(instruction)
                    || Disassembler::is_trap_instruction(instruction) {
                    pc += instruction.bytes().len() as u64;
                    continue;
                }
                if let Some(imm) = self.get_jump_immutable(instruction) {
                    if self.is_executable_address(imm) {
                        valid_jumps += 1;
                    } else {
                        valid_instructions = 0;
                        valid_jumps = 0;
                        pc += 1;
                        continue;
                    }
                }
                if let Some(imm) = self.get_call_immutable(instruction) {
                    if valid_jumps >= valid_jump_threshold
                        && valid_instructions >= valid_instruction_threshold {
                        if self.is_executable_address(imm) {
                            functions.insert(imm);
                        } else {
                            valid_instructions = 0;
                            valid_jumps = 0;
                            pc += 1;
                            continue;
                        }
                    }
                }
                valid_instructions += 1;
                pc += instruction.bytes().len() as u64;
            }
        }
        return functions;
    }

    #[allow(dead_code)]
    pub fn disassemble_control_flow(&self, addresses: HashSet<u64>) -> Result<CFG, Error> {

        let mut cfg = CFG::new();
        let mut functions_queue = VecDeque::<u64>::from_iter(addresses);
        let mut functions_processed = HashSet::<u64>::new();

        while let Some(pc) = functions_queue.pop_front() {

            if functions_processed.contains(&pc) {
                continue;
            }

            functions_processed.insert(pc);

            let function = match self.disassemble_function(pc) {
                Ok(function) => function,
                Err(_) => continue,
            };

            for (_, function_address) in function.functions() {
                if !functions_processed.contains(&function_address) {
                    functions_queue.push_back(function_address);
                }
            }

            for block in function.blocks() {
                if block.prologue {
                    if !functions_processed.contains(&block.address) {
                        functions_queue.push_back(block.address);
                    }
                }
            }

            cfg.functions.insert(function.address, function);

        }
        return Ok(cfg);
    }

    #[allow(dead_code)]
    pub fn disassemble_function(&self, address: u64) -> Result<Function, Error> {

        let mut function = Function::new(address, self.options.clone())?;
        function.options = self.options.clone();
        let mut block_queue = VecDeque::<u64>::from(vec![address]);
        let mut block_processed = HashSet::<u64>::new();

        while let Some(pc) = block_queue.pop_front() {
            if block_processed.contains(&pc) {
                continue;
            }

            block_processed.insert(pc);

            let block = match self.disassemble_block(pc) {
                Ok(block) => block,
                Err(error) => {
                    Debug::print(error.to_string());
                    return Err(error);
                }
            };

            if let Some(_) = block.error {
                continue;
            }

            for &addr in &block.to {
                if !block_processed.contains(&addr) {
                    block_queue.push_back(addr);
                }
            }

            if let Some(next) = block.next {
                if !block_processed.contains(&next) {
                    block_queue.push_back(next);
                }
            }

            function.blocks.insert(block.address, block);
        }

        function.patch_block_overlaps();

        if function.blocks.len() <= 0 {
            return Err(Error::new(ErrorKind::Other, "function does not contain any blocks"));
        }

        return Ok(function);
    }

    #[allow(dead_code)]
    pub fn get_block_by_address(blocks: &Vec<Block>, address: u64) -> Option<&Block> {
        for block in blocks {
            if block.address == address {
                return Some(block);
            }
        }
        return None;
    }

    #[allow(dead_code)]
    pub fn disassemble_block(&self, address: u64) -> Result<Block, Error> {

        if !self.is_executable_address(address) {
            return Err(Error::new(ErrorKind::Other, format!("Block -> 0x{:x}: does not start in executable memory", address)));
        }

        let mut block = match Block::new(address, self.options.clone()) {
            Ok(block) => block,
            Err(error) => return Err(error),
        };

        block.prologue = self.is_function_prologue(block.address);

        let mut pc: u64 = address;
        let mut functions: HashSet<u64>= HashSet::<u64>::new();
        loop {

            let instruction_container = match self.disassemble_instructions(pc, 1) {
                Ok(instruction_container) => instruction_container,
                Err(error) => {
                    block.edges = 0;
                    block.conditional = false;
                    block.error = Some(error);
                    return Ok(block);
                },
            };

            let instruction = instruction_container.iter().next().unwrap();

            let instruction_signature = match self.get_instruction_signature(&instruction) {
                Ok(wc) => wc,
                Err(error) => return Err(error),
            };

            let block_instruction = Instruction::new(
                instruction.address(),
                instruction.bytes().to_vec(),
                instruction_signature,
                Disassembler::is_return_instruction(instruction));

            block.instructions.insert(instruction.address(), block_instruction);

            // Starts with Trap Instruction
            if instruction.address() as u64 == address
                && Disassembler::is_trap_instruction(instruction) {
                block.edges = 0;
                block.conditional = false;
                return Ok(block);
            }

            // Ends with Trap Instruction
            if instruction.address() as u64 != address
                && Disassembler::is_trap_instruction(instruction){
                block.edges = self.get_instruction_edges(&instruction);
                block.conditional = false;
                return Ok(block);
            }

            if let Some(imm) = self.get_call_immutable(&instruction) {
                block.functions.insert(instruction.address(), imm);
                functions.insert(imm);
            }

            // Function Executable References
            let instruction_executable_addresses = self.get_instruction_executable_addresses(instruction);
            for instruction_executable_address in instruction_executable_addresses {
                block.functions.insert(instruction.address(), instruction_executable_address);
            }

            if let Some(imm) = self.get_conditional_jump_immutable(&instruction) {
                block.edges = self.get_instruction_edges(&instruction);
                block.next = Some(pc + instruction.bytes().len() as u64);
                block.to.insert(imm);
                block.conditional = true;
                return Ok(block);
            }

            if Disassembler::is_unconditional_jump_instruction(instruction) {
                if let Some(imm) = self.get_unconditional_jump_immutable(&instruction) {
                    block.edges = self.get_instruction_edges(&instruction);
                    block.to.insert(imm);
                    block.conditional = false;
                    return Ok(block);
                } else {
                    block.edges = self.get_instruction_edges(&instruction);
                    block.conditional = false;
                    return Ok(block);
                }
            }

            if Disassembler::is_return_instruction(&instruction) {
                block.conditional = false;
                block.edges = self.get_instruction_edges(&instruction);
                return Ok(block);
            }

            pc += instruction.bytes().len() as u64;
        }
    }

    pub fn is_function_prologue(&self, address: u64) -> bool {

        // Starting Instructions
        if let Ok(instructions) = self.disassemble_instructions(address, 2) {
            match self.machine {
                BinaryArchitecture::AMD64 => {
                    if instructions[0].id() == InsnId(X86Insn::X86_INS_PUSH as u32)
                        && self.instruction_has_register_operand(&instructions[0], 0, RegId(X86_REG_RBP as u16))
                        && instructions[1].id() != InsnId(X86Insn::X86_INS_MOV as u32)
                        && self.instruction_has_register_operand(&instructions[1], 0, RegId(X86_REG_RBP as u16))
                        && self.instruction_has_register_operand(&instructions[1], 1, RegId(X86_REG_RSP as u16))
                        {
                            return true;
                        }
                },
                BinaryArchitecture::I386 => {
                    if instructions[0].id() == InsnId(X86Insn::X86_INS_PUSH as u32)
                        && self.instruction_has_register_operand(&instructions[0], 0, RegId(X86_REG_EBP as u16))
                        && instructions[1].id() != InsnId(X86Insn::X86_INS_MOV as u32)
                        && self.instruction_has_register_operand(&instructions[1], 0, RegId(X86_REG_EBP as u16))
                        && self.instruction_has_register_operand(&instructions[1], 1, RegId(X86_REG_ESP as u16))
                        {
                            return true;
                        }
                },
                _ => {}
            }
        }

        let mut pc: u64 = address;
        let mut search: usize = 0;
        let mut has_push = false;
        while let Ok(instructions) = self.disassemble_instructions(pc, 1) {
            if search > 12 { break; }
            let instruction = instructions.iter().next().unwrap();
            if Disassembler::is_trap_instruction(instruction) { break; }
            if Disassembler::is_call_instruction(instruction) { break; }
            if Disassembler::is_jump_instruction(instruction) { break; }
            if Disassembler::is_privilege_instruction(instruction) { break; }
            if instruction.id() == InsnId(X86Insn::X86_INS_PUSH as u32) { has_push = true; }
            match self.machine {
                BinaryArchitecture::AMD64 => {
                    if instruction.id() == InsnId(X86Insn::X86_INS_ADD as u32)
                        && self.instruction_has_register_operand(instruction, 0, RegId(X86_REG_RSP as u16))
                        && self.instruction_contains_immutable_operand(instruction) {
                        return false;
                    }
                    if has_push
                        && instruction.id() == InsnId(X86Insn::X86_INS_SUB as u32)
                        && self.instruction_has_register_operand(instruction, 0, RegId(X86_REG_RSP as u16))
                        && self.instruction_contains_immutable_operand(instruction) {
                        return true;
                    }
                },
                BinaryArchitecture::I386 => {
                    if instruction.id() == InsnId(X86Insn::X86_INS_ADD as u32)
                        && self.instruction_has_register_operand(instruction, 0, RegId(X86_REG_ESP as u16))
                        && self.instruction_contains_immutable_operand(instruction) {
                        return false;
                    }
                    if has_push
                        && instruction.id() == InsnId(X86Insn::X86_INS_SUB as u32)
                        && self.instruction_has_register_operand(instruction, 0, RegId(X86_REG_ESP as u16))
                        && self.instruction_contains_immutable_operand(instruction) {
                        return true;
                    }
                },
                _ => {}
            }
            pc += instruction.bytes().len() as u64;
            search += 1;
        }

        return false;
    }

    fn instruction_has_register_operand(&self, instruction: &Insn, index: usize, register_id: RegId) -> bool {
        let operands = match self.get_instruction_operands(instruction) {
            Ok(operands) => operands,
            Err(_) => return false,
        };

        if let Some(operand) = operands.get(index) {
            if let ArchOperand::X86Operand(op) = operand {
                if let X86OperandType::Reg(reg_id) = op.op_type {
                    return reg_id == register_id;
                }
            }
        }
        return false;
    }

    #[allow(dead_code)]
    pub fn get_operand_mem(operand: &ArchOperand) -> Option<X86OpMem> {
        if let ArchOperand::X86Operand(operand) = operand {
            if let X86OperandType::Mem(_operand) = operand.op_type {
                return Some(_operand);
            }
        }
        None
    }

    #[allow(dead_code)]
    pub fn get_instruction_total_operand_size(&self, instruction: &Insn) -> Result<usize, Error> {
        let operands = self.get_instruction_operands(instruction)?;
        let mut result: usize = 0;
        for operand in operands {
            match operand {
                ArchOperand::X86Operand(op) => {
                    result += op.size as usize;
                },
                _ => return Err(Error::new(ErrorKind::Other, "unsupported operand architecture")),
            }
        }
        return Ok(result);
    }

    pub fn instruction_contains_memory_operand(&self, instruction: &Insn) -> bool {
        let operands = match self.get_instruction_operands(instruction) {
            Ok(operands) => operands,
            Err(_) => return false,
        };
        for operand in operands {
            if let ArchOperand::X86Operand(op) = operand {
                match op.op_type {
                    X86OperandType::Mem(_) => {
                        return true
                    },
                    _ => continue,
                };
            }
        }
        return false;
    }

    pub fn instruction_contains_immutable_operand(&self, instruction: &Insn) -> bool {
        let operands = match self.get_instruction_operands(instruction) {
            Ok(operands) => operands,
            Err(_) => return false,
        };
        for operand in operands {
            if let ArchOperand::X86Operand(op) = operand {
                match op.op_type {
                    X86OperandType::Imm(_) => return true,
                    _ => continue,
                };
            }
        }
        return false;
    }

    #[allow(dead_code)]
    pub fn get_instruction_signature(&self, instruction: &Insn) -> Result<String, Error> {

        if Disassembler::is_unsupported_signature_instruction(instruction) {
            return Ok(Binary::to_hex(instruction.bytes()));
        }

        if Disassembler::is_wildcard_instruction(instruction) {
            return Ok("??".repeat(instruction.bytes().len()));
        }

        if !self.instruction_contains_immutable_operand(instruction)
            && !self.instruction_contains_memory_operand(instruction) {
            return Ok(Binary::to_hex(instruction.bytes()));
        }

        let instruction_size = instruction.bytes().len() * 8;

        let mut wildcarded = vec![false; instruction_size];

        let instruction_trailing_null_size = instruction.bytes().iter().rev().take_while(|&&b| b == 0).count() * 8;

        let operands = self.get_instruction_operands(instruction)?;

        let total_operand_size = self.get_instruction_total_operand_size(instruction)?;

        if total_operand_size > instruction_size {
            Disassembler::print_instruction(instruction);
            return Err(Error::new(ErrorKind::Other,  format!("Instruction -> 0x{:x}: operand offset exceeds instruction size", instruction.address())));
        }

        let instruction_trailing_null_offset = instruction_size - instruction_trailing_null_size;

        let is_immutable_signature = self.is_immutable_instruction_to_signature(instruction);

        if total_operand_size <= 0 && operands.len() > 0 {
           return Err(Error::new(ErrorKind::Other, format!("Instruction -> 0x{:x}: instruction has operands but missing operand sizes", instruction.address())));
        }

        for operand in operands {
            if let ArchOperand::X86Operand(op) = operand {
                let should_wildcard = match op.op_type {
                    X86OperandType::Imm(_) => is_immutable_signature,
                    X86OperandType::Mem(mem) => {
                        mem.index() == RegId(0)
                    },
                    _ => false,
                };

                let displacement_size = match op.op_type {
                    X86OperandType::Mem(op_mem) => {
                        Disassembler::get_displacement_size(op_mem.disp() as u64) * 8
                    },
                    _ => 0,
                };

                let mut op_size = if (op.size as usize) > displacement_size {
                    op.size as usize
                } else {
                    displacement_size
                };

                if op_size > instruction_size {
                    op_size = op.size as usize;
                }

                if op_size > instruction_size {
                    return Err(Error::new(ErrorKind::Other, format!("Instruction -> 0x{:x}: instruction operand size exceeds instruction size", instruction.address())));
                }

                let operand_offset = instruction_size - op_size;

                if should_wildcard {
                    for i in 0..op_size as usize {
                        if operand_offset + i > wildcarded.len() {
                            return Err(Error::new(ErrorKind::Other, format!("Instruction -> 0x{:x}: instruction wildcard index is out of bounds", instruction.address())));
                        }
                        wildcarded[operand_offset + i] = true;
                    }
                }
            }
        }

        let instruction_hex = Binary::to_hex(instruction.bytes());

        if instruction_hex.len() % 2 != 0 {
            return Err(Error::new(ErrorKind::Other, format!("Instruction -> 0x{:x}: instruction hex string length is not even", instruction.address())));
        }

        let signature: String = instruction_hex
            .chars()
            .enumerate()
            .map(|(index, ch)| {
                let start = index * 4;
                let end = start + 4;
                if start >= instruction_trailing_null_offset  && is_immutable_signature {
                    '?'
                } else if wildcarded[start..end].iter().all(|&x| x) {
                    '?'
                } else {
                    ch
                }
            })
            .collect();

        if signature.len() % 2 != 0 {
            return Err(Error::new(ErrorKind::Other, format!("Instruction -> 0x{:x}: wildcarded hex string length is not even", instruction.address())));
        }

        if instruction_hex.len() != signature.len() {
            return Err(Error::new(ErrorKind::Other, format!("Instruction -> 0x{:x}: instruction hex length not same as wildcard hex length", instruction.address())));
        }

        return Ok(signature);

    }

    fn get_displacement_size(displacement: u64) -> usize {
        match displacement {
            0x00..=0xFF => 1,
            0x100..=0xFFFF => 2,
            0x10000..=0xFFFFFFFF => 4,
            _ => 8,
        }
    }

    #[allow(dead_code)]
    pub fn get_jump_immutable(&self, instruction: &Insn) -> Option<u64> {
        if Disassembler::is_jump_instruction(instruction) {
            let operand = match self.get_instruction_operand(instruction, 0) {
                Ok(operand) => operand,
                Err(_error) => return None,
            };
            return Disassembler::get_operand_immutable(&operand);
        }
        None
    }

    #[allow(dead_code)]
    pub fn get_conditional_jump_immutable(&self, instruction: &Insn) -> Option<u64> {
        if Disassembler::is_conditional_jump_instruction(instruction) {
            let operand = match self.get_instruction_operand(instruction, 0) {
                Ok(operand) => operand,
                Err(_error) => return None,
            };
            return Disassembler::get_operand_immutable(&operand);
        }
        None
    }

    #[allow(dead_code)]
    pub fn get_unconditional_jump_immutable(&self, instruction: &Insn) -> Option<u64> {
        if Disassembler::is_unconditional_jump_instruction(instruction) {
            let operand = match self.get_instruction_operand(instruction, 0) {
                Ok(operand) => operand,
                Err(_error) => return None,
            };
            return Disassembler::get_operand_immutable(&operand);
        }
        None
    }

    #[allow(dead_code)]
    pub fn get_instruction_executable_addresses(&self, instruction: &Insn) -> HashSet<u64> {
        let mut result = HashSet::<u64>::new();
        if !Disassembler::is_load_address_instruction(instruction) {
            return result;
        }
        let operands = match self.get_instruction_operands(instruction) {
            Ok(operands) => operands,
            Err(_) => return result,
        };
        for operand in operands {
            if let ArchOperand::X86Operand(operand) = operand {
                if let X86OperandType::Mem(mem) = operand.op_type {
                    if mem.base() != RegId(X86_REG_RIP as u16) { continue; }
                    if mem.index() != RegId(0) { continue; }
                    let address: u64 = (instruction.address() as i64 + mem.disp() + instruction.bytes().len() as i64) as u64;
                    if !self.is_executable_address(address) { continue; }
                    if !self.is_function_prologue(address) { continue; }
                    result.insert(address);
                }
            }
        }
        result
    }

    #[allow(dead_code)]
    pub fn get_call_immutable(&self, instruction: &Insn) -> Option<u64> {
        if Disassembler::is_call_instruction(instruction) {
            let operand = match self.get_instruction_operand(instruction, 0) {
                Ok(operand) => operand,
                Err(_error) => return None,
            };
            return Disassembler::get_operand_immutable(&operand);
        }
        None
    }

    #[allow(dead_code)]
    pub fn get_operand_immutable(op: &ArchOperand) -> Option<u64> {
        if let ArchOperand::X86Operand(op) = op {
            if let X86OperandType::Imm(imm) = op.op_type {
                return Some(imm as u64);
            }
        }
        None
    }

    #[allow(dead_code)]
    pub fn get_instruction_operands(&self, instruction: &Insn) -> Result<Vec<ArchOperand>, Error> {
        let detail = match self.cs.insn_detail(&instruction) {
            Ok(detail) => detail,
            Err(_error) => return Err(Error::new(ErrorKind::Other, "failed to get instruction detail")),
        };
        let arch = detail.arch_detail();
        return Ok(arch.operands());
    }

    #[allow(dead_code)]
    pub fn get_instruction_operand(&self, instruction: &Insn, index: usize) -> Result<ArchOperand, Error> {
        let operands = match self.get_instruction_operands(instruction) {
            Ok(operands) => operands,
            Err(error) => return Err(error),
        };
        let operand = match operands.get(index) {
            Some(operand) => operand.clone(),
            None => return Err(Error::new(ErrorKind::Other, "failed to get instruction operand"))
        };
        return Ok(operand);
    }

    #[allow(dead_code)]
    pub fn print_instructions(instructions: &Instructions) {
        for instruction in instructions.iter() {
            Disassembler::print_instruction(&instruction);
        }
    }

    #[allow(dead_code)]
    pub fn get_instruction_edges(&self, instruction: &Insn) -> usize {
        if Disassembler::is_unconditional_jump_instruction(instruction) {
            return 1;
        }
        if Disassembler::is_return_instruction(instruction) {
            return 1;
        }
        if Disassembler::is_conditional_jump_instruction(instruction){
            return 2;
        }
        return 0;
    }

    #[allow(dead_code)]
    pub fn is_immutable_instruction_to_signature(&self, instruction: &Insn) -> bool {

        if !self.instruction_contains_immutable_operand(instruction) {
            return false;
        }

        if Disassembler::is_call_instruction(instruction) || Disassembler::is_jump_instruction(instruction) {
            return true;
        }

        const STACK_INSTRUCTIONS: [InsnId; 5] = [
            InsnId(X86Insn::X86_INS_MOV as u32),
            InsnId(X86Insn::X86_INS_SUB as u32),
            InsnId(X86Insn::X86_INS_ADD as u32),
            InsnId(X86Insn::X86_INS_INC as u32),
            InsnId(X86Insn::X86_INS_DEC as u32),
        ];

        if STACK_INSTRUCTIONS.contains(&instruction.id()) {
            let operands = match self.get_instruction_operands(instruction) {
                Ok(operands) => operands,
                Err(_) => return false,
            };

            for operand in operands {
                if let ArchOperand::X86Operand(op) = operand {
                    if let X86OperandType::Reg(register_id) = op.op_type {
                        if [X86_REG_RSP, X86_REG_RBP, X86_REG_ESP, X86_REG_EBP].contains(&(register_id.0 as u32)) {
                            return true;
                        }
                    }
                }
            }
        }

        return false;
    }

    #[allow(dead_code)]
    pub fn is_unsupported_signature_instruction(instruction: &Insn) -> bool {
        vec![
            InsnId(X86Insn::X86_INS_MOVUPS as u32),
            InsnId(X86Insn::X86_INS_MOVAPS as u32),
            InsnId(X86Insn::X86_INS_XORPS as u32),
        ].contains(&instruction.id())
    }

    #[allow(dead_code)]
    pub fn is_return_instruction(instruction: &Insn) -> bool {
        vec![
            InsnId(X86Insn::X86_INS_RET as u32),
            InsnId(X86Insn::X86_INS_RETF as u32),
            InsnId(X86Insn::X86_INS_RETFQ as u32),
            InsnId(X86Insn::X86_INS_IRET as u32),
            InsnId(X86Insn::X86_INS_IRETD as u32),
            InsnId(X86Insn::X86_INS_IRETQ as u32),
        ].contains(&instruction.id())
    }

    #[allow(dead_code)]
    pub fn is_privilege_instruction(instruction: &Insn) -> bool {
        vec![
            InsnId(X86Insn::X86_INS_HLT as u32),
            InsnId(X86Insn::X86_INS_IN as u32),
            InsnId(X86Insn::X86_INS_INSB as u32),
            InsnId(X86Insn::X86_INS_INSW as u32),
            InsnId(X86Insn::X86_INS_INSD as u32),
            InsnId(X86Insn::X86_INS_OUT as u32),
            InsnId(X86Insn::X86_INS_OUTSB as u32),
            InsnId(X86Insn::X86_INS_OUTSW as u32),
            InsnId(X86Insn::X86_INS_OUTSD as u32),
            InsnId(X86Insn::X86_INS_RDMSR as u32),
            InsnId(X86Insn::X86_INS_WRMSR as u32),
            InsnId(X86Insn::X86_INS_RDPMC as u32),
            InsnId(X86Insn::X86_INS_RDTSC as u32),
            InsnId(X86Insn::X86_INS_LGDT as u32),
            InsnId(X86Insn::X86_INS_LLDT as u32),
            InsnId(X86Insn::X86_INS_LTR as u32),
            InsnId(X86Insn::X86_INS_LMSW as u32),
            InsnId(X86Insn::X86_INS_CLTS as u32),
            InsnId(X86Insn::X86_INS_INVD as u32),
            InsnId(X86Insn::X86_INS_INVLPG as u32),
            InsnId(X86Insn::X86_INS_WBINVD as u32),
        ].contains(&instruction.id())
    }

    #[allow(dead_code)]
    pub fn is_wildcard_instruction(instruction: &Insn) -> bool {
        Disassembler::is_nop_instruction(instruction)
        || Disassembler::is_trap_instruction(instruction)
    }

    #[allow(dead_code)]
    pub fn is_nop_instruction(instruction: &Insn) -> bool {
        vec![
            InsnId(X86Insn::X86_INS_NOP as u32),
            InsnId(X86Insn::X86_INS_FNOP as u32),
        ].contains(&instruction.id())
    }

    #[allow(dead_code)]
    pub fn is_trap_instruction(instruction: &Insn) -> bool {
        vec![
            InsnId(X86Insn::X86_INS_INT3 as u32),
            InsnId(X86Insn::X86_INS_UD2 as u32),
            InsnId(X86Insn::X86_INS_INT1 as u32),
            InsnId(X86Insn::X86_INS_INTO as u32),
        ].contains(&instruction.id())
    }

    #[allow(dead_code)]
    pub fn is_jump_instruction(instruction: &Insn) -> bool {
        if Disassembler::is_conditional_jump_instruction(instruction){
            return true;
        }
        if Disassembler::is_unconditional_jump_instruction(instruction){
            return true;
        }
        return false;
    }

    #[allow(dead_code)]
    pub fn is_load_address_instruction(instruction: &Insn) -> bool {
        vec![
            InsnId(X86Insn::X86_INS_LEA as u32),
        ].contains(&instruction.id())
    }

    #[allow(dead_code)]
    pub fn is_call_instruction(instruction: &Insn) -> bool {
        vec![
            InsnId(X86Insn::X86_INS_CALL as u32),
            InsnId(X86Insn::X86_INS_LCALL as u32),
        ].contains(&instruction.id())
    }

    #[allow(dead_code)]
    pub fn is_unconditional_jump_instruction(instruction: &Insn) -> bool {
        vec![
            InsnId(X86Insn::X86_INS_JMP as u32),
        ].contains(&instruction.id())
    }

    #[allow(dead_code)]
    pub fn is_conditional_jump_instruction(instruction: &Insn) -> bool {
        vec![
            InsnId(X86Insn::X86_INS_JNE as u32),
            InsnId(X86Insn::X86_INS_JNO as u32),
            InsnId(X86Insn::X86_INS_JNP as u32),
            InsnId(X86Insn::X86_INS_JL as u32),
            InsnId(X86Insn::X86_INS_JLE as u32),
            InsnId(X86Insn::X86_INS_JG as u32),
            InsnId(X86Insn::X86_INS_JGE as u32),
            InsnId(X86Insn::X86_INS_JE as u32),
            InsnId(X86Insn::X86_INS_JECXZ as u32),
            InsnId(X86Insn::X86_INS_JCXZ as u32),
            InsnId(X86Insn::X86_INS_JB as u32),
            InsnId(X86Insn::X86_INS_JBE as u32),
            InsnId(X86Insn::X86_INS_JA as u32),
            InsnId(X86Insn::X86_INS_JAE as u32),
            InsnId(X86Insn::X86_INS_JNS as u32),
            InsnId(X86Insn::X86_INS_JO as u32),
            InsnId(X86Insn::X86_INS_JP as u32),
            InsnId(X86Insn::X86_INS_JRCXZ as u32),
            InsnId(X86Insn::X86_INS_JS as u32),
            InsnId(X86Insn::X86_INS_LOOPE as u32),
            InsnId(X86Insn::X86_INS_LOOPNE as u32),
        ].contains(&instruction.id())
    }

    pub fn print_instruction(instruction: &Insn) {
        println!(
            "0x{:x}: {} {} {}",
            instruction.address(),
            instruction
                .bytes()
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<Vec<_>>()
                .join(" "),
            instruction.mnemonic().unwrap_or(""),
            instruction.op_str().unwrap_or(""),
        );
    }

    pub fn disassemble_instructions(&self, address: u64, count: u64) -> Result<Instructions<'_>, Error> {
        let instructions = self
            .cs
            .disasm_count(&self.image[address as usize..], address, count as usize)
            .map_err(|_| Error::new(ErrorKind::Other, "failed to disassemble instructions"))?;
        if instructions.len() <= 0 {
            return Err(Error::new(ErrorKind::Other, "no instructions found"));
        }
        Ok(instructions)
    }

    fn cs_new(machine: BinaryArchitecture, detail: bool) -> Result<Capstone, Error> {
        match machine {
            BinaryArchitecture::AMD64 => {
                Capstone::new()
                    .x86()
                    .mode(arch::x86::ArchMode::Mode64)
                    .syntax(arch::x86::ArchSyntax::Intel)
                    .detail(detail)
                    .build()
                    .map_err(|e| Error::new(ErrorKind::Other, format!("capstone error: {:?}", e)))
            },
            BinaryArchitecture::I386 => {
                Capstone::new()
                    .x86()
                    .mode(arch::x86::ArchMode::Mode32)
                    .syntax(arch::x86::ArchSyntax::Intel)
                    .detail(detail)
                    .build()
                    .map_err(|e| Error::new(ErrorKind::Other, format!("capstone error: {:?}", e)))
            },
            _ => Err(Error::new(ErrorKind::Other, "unsupported architecture"))
        }
    }
}
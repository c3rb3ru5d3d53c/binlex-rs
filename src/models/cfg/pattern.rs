use lief::pe::headers::MachineType;
use regex::bytes::Regex;

pub const PATTERN_MAX_MATCH_SIZE: usize = 32;
pub struct Pattern {
    machine: MachineType,
    amd64: Vec<Regex>,
    i386: Vec<Regex>,
}

impl Pattern {
    pub fn new(machine: MachineType) -> Self {

        let amd64: Vec<Regex> = vec![
            // mov reg, rsp
            // sub rsp, imm
            concat!(
                r"^(?:",
                r"[\x40-\x4F]\x8B[\x00-\xFF]",
                r"[\x40-\x4F]\x83\xEC[\x00-\xFF]",
                r")"
            ),
            concat!(
                // mov reg, rsp
                // mov qword [reg +local ], param
                r"^(?:",
                r"[\x40-\x4F]\x8B[\x00-\xFF]",
                r"[\x40-\x4F]\x89[\x00-\xff][\x00-\xff]",
                r")"
            ),
            // sub rsp, imm
            concat!(
                r"^(?:",
                r"[\x40-\x4F]\x83\xec[\x00-\xff]",
                r")"
            ),
            // mov rbp, rsp
            // sub rsp, imm
            concat!(
                r"(?:",
                r"[\x40-\x4F]\x8b\xec[\x40-\x4F]\x81\xec[\x00-\xff][\x00-\xff][\x00-\xff][\x00-\xff]",
                r")"
            )
        ]
        .into_iter()
        .map(Regex::new)
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|e| panic!("AMD64 Pattern: {}", e));

        let i386: Vec<Regex> = vec![
            r"^\x89\x44\x24[\x00-\xFF](\x50|\x51|\x52|\x53|\x55|\x56|\x57){2}\x83\xEC[\x00-\xFF]"
        ]
        .into_iter()
        .map(Regex::new)
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|e| panic!("I386 Pattern: {}", e));

        Self {
            machine,
            amd64,
            i386,
        }
    }

    pub fn is_prologue(&self, bytes: &[u8]) -> bool {
        match self.machine {
            MachineType::AMD64 => {
                self.amd64.iter().any(|m| m.is_match(bytes))
            },
            MachineType::I386 => {
                self.i386.iter().any(|m| m.is_match(bytes))
            },
            _ => false,
        }
    }
}
use std::collections::HashMap;

/*
Registers:
r0: stores return values, both for function calls and the current program exit code
r1-r5: used as function call arguments, upon program start r1 contains the "context" argument pointer
r6-r9: these get preserved between kernel function calls
r10: read-only pointer to the per-eBPF program 512 byte stack
*/

pub const MEM_SIZE : usize = 8000;

// Machine is an simulation of a machine ebpf would be running on
pub struct Machine {
    pub tape: Vec<Insn>,
    pub regs: HashMap<Register, i64>,
    pub mem: [i64; MEM_SIZE],
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Register {
    R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10,
}

#[derive(Copy, Clone)]
// Operands can be immediate values or register values
pub enum Operand {
    Imm (i32),
    Reg (Register),
}

#[derive(Copy, Clone)]
// Instructions eom
pub enum Insn {
    // ALU instructions
    Add (Register, Operand),
    Sub (Register, Operand),
    Mul (Register, Operand),
    Div (Register, Operand),
    Or (Register, Operand),
    And (Register, Operand),
    Lsh (Register, Operand),
    Rsh (Register, Operand),
    Neg (Register),
    Mod (Register, Operand),
    Xor (Register, Operand),
    Mov (Register, Operand),
    Arsh (Register, Operand),
    Stop,
}

use std::collections::HashMap;
use bpfeval::interp;
use bpfeval::types::{
    *,
    Insn::*,
    Operand::*,
    Register::*,
};

#[test]
fn simple_load() {
    let tape : Vec<Insn> = vec![
        Ldxdw(R0, Imm(0)),
        Stop,
    ];
    let mut mach = Machine {
        tape: tape,
        regs: HashMap::new(),
        mem: [0; MEM_SIZE],
    };
    mach.mem[0] = 23;
    interp::run_tape(&mut mach);
    assert_eq!(mach.val_of_reg(R0), 23);
}

#[test]
fn simple_lddw() {
    let tape : Vec<Insn> = vec![
        Lddw(R0, Imm(12)),
        Stop,
    ];
    let mut mach = Machine {
        tape: tape,
        regs: HashMap::new(),
        mem: [0; MEM_SIZE],
    };
    interp::run_tape(&mut mach);
    assert_eq!(mach.val_of_reg(R0), 12);
}

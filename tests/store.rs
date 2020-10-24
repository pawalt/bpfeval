use std::collections::HashMap;
use bpfeval::interp;
use bpfeval::types::{
    *,
    Insn::*,
    Operand::*,
    Register::*,
};

#[test]
fn store_imm() {
    let tape : Vec<Insn> = vec![
        Stdw(Imm(0), Imm(23)),
        Stop,
    ];
    let mut mach = Machine {
        tape: tape,
        regs: HashMap::new(),
        mem: [0; MEM_SIZE],
    };
    interp::run_tape(&mut mach);
    assert_eq!(mach.mem[0], 23);
}

#[test]
fn store_reg() {
    let tape : Vec<Insn> = vec![
        Add(R0, Imm(12)),
        Add(R1, Imm(13)),
        Stdw(Imm(0), Reg(R0)),
        Stdw(Imm(1), Reg(R1)),
        Stop,
    ];
    let mut mach = Machine {
        tape: tape,
        regs: HashMap::new(),
        mem: [0; MEM_SIZE],
    };
    interp::run_tape(&mut mach);
    assert_eq!(mach.mem[0], 12);
    assert_eq!(mach.mem[1], 13);
}

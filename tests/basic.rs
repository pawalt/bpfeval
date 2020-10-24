use std::collections::HashMap;
use bpfeval::types::*;
use bpfeval::types::Insn::*;
use bpfeval::types::Operand::*;
use bpfeval::types::Register::*;
use bpfeval::interp;

#[test]
fn simple_addition() {
    let tape : Vec<Insn> = vec![
        Add(R0, Imm(12)),
        Add(R1, Imm(13)),
        Add(R0, Reg(R1)),
        Stop,
    ];
    let mut mach = Machine {
        tape: tape,
        regs: HashMap::new(),
        mem: [0; MEM_SIZE],
    };
    interp::run_tape(&mut mach);
    assert_eq!(mach.val_of_reg(R0), 25);
    assert_eq!(mach.val_of_reg(R1), 13);
}
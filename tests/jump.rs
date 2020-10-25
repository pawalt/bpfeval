use bpfeval::interp;
use bpfeval::types::{
    *,
    Insn::*,
    Operand::*,
    Register::*,
    Immediate::*,
};

#[test]
fn simple_ja() {
    let tape : Vec<Insn> = vec![
        Ja(Val(5)),
        Stop,
    ];
    let mut mach = Machine::new(tape);
    interp::run_tape(&mut mach);
    assert_eq!(mach.pc, 6);
}

#[test]
fn simple_lt() {
    let tape : Vec<Insn> = vec![
        Add(R0, Imm(Val(3))),
        Add(R1, Imm(Val(4))),
        Jlt(R0, Reg(R1), Val(1)),
        Add(R0, Imm(Val(20))),
        Stop,
    ];
    let mut mach = Machine::new(tape);
    interp::run_tape(&mut mach);
    assert_eq!(mach.val_of_reg(R0), 3);
    assert_eq!(mach.val_of_reg(R1), 4);
}

#[test]
fn simple_lt_false() {
    let tape : Vec<Insn> = vec![
        Add(R0, Imm(Val(4))),
        Add(R1, Imm(Val(4))),
        Jlt(R0, Reg(R1), Val(1)),
        Add(R0, Imm(Val(20))),
        Stop,
    ];
    let mut mach = Machine::new(tape);
    interp::run_tape(&mut mach);
    assert_eq!(mach.val_of_reg(R0), 24);
    assert_eq!(mach.val_of_reg(R1), 4);
}

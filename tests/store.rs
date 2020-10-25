use bpfeval::interp;
use bpfeval::types::{
    *,
    Insn::*,
    Operand::*,
    Register::*,
    Immediate::*,
};

#[test]
fn store_imm() {
    let tape : Vec<Insn> = vec![
        Stdw(Imm(Val(0)), Imm(Val(23))),
        Stop,
    ];
    let mut mach = Machine::new(tape);
    interp::run_tape(&mut mach);
    assert_eq!(mach.mem[0], 23);
}

#[test]
fn store_reg() {
    let tape : Vec<Insn> = vec![
        Add(R0, Imm(Val(12))),
        Add(R1, Imm(Val(13))),
        Stdw(Imm(Val(0)), Reg(R0)),
        Stdw(Imm(Val(1)), Reg(R1)),
        Stop,
    ];
    let mut mach = Machine::new(tape);
    interp::run_tape(&mut mach);
    assert_eq!(mach.mem[0], 12);
    assert_eq!(mach.mem[1], 13);
}

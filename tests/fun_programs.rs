use bpfeval::interp;
use bpfeval::types;
use bpfeval::types::{
    *,
    Insn::*,
    Operand::*,
    Register::*,
    Immediate::*,
    LabeledTapeEntry::*,
};

/*
here we're implementing fibonacci with the following registers:
R0: higher fib
R1: lower fib
R2: num iterations
R3: max iterations
R4: temp register
*/
#[test]
fn test_fib() {
    let tape = vec![
        // initialize to proper values
        Insn(Lddw(R0, Imm(Val(1)))),
        Insn(Lddw(R1, Imm(Val(0)))),
        Insn(Lddw(R2, Imm(Val(0)))),
        Insn(Lddw(R3, Imm(Val(10)))),

        // jump to end if done
        LabeledTapeEntry::Lbl("TEST"),
        Insn(Jge(R2, Reg(R3), Immediate::Lbl("FIN"))),

        // run fib calculation
        Insn(Lddw(R4, Reg(R0))),
        Insn(Add(R0, Reg(R1))),
        Insn(Lddw(R1, Reg(R4))),

        Insn(Add(R2, Imm(Val(1)))),

        // jump back to conditional
        Insn(Ja(Immediate::Lbl("TEST"))),

        // end program
        LabeledTapeEntry::Lbl("FIN"),
        Insn(Stop),
    ];

    let mut mach = Machine::new_labeled(tape);
    interp::run_tape(&mut mach);
    assert_eq!(mach.val_of_reg(R0), 89);
}

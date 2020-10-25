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

#[test]
fn simple_lt() {
    let tape : Vec<LabeledTapeEntry> = vec![
        Insn(Add(R0, Imm(Val(3)))),
        Insn(Add(R1, Imm(Val(4)))),
        Insn(Jlt(R0, Reg(R1), Immediate::Lbl("JUMP_HERE"))),
        Insn(Add(R0, Imm(Val(20)))),
        LabeledTapeEntry::Lbl("JUMP_HERE"),
        Insn(Stop),
    ];
    let mut mach = Machine::new_labeled(tape);
    interp::run_tape(&mut mach);
    assert_eq!(mach.val_of_reg(R0), 3);
    assert_eq!(mach.val_of_reg(R1), 4);
}

#[test]
fn simple_lt_false() {
    let tape : Vec<LabeledTapeEntry> = vec![
        Insn(Add(R0, Imm(Val(5)))),
        Insn(Add(R1, Imm(Val(4)))),
        Insn(Jlt(R0, Reg(R1), Immediate::Lbl("JUMP_HERE"))),
        Insn(Add(R0, Imm(Val(20)))),
        LabeledTapeEntry::Lbl("JUMP_HERE"),
        Insn(Stop),
    ];
    let mut mach = Machine::new_labeled(tape);
    interp::run_tape(&mut mach);
    assert_eq!(mach.val_of_reg(R0), 25);
    assert_eq!(mach.val_of_reg(R1), 4);
}

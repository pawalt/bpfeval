use bpfeval::interp;
use bpfeval::types::{
    *,
    Insn::*,
    Operand::*,
    Register::*,
    Immediate::*,
};

#[test]
fn simple_addition() {
    let tape : Vec<Insn> = vec![
        Add(R0, Imm(Val(12))),
        Add(R1, Imm(Val(13))),
        Add(R0, Reg(R1)),
        Stop,
    ];
    let mut mach = Machine::new(tape);
    interp::run_tape(&mut mach);
    assert_eq!(mach.val_of_reg(R0), 25);
    assert_eq!(mach.val_of_reg(R1), 13);
}

#[test]
fn more_operations() {
    let tape : Vec<Insn> = vec![
        Add(R0, Imm(Val(12))),
        Add(R1, Imm(Val(13))),
        Add(R2, Imm(Val(23))),
        Add(R3, Imm(Val(82))),
        Add(R4, Imm(Val(2))),
        Add(R5, Imm(Val(3))),
        // R0 = 12
        And(R0, Reg(R1)),
        // R1 = 299
        Mul(R1, Reg(R2)),
        // R3 = 41
        Div(R3, Reg(R4)),
        // R4 = 7
        Add(R4, Imm(Val(5))),
        Stop,
    ];
    let mut mach = Machine::new(tape);
    interp::run_tape(&mut mach);
    let vals: Vec<(Register, i64)> = vec![
        (R0, 12),
        (R1, 299),
        (R2, 23),
        (R3, 41),
        (R4, 7),
        (R5, 3),
    ];
    for (reg, exp) in vals.iter() {
        assert_eq!(mach.val_of_reg(*reg), *exp);
    }
}

#[test]
fn test_neg() {
    let tape : Vec<Insn> = vec![
        Add(R0, Imm(Val(12))),
        Neg(R0),
        Stop,
    ];
    let mut mach = Machine::new(tape);
    interp::run_tape(&mut mach);
    assert_eq!(mach.val_of_reg(R0), -12)
}

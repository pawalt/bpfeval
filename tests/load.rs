use bpfeval::interp;
use bpfeval::types::{
    *,
    Insn::*,
    Operand::*,
    Register::*,
    Immediate::*,
};

#[test]
fn simple_load() {
    let tape : Vec<Insn> = vec![
        Ldxdw(R0, Imm(Val(0))),
        Stop,
    ];
    let mut mach = Machine::new(tape);
    mach.mem[0] = 23;
    interp::run_tape(&mut mach);
    assert_eq!(mach.val_of_reg(R0), 23);
}

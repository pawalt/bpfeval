use crate::types::*;
use crate::types::Insn::*;
use crate::types::Operand::*;

impl Machine {
    fn val_of_op(&self, op: &Operand) -> i64 {
        match op {
            Imm(v) => *v as i64,
            Reg(r) => self.val_of_reg(*r),
        }
    }

    pub fn val_of_reg(&self, reg: Register) -> i64 {
        match self.regs.get(&reg) {
            Some(v) => *v,
            None => 0,
        }
    }

    pub fn put_reg(&mut self, reg: Register, val: i64) {
        self.regs.insert(reg, val);
    }

    fn val_of_binop(&self, insn: &Insn, r: &Register, o: &Operand) -> i64 {
        let reg_val = self.val_of_reg(*r);
        let op_val = self.val_of_op(o);

        match insn {
            Add(_, _) => reg_val + op_val,
            Sub(_, _) => reg_val - op_val,
            Mul(_, _) => reg_val * op_val,
            Div(_, _) => reg_val / op_val,
            Or(_, _) => reg_val | op_val,
            And(_, _) => reg_val & op_val,
            Lsh(_, _) => reg_val << op_val,
            // need to cast so we get logical right shift
            Rsh(_, _) => (reg_val as u64 >> op_val) as i64,
            Mod(_, _) => reg_val % op_val,
            Xor(_, _) => reg_val ^ op_val,
            Mov(_, _) => op_val,
            Arsh(_, _) => reg_val >> op_val,
            _ => panic!("non binop operator fed into binop!"),
        }
    }
}

pub fn run_tape(mach: &mut Machine) {
    // have to clone here so that we still have mutable control of the tape
    // there's definitely a better way to do this, but i dont wanna implement it
    for insn in mach.tape.clone().iter() {
        match insn {
            // Start by matching all our ALU instructions
            Add(reg, op) |
            Sub(reg, op) |
            Mul(reg, op) |
            Div(reg, op) |
            Or(reg, op) |
            And(reg, op) |
            Lsh(reg, op) |
            Rsh(reg, op) |
            Mod(reg, op) |
            Xor(reg, op) |
            Mov(reg, op) |
            Arsh(reg, op) => {
                let to_store = mach.val_of_binop(
                    insn, reg, op
                );
                mach.put_reg(*reg, to_store);
            },
            Stop => return,
            _ => panic!("unexpected instruction"),
        }
    }
}

use crate::types::*;
use crate::types::Insn::*;
use crate::types::Operand::*;

impl Machine {
    fn val_of_op(&self, op: &Operand) -> i64 {
        match op {
            Imm(v) => *v as i64,
            Reg(r) => self.val_of_reg(*r),
            Ind(reg, imm) => self.val_of_reg(*reg) + *imm as i64,
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

    pub fn exec_ld(&mut self, r: &Register, o: &Operand, size: i64) {
        let mut load_addr = self.val_of_op(o) as u64;
        // cut load_addr down to size
        let num_cut = 64 - size;
        load_addr = load_addr << num_cut;
        load_addr = load_addr >> num_cut;
        let to_store = self.mem[load_addr as usize];
        self.put_reg(*r, to_store)
    }

    pub fn exec_str(&mut self, loc: &Operand, val: &Operand, size: i64) {
        let mut str_addr = self.val_of_op(loc) as u64;
        let num_cut = 64 - size;
        str_addr = str_addr << num_cut;
        str_addr = str_addr >> num_cut;
        let to_store = self.val_of_op(val);
        self.mem[str_addr as usize] = to_store;
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
            Neg(reg) => {
                let to_store = mach.val_of_reg(*reg) * -1;
                mach.put_reg(*reg, to_store);
            },
            Lddw(reg, op) => {
                let to_store = mach.val_of_op(op);
                mach.put_reg(*reg, to_store);
            },
            Ldxdw(reg, op) => mach.exec_ld(reg, op, 64),
            Ldxw(reg, op) => mach.exec_ld(reg, op, 32),
            Ldxh(reg, op) => mach.exec_ld(reg, op, 16),
            Ldxb(reg, op) => mach.exec_ld(reg, op, 8),
            Stdw(loc, val) => mach.exec_str(loc, val, 64),
            Stw(loc, val) => mach.exec_str(loc, val, 32),
            Sth(loc, val) => mach.exec_str(loc, val, 16),
            Stb(loc, val) => mach.exec_str(loc, val, 8),
            Stop => return,
        }
    }
}

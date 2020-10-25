use std::collections::HashMap;
use crate::types::{
    *,
    Insn::*,
    Operand::*,
    Immediate::*,
};

impl Machine {
    pub fn new(tape: Vec<Insn>) -> Self {
        Machine {
            tape: tape,
            regs: HashMap::new(),
            mem: [0; MEM_SIZE],
            pc: 0,
            labels: HashMap::new(),
        }
    }

    pub fn new_labeled(labeled_tape: Vec<LabeledTapeEntry>) -> Self {
        let (map, tape) = delabel_tape(labeled_tape);
        Machine {
            tape: tape,
            regs: HashMap::new(),
            mem: [0; MEM_SIZE],
            pc: 0,
            labels: map,
        }
    }

    fn val_of_op(&self, op: Operand) -> i64 {
        match op {
            Imm(imm) => self.val_of_imm(imm),
            Reg(r) => self.val_of_reg(r),
            Ind(reg, imm) => self.val_of_reg(reg) + self.val_of_imm(imm),
        }
    }

    fn val_of_imm(&self, imm: Immediate) -> i64 {
        match imm {
            Val(i) => i as i64,
            Lbl(l) => match self.labels.get(l) {
                Some(s) => *s as i64 - (self.pc as i64) - 1,
                None => panic!("could not find label"),
            },
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

    fn val_of_binop(&self, insn: Insn, r: Register, o: Operand) -> i64 {
        let reg_val = self.val_of_reg(r);
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

    pub fn exec_ld(&mut self, r: Register, o: Operand, size: i64) {
        let mut load_addr = self.val_of_op(o) as u64;
        // cut load_addr down to size
        let num_cut = 64 - size;
        load_addr = load_addr << num_cut;
        load_addr = load_addr >> num_cut;
        let to_store = self.mem[load_addr as usize];
        self.put_reg(r, to_store)
    }

    pub fn exec_str(&mut self, loc: Operand, val: Operand, size: i64) {
        let mut str_addr = self.val_of_op(loc) as u64;
        let num_cut = 64 - size;
        str_addr = str_addr << num_cut;
        str_addr = str_addr >> num_cut;
        let to_store = self.val_of_op(val);
        self.mem[str_addr as usize] = to_store;
    }


    fn exec_cnd_unsigned(&mut self, dst: Register, src: Operand, imm_offset: Immediate, cnd: impl Fn(u64, u64) -> bool) {
        let dst_val = self.val_of_reg(dst) as u64;
        let src_val = self.val_of_op(src) as u64;

        let offset = self.val_of_imm(imm_offset);
        if cnd(dst_val, src_val) {
            // need to do this fanciness because mach.pc is a usize!
            if offset.is_negative() {
                self.pc -= offset.wrapping_abs() as usize
            } else {
                self.pc += offset as usize;
            }
        }
    }

    fn exec_cnd_signed(&mut self, dst: Register, src: Operand, imm_offset: Immediate, cnd: impl Fn(i64, i64) -> bool) {
        let dst_val = self.val_of_reg(dst);
        let src_val = self.val_of_op(src);

        let offset = self.val_of_imm(imm_offset);
        if cnd(dst_val, src_val) {
            // need to do this fanciness because mach.pc is a usize!
            if offset.is_negative() {
                self.pc -= offset.wrapping_abs() as usize
            } else {
                self.pc += offset as usize;
            }
        }
    }
}

pub fn delabel_tape(labeled_tape: Vec<LabeledTapeEntry>) -> (HashMap<&'static str, usize>, Vec<Insn>) {
    let mut label_location_map: HashMap<&'static str, usize> = HashMap::new();
    let mut insn_count = 0;
    for entry in labeled_tape.iter() {
        match entry {
            LabeledTapeEntry::Lbl(s) => {label_location_map.insert(s, insn_count);},
            LabeledTapeEntry::Insn(_) => insn_count += 1,
        }
    }

    let mut final_tape = Vec::new();
    for entry in labeled_tape.iter() {
        match entry {
            LabeledTapeEntry::Lbl(s) => (),
            LabeledTapeEntry::Insn(i) => final_tape.push(*i),
        }
    }

    return (label_location_map, final_tape);
}

pub fn run_tape(mach: &mut Machine) {
    while mach.pc < mach.tape.len() {
        let insn = mach.tape[mach.pc].clone();

        match insn {
            // ALU instructions
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
                mach.put_reg(reg, to_store);
            },
            Neg(reg) => {
                let to_store = mach.val_of_reg(reg) * -1;
                mach.put_reg(reg, to_store);
            },

            // Load instructions
            Lddw(reg, op) => {
                let to_store = mach.val_of_op(op);
                mach.put_reg(reg, to_store);
            },
            Ldxdw(reg, op) => mach.exec_ld(reg, op, 64),
            Ldxw(reg, op) => mach.exec_ld(reg, op, 32),
            Ldxh(reg, op) => mach.exec_ld(reg, op, 16),
            Ldxb(reg, op) => mach.exec_ld(reg, op, 8),

            // Store instructions
            Stdw(loc, val) => mach.exec_str(loc, val, 64),
            Stw(loc, val) => mach.exec_str(loc, val, 32),
            Sth(loc, val) => mach.exec_str(loc, val, 16),
            Stb(loc, val) => mach.exec_str(loc, val, 8),

            // Jump
            Ja(off) => mach.exec_cnd_signed(Register::R0, Imm(Val(12)), off, |_, _| true),

            // Jump on equality
            Jeq(dst, src, off) => mach.exec_cnd_unsigned(dst, src, off, |x, y| x == y),
            Jne(dst, src, off) => mach.exec_cnd_unsigned(dst, src, off, |x, y| x != y),
            Jset(dst, src, off) => mach.exec_cnd_unsigned(dst, src, off, |x, y| x & y != 0),

            // Jump on unsigned conditional
            Jgt(dst, src, off) => mach.exec_cnd_unsigned(dst, src, off, |x, y| x > y),
            Jge(dst, src, off) => mach.exec_cnd_unsigned(dst, src, off, |x, y| x >= y),
            Jlt(dst, src, off) => mach.exec_cnd_unsigned(dst, src, off, |x, y| x < y),
            Jle(dst, src, off) => mach.exec_cnd_unsigned(dst, src, off, |x, y| x <= y),

            // Jump on signed conditional
            Jsgt(dst, src, off) => mach.exec_cnd_signed(dst, src, off, |x, y| x > y),
            Jsge(dst, src, off) => mach.exec_cnd_signed(dst, src, off, |x, y| x >= y),
            Jslt(dst, src, off) => mach.exec_cnd_signed(dst, src, off, |x, y| x < y),
            Jsle(dst, src, off) => mach.exec_cnd_signed(dst, src, off, |x, y| x <= y),

            Stop => return,
        }

        mach.pc += 1;
    }
}

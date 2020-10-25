# eBPF Instruction Set Interpreter

This project is a small interpreter for the eBPF instruction set. It does not deal with the actual assembling, instead just taking in types (specifed in `src/types.rs`) and matches on those. It terminates on the special `Stop` instruction.

For an example of how this instruction set looks, check out this fibonacci example:

```rust
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
```

# Where to look

For me, the coolest part of this project is `src/types.rs`. It blows my mind I can get such _fancy types_ in a systems language.

If you want to see the meat of the evaluation, check out `src/interp.rs`. There's nothing too surprising there -- mostly just emulating the eBPF instructions on our `Machine` type.

There are also my `tests/` if you're interested in those. Most interesting is `tests/fun_programs.rs` where I have my fibonacci test :)

# Why

I've been wanting to learn Rust for a while now, and I figured what better than an interpreter project! Rust has lots of cool type features that make writing an interpreter much easier than a traditional systems language (see C/C++/Go). It's also got great performance, which while not demonstrated here, is critical for an interpreter.

This project is also serving as a reminder for me that I need to actually dive into eBPF.

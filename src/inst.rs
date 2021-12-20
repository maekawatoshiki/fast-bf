pub enum Inst {
    ValAdd,
    ValSub,
    PtrAdd,
    PtrSub,
    In,
    Out,
    Jmp(u32),
}

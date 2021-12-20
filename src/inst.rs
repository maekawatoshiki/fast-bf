#[derive(Debug)]
pub enum Inst {
    ValAdd(u32),
    ValSub(u32),
    PtrAdd(u32),
    PtrSub(u32),
    In,
    Out,
    JmpIfZero(i32),
    Jmp(i32),
}

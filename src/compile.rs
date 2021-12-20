use crate::inst;

pub fn compile(code: impl AsRef<str>) -> Vec<inst::Inst> {
    let code = code.as_ref();
    let mut insts = vec![];
    let mut jmp_stack = vec![];

    for c in code.chars() {
        match c {
            '>' => insts.push(inst::Inst::PtrAdd(1)),
            '<' => insts.push(inst::Inst::PtrSub(1)),
            '+' => insts.push(inst::Inst::ValAdd(1)),
            '-' => insts.push(inst::Inst::ValSub(1)),
            '.' => insts.push(inst::Inst::Out),
            ',' => insts.push(inst::Inst::In),
            '[' => {
                jmp_stack.push(insts.len());
                insts.push(inst::Inst::JmpIfZero(0))
            }
            ']' => {
                let start = jmp_stack.pop().expect("missing '['");
                let end = insts.len();
                insts[start] = inst::Inst::JmpIfZero((end as isize - start as isize) as i32 + 1);
                insts.push(inst::Inst::Jmp((start as isize - end as isize) as i32))
            }
            _ => {}
        }
    }

    insts
}

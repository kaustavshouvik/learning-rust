mod add;

pub enum CalcOp {
    Add(i32, i32),
    SqRoot(i32),
}

pub fn op(op: CalcOp) -> i32 {
    match op {
        CalcOp::Add(a, b) => add::add(a, b),
        _ => {
            panic!("Not supported yet");
        }
    }
}

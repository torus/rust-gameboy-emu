#[derive(Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
}
pub struct Cpu {
    regs: Registers,
    ctx: Ctx,
}

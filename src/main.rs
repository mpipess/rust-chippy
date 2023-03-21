mod chippy;
fn main() {
    let mut chip1 = chippy::Chippy::new();
    chip1.pmem[0] = 0x2008;
    chip1.pmem[1] = 0x2105;
    chip1.pmem[2] = 0x20FF;
    chip1.pmem[3] = 0x4110;
    chip1.run();
}

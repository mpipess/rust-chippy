mod chippy;
fn main() {
    let mut chip1 = chippy::Chippy::new();
    chip1.pmem[0] = 0x2008;
    chip1.pmem[1] = 0x1064;
    chip1.pmem[2] = 0x2008;
    chip1.pmem[101] = 0x2005;
    chip1.run();
}

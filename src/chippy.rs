pub struct Chippy {
    pub pmem: [u16; 4096],
    pub dmem: [u8; 4096],
    stack: [u16; 16],
    v: [u8; 16],
    pc: u16,
    sp: u8,
}

impl Chippy {

    pub fn new() -> Self {
        Chippy {
            pmem: [0; 4096],
            dmem: [0; 4096],
            stack: [0; 16],
            v: [0; 16],
            pc: 0,
            sp: 0,
        }
    }

    pub fn run(&mut self) {
        while (self.pc as usize) < self.pmem.len() {
            self.decode(self.pmem[self.pc as usize]);
            self.pc += 1;
        }
        println!("Terminated!");
        println!("Register Contents:");
        for i in 0..self.v.len() {
            println!("v{}: {}", i, self.v[i]);
        }
    }

    fn decode(&mut self, opcode: u16) {
        let nybbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x0FF) as u8;
        let x = nybbles.1 as u8;
        let y = nybbles.2 as u8;
        let n = nybbles.3 as u8;

        match nybbles {
            (0x02, _, _, _) => self.op_2xkk(x, kk),
            (0x03, _, _, _) => self.op_3xkk(x, kk),
            (0x04, _, _, _) => self.op_4xkk(x, kk),
            (0x05, _, _, _) => self.op_5xkk(x, kk),
            (0x06, _, _, 0x00) => self.op_6xy0(x, y),
            (0x06, _, _, 0x01) => self.op_6xy1(x, y),
            (0x06, _, _, 0x02) => self.op_6xy2(x, y),
            (0x06, _, _, 0x03) => self.op_6xy3(x, y),
            (0x06, _, _, 0x04) => self.op_6xy4(x, y),
            (0x06, _, _, 0x05) => self.op_6xy5(x, y),
            (0x06, _, _, 0x06) => self.op_6xy6(x, y),
            _ => self.pc += 1,
        }

    }
    
    // Literal to Register Addition
    fn op_2xkk(&mut self, x: u8, kk: u8) {
        let vx = self.v[x as usize] as u16;
        let val = kk as u16;
        let result = vx + val;
        self.v[x as usize] = result as u8;
    }

    // Literal to Register Subtraction
    fn op_3xkk(&mut self, x: u8, kk: u8) {
        let vx = self.v[x as usize] as u16;
        let val = kk as u16;
        let result = vx - val;
        self.v[x as usize] = result as u8;
    }

    // Literal to Register Multiplication
    fn op_4xkk(&mut self, x: u8, kk: u8) {
        let vx = self.v[x as usize] as u16;
        let val = kk as u16;
        let result = vx * val;
        self.v[x as usize] = result as u8;
    }

    // Literal to Register Division
    fn op_5xkk(&mut self, x: u8, kk: u8) {
        let vx = self.v[x as usize] as u16;
        let val = kk as u16;
        let result = vx / val;
        self.v[x as usize] = result as u8;
    }

    // Register to Register Addition
    fn op_6xy0(&mut self, x: u8, y: u8) {
        let vx = self.v[x as usize] as u16;
        let vy = self.v[y as usize] as u16;
        let result = vx + vy;
        self.v[x as usize] = result as u8;
    }

    // Register to Register Subtraction
    fn op_6xy1(&mut self, x: u8, y: u8) {
        let vx = self.v[x as usize] as u16;
        let vy = self.v[y as usize] as u16;
        let result = vx - vy;
        self.v[x as usize] = result as u8;
    }

    // Register to Register Multiplication
    fn op_6xy2(&mut self, x: u8, y: u8) {
        let vx = self.v[x as usize] as u16;
        let vy = self.v[y as usize] as u16;
        let result = vx * vy;
        self.v[x as usize] = result as u8;
    }
    
    // Register to Register Division
    fn op_6xy3(&mut self, x: u8, y: u8) {
        let vx = self.v[x as usize] as u16;
        let vy = self.v[y as usize] as u16;
        let result = vx / vy;
        self.v[x as usize] = result as u8;
    }

    // Register to Register OR
    fn op_6xy4(&mut self, x: u8, y: u8) {
        self.v[x as usize] |= self.v[y as usize];
    }

    // Register to Register AND
    fn op_6xy5(&mut self, x: u8, y: u8) {
        self.v[x as usize] &= self.v[y as usize];
    }

    // Register to Register XOR
    fn op_6xy6(&mut self, x: u8, y: u8) {
        self.v[x as usize] ^= self.v[y as usize];
    }

}

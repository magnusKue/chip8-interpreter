use rand::Rng;

const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];



pub const RAM_SIZE: usize = 4096;
pub const REGISTER_COUNT: usize = 16;
pub const STACK_SIZE: usize = 16;
pub const NUM_KEYS: usize = 16;

pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_WIDTH: usize = 64;

const START_ADDR: u16 = 0x200;

pub struct Emu {
    pub programm_counter: u16,  // to keep count at which instruction we are
    pub ram: [u8; RAM_SIZE],
    
    pub screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    pressed_keys: [bool; NUM_KEYS],

    pub registers: [u8; REGISTER_COUNT],
    pub i_register: u16,
    
    pub stack_pointer: u16,
    pub stack: [u16; STACK_SIZE],

    pub delay_timer: u8,    // performs any action after finished
    pub sound_timer: u8,    // plays sound after finished

    pub is_paused: bool,
}

impl Default for Emu {
    fn default() -> Self {
        Emu {
            programm_counter: START_ADDR,  
            ram: [0; RAM_SIZE],
    
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            pressed_keys: [false; NUM_KEYS],

            registers: [0; REGISTER_COUNT],
            i_register: 0,
    
            stack_pointer: 0,
            stack: [0; STACK_SIZE],

            delay_timer: 0,    // performs any action after finished
            sound_timer: 0,    // plays sound after finished
            
            is_paused: false,

        }
    }
}

impl Emu {
    pub fn new() -> Self {
        let mut new_instance = Self::default();

        new_instance.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        new_instance
    }

    // RESET INSTANCE

    pub fn reset(&mut self) {
        self.programm_counter = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.registers = [0; REGISTER_COUNT];
        self.i_register = 0;
        self.stack_pointer = 0;
        self.stack = [0; STACK_SIZE];
        self.pressed_keys = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;

        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    // EXPOSE TO FRONTEND

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    pub fn keypress(&mut self, id: usize, pressed: bool) {
        self.pressed_keys[id] = pressed;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }
    
    // STACK
    
    fn push(&mut self, val:u16) {
        self.stack[self.stack_pointer as usize] = val;
        self.stack_pointer += 1;
    }

    fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }

    
    // CPU
    
    pub fn tick(&mut self) -> bool {
        if self.is_paused {
            return false
        }
        // fetch
        let opcode = self.fetch();
        // decode and execute
        self.execute(opcode)
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.programm_counter as usize] as u16;
        let lower_byte = self.ram[(self.programm_counter + 1) as usize] as u16;
        let opcode: u16 = (higher_byte << 8) | lower_byte;
        // println!("opcode loaded: {:0x}", opcode);
        self.programm_counter += 2;

        opcode
    }

    fn execute(&mut self, op: u16) -> bool {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        let mut framebuffer_modified = false;

        match (digit1, digit2, digit3, digit4) {
            // NOP
            (0, 0, 0, 0) => return false,
            // CLS
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            },
            // RET
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.programm_counter = ret_addr;
            },
            // JMP NNN
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.programm_counter = nnn;
            },
            // CALL NNN
            (2, _, _, _) => {
                let nnn = op & 0xFFF;
                self.push(self.programm_counter);
                self.programm_counter = nnn;
            },
            // SKIP VX == NN
            (3, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.registers[x] == nn {
                    self.programm_counter += 2;
                }
            },
            // SKIP VX != NN
            (4, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                if self.registers[x] != nn {
                    self.programm_counter += 2;
                }
            },
            // SKIP VX == VY
            (5, _, _, _) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.registers[x] == self.registers[y] {
                    self.programm_counter += 2;
                }
            },
            // VX = NN
            (6, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.registers[x] = nn;
            },
            // VX += NN
            (7, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                self.registers[x] = self.registers[x].wrapping_add(nn);
            },
            // VX = VY
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.registers[x] = self.registers[y];
            },
            // VX |= VY
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.registers[x] |= self.registers[y];
            },
            // VX &= VY
            (8, _, _, 2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.registers[x] &= self.registers[y];
            },
            // VX ^= VY
            (8, _, _, 3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.registers[x] ^= self.registers[y];
            },
            // VX += VY
            (8, _, _, 4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, carry) = self.registers[x].overflowing_add(self.registers[y]);
                let new_vf = if carry { 1 } else { 0 };

                self.registers[x] = new_vx;
                self.registers[0xF] = new_vf;
            },
            // VX -= VY
            (8, _, _, 5) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.registers[x].overflowing_sub(self.registers[y]);
                let new_vf = if borrow { 0 } else { 1 };

                self.registers[x] = new_vx;
                self.registers[0xF] = new_vf;
            },
            // VX >>= 1
            (8, _, _, 6) => {
                let x = digit2 as usize;
                let lsb = self.registers[x] & 1;
                self.registers[x] >>= 1;
                self.registers[0xF] = lsb;
            },
            // VX = VY - VX
            (8, _, _, 7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.registers[y].overflowing_sub(self.registers[x]);
                let new_vf = if borrow { 0 } else { 1 };

                self.registers[x] = new_vx;
                self.registers[0xF] = new_vf;
            },
            // VX <<= 1
            (8, _, _, 0xE) => {
                let x = digit2 as usize;
                let msb = (self.registers[x] >> 7) & 1;
                self.registers[x] <<= 1;
                self.registers[0xF] = msb;
            },
            // SKIP VX != VY
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.registers[x] != self.registers[y] {
                    self.programm_counter += 2;
                }
            },
            // I = NNN
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.i_register = nnn;
            },
            // JMP V0 + NNN
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.programm_counter = (self.registers[0] as u16) + nnn;
            },
            // VX = rand() & NN
            (0xC, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = rand::thread_rng().gen();
                self.registers[x] = rng & nn;
            },
            // DRAW
            (0xD, _, _, _) => {
                // Get the (x, y) coords for our sprite
                let x_coord = self.registers[digit2 as usize] as u16;
                let y_coord = self.registers[digit3 as usize] as u16;
                // The last digit determines how many rows high our sprite is
                let num_rows = digit4;

                // Keep track if any pixels were flipped
                let mut flipped = false;
                // Iterate over each row of our sprite
                for y_line in 0..num_rows {
                    // Determine which memory address our row's data is stored
                    let addr = self.i_register + y_line as u16;
                    let pixels = self.ram[addr as usize];
                    // Iterate over each column in our row
                    for x_line in 0..8 {
                        // Use a mask to fetch current pixel's bit. Only flip if a 1
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            // Sprites should wrap around screen, so apply modulo
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            // Get our pixel's index in the 1D screen array
                            let idx = x + SCREEN_WIDTH * y;
                            // Check if we're about to flip the pixel and set
                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }
                // Populate VF register
                if flipped {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }

                framebuffer_modified = true;
            },
            // SKIP KEY PRESS
            (0xE, _, 9, 0xE) => {
                let x = digit2 as usize;
                let vx = self.registers[x];
                let key = self.pressed_keys[vx as usize];
                if key {
                    self.programm_counter += 2;
                }
            },
            // SKIP KEY RELEASE
            (0xE, _, 0xA, 1) => {
                let x = digit2 as usize;
                let vx = self.registers[x];
                let key = self.pressed_keys[vx as usize];
                if !key {
                    self.programm_counter += 2;
                }
            },
            // VX = DT
            (0xF, _, 0, 7) => {
                let x = digit2 as usize;
                self.registers[x] = self.delay_timer;
            },
            // WAIT KEY
            (0xF, _, 0, 0xA) => {
                let x = digit2 as usize;
                let mut pressed = false;
                for i in 0..self.pressed_keys.len() {
                    if self.pressed_keys[i] {
                        self.registers[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    // Redo opcode
                    self.programm_counter -= 2;
                }
            },
            // DT = VX
            (0xF, _, 1, 5) => {
                let x = digit2 as usize;
                self.delay_timer = self.registers[x];
            },
            // ST = VX
            (0xF, _, 1, 8) => {
                let x = digit2 as usize;
                self.sound_timer = self.registers[x];
            },
            // I += VX
            (0xF, _, 1, 0xE) => {
                let x = digit2 as usize;
                let vx = self.registers[x] as u16;
                self.i_register = self.i_register.wrapping_add(vx);
            },
            // I = FONT
            (0xF, _, 2, 9) => {
                let x = digit2 as usize;
                let c = self.registers[x] as u16;
                self.i_register = c * 5;
            },
            // BCD
            (0xF, _, 3, 3) => {
                let x = digit2 as usize;
                let vx = self.registers[x] as f32;

                // Fetch the hundreds digit by dividing by 100 and tossing the decimal
                let hundreds = (vx / 100.0).floor() as u8;
                // Fetch the tens digit by dividing by 10, tossing the ones digit and the decimal
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                // Fetch the ones digit by tossing the hundreds and the tens
                let ones = (vx % 10.0) as u8;

                self.ram[self.i_register as usize] = hundreds;
                self.ram[(self.i_register + 1) as usize] = tens;
                self.ram[(self.i_register + 2) as usize] = ones;
            },
            // STORE V0 - VX
            (0xF, _, 5, 5) => {
                let x = digit2 as usize;
                let i = self.i_register as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.registers[idx];
                }
            },
            // LOAD V0 - VX
            (0xF, _, 6, 5) => {
                let x = digit2 as usize;
                let i = self.i_register as usize;
                for idx in 0..=x {
                    self.registers[idx] = self.ram[i + idx];
                }
            },
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {:#04x}", op),
        }

        framebuffer_modified
    }
    // TIMERS

    pub fn tick_timers(&mut self) -> bool {
        if self.is_paused {
            return false;
        }
        // println!("ticked timer");
        let mut beep = false;
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                beep = true;
            }
            self.sound_timer -= 1;
        }

        beep
    }

}

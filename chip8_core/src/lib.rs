use rand::Rng;

mod font;
use font::*;

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
    
    fn stack_push(&mut self, val:u16) {
        self.stack[self.stack_pointer as usize] = val;
        self.stack_pointer += 1;
    }

    fn stack_pop(&mut self) -> u16 {
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
        
        let x = digit2 as usize;
        let y = digit3 as usize;

        let nnn = op & 0xFFF;
        let nn = op & 0xFF;
        let n = op & 0xF;

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => self.op_0000(),
            (0, 0, 0xE, 0) => self.op_00e0(),
            (0, 0, 0xE, 0xE) => self.op_00ee(),
            (1, _, _, _) => self.op_1nnn(nnn),
            (2, _, _, _) => self.op_2nnn(nnn),
            (3, _, _, _) => self.op_3xnn(x, nn),
            (4, _, _, _) => self.op_4xnn(x, nn),
            (5, _, _, _) => self.op_5xy0(x, y),
            (6, _, _, _) => self.op_6xnn(x, nn),
            (7, _, _, _) => self.op_7xnn(x, nn),
            (8, _, _, 0) => self.op_8xy0(x, y),
            (8, _, _, 1) => self.op_8xy1(x, y),
            (8, _, _, 2) => self.op_8xy2(x, y),
            (8, _, _, 3) => self.op_8xy3(x, y),
            (8, _, _, 4) => self.op_8xy4(x, y),
            (8, _, _, 5) => self.op_8xy5(x, y),
            (8, _, _, 6) => self.op_8xy6(x, y),
            (8, _, _, 7) => self.op_8xy7(x, y),
            (8, _, _, 0xE) => self.op_8xye(x, y),
            (9, _, _, 0) => self.op_9xy0(x, y),
            (0xA, _, _, _) => self.op_annn(nnn),
            (0xB, _, _, _) => self.op_bnnn(nnn),
            (0xC, _, _, _) => self.op_cxnn(x, nn),
            (0xD, _, _, _) => self.op_dxyn(x, y, n),
            (0xE, _, 9, 0xE) => self.op_ex9e(x),
            (0xE, _, 0xA, 1) => self.op_exa1(x),
            (0xF, _, 0, 7) => self.op_fx07(x),
            (0xF, _, 0, 0xA) => self.op_fx0a(x),
            (0xF, _, 1, 5) => self.op_fx15(x),
            (0xF, _, 1, 8) => self.op_fx18(x),
            (0xF, _, 1, 0xE) => self.op_fx1e(x),
            (0xF, _, 2, 9) => self.op_fx29(x),
            (0xF, _, 3, 3) => self.op_fx33(x),
            (0xF, _, 5, 5) => self.op_fx55(x),
            (0xF, _, 6, 5) => self.op_fx65(x),
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {:#04x}", op),
        }
    }

    // NOP
    fn op_0000(&self) -> bool { false }

    // CLS
    fn op_00e0(&mut self) -> bool {
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        false
    }

    // RETURN FROM SUBROUTINE
    fn op_00ee(&mut self) -> bool {
        let ret_addr = self.stack_pop();
        self.programm_counter = ret_addr;
        false
    }

    // JUMP TO NNN
    fn op_1nnn(&mut self, nnn: u16) -> bool {
        self.programm_counter = nnn;
        false
    }

    // CALL nnn
    fn op_2nnn(&mut self, nnn: u16) -> bool {
        self.stack_push(self.programm_counter);
        self.programm_counter = nnn;
        false
    }
    // SKIP VX == NN
    fn op_3xnn(&mut self, x: usize, nn: u16) -> bool {
        if self.registers[x] == nn as u8 {
            self.programm_counter += 2;
        };
        false
    }
    // SKIP VX != NN
    fn op_4xnn(&mut self, x: usize, nn: u16) -> bool {
        if self.registers[x] != nn as u8 {
            self.programm_counter += 2;
        };
        false
    }

    // SKIP VX == VY
    fn op_5xy0(&mut self, x: usize, y: usize) -> bool {
        if self.registers[x] == self.registers[y] {
            self.programm_counter += 2;
        };
        false
    }

    // VX = NN
    fn op_6xnn(&mut self, x: usize, nn: u16) -> bool {
        self.registers[x] = nn as u8;
        false
    }
    
    // VX += NN
    fn op_7xnn(&mut self, x: usize, nn: u16) -> bool {
        self.registers[x] = self.registers[x].wrapping_add(nn as u8);
        false
    }

    // VX = VY
    fn op_8xy0(&mut self, x: usize, y:usize) -> bool {
        self.registers[x] = self.registers[y];
        false
    }

    // VX |= VY
    fn op_8xy1(&mut self, x: usize, y:usize) -> bool {
        self.registers[x] |= self.registers[y];
        false
    }
    // VX &= VY
    fn op_8xy2(&mut self, x: usize, y:usize) -> bool {
        self.registers[x] &= self.registers[y];
        false
    }

    // VX ^= VY
    fn op_8xy3(&mut self, x: usize, y:usize) -> bool {
        self.registers[x] ^= self.registers[y];
        false
    }

    // VX += VY
    fn op_8xy4(&mut self, x: usize, y: usize) -> bool {
        let (new_vx, carry) = self.registers[x].overflowing_add(self.registers[y]);
        let new_vf = if carry { 1 } else { 0 };

        self.registers[x] = new_vx;
        self.registers[0xF] = new_vf;
        false
    }

    // VX -= VY
    fn op_8xy5(&mut self, x: usize, y: usize) -> bool {
        let (new_vx, borrow) = self.registers[x].overflowing_sub(self.registers[y]);
        let new_vf = if borrow { 0 } else { 1 };

        self.registers[x] = new_vx;
        self.registers[0xF] = new_vf;
        false
    }

    // VX >>= 1
    fn op_8xy6(&mut self, x: usize, _y: usize) -> bool {
        let lsb = self.registers[x] & 1;
        self.registers[x] >>= 1;
        self.registers[0xF] = lsb;
        false
    }

    // VX = VY - VX
    fn op_8xy7(&mut self, x: usize, y: usize) -> bool {
        let (new_vx, borrow) = self.registers[y].overflowing_sub(self.registers[x]);
        let new_vf = if borrow { 0 } else { 1 };

        self.registers[x] = new_vx;
        self.registers[0xF] = new_vf;
        false
    }

    // VX <<= 1
    fn op_8xye(&mut self, x: usize, _y: usize) -> bool {
        let msb = (self.registers[x] >> 7) & 1;
        self.registers[x] <<= 1;
        self.registers[0xF] = msb;
        false
    }

    // SKIP VX != VY
    fn op_9xy0(&mut self, x: usize, y: usize) -> bool {
        if self.registers[x] != self.registers[y] {
            self.programm_counter += 2;
        };
        false
    }

    // I = NNN
    fn op_annn(&mut self, nnn: u16) -> bool {
        self.i_register = nnn;
        false
    }

    // JMP V0 + NNN
    fn op_bnnn(&mut self, nnn: u16) -> bool {
        self.programm_counter = (self.registers[0] as u16) + nnn;
        false
    }

    // VX = rand() & NN
    fn op_cxnn(&mut self, x: usize, nn: u16) -> bool {
        let rng: u8 = rand::thread_rng().gen();
        self.registers[x] = rng & (nn as u8);
        false
    }

    // DRAW
    fn op_dxyn(&mut self, x: usize, y: usize, n: u16) -> bool {
        // Get the (x, y) coords for our sprite
        let x_coord = self.registers[x] as u16;
        let y_coord = self.registers[y] as u16;
        // The last digit determines how many rows high our sprite is
        let num_rows = n;

        // Keep track if any pixels were flipped
        let mut flipped = false;
        // Iterate over each row of our sprite
        for y_line in 0..num_rows {
            // Determine which memory address our row's data is stored
            let addr = self.i_register + y_line;
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

        true
    }

    // SKIP KEY PRESS
    fn op_ex9e(&mut self, x: usize) -> bool {
        let vx = self.registers[x];
        let key = self.pressed_keys[vx as usize];
        if key {
            self.programm_counter += 2;
        };
        false
    }

    // SKIP KEY RELEASE
    fn op_exa1(&mut self, x: usize) -> bool {
        let vx = self.registers[x];
        let key = self.pressed_keys[vx as usize];
        if !key {
            self.programm_counter += 2;
        };
        false
    }

    // VX = DT
    fn op_fx07(&mut self, x: usize) -> bool {
        self.registers[x] = self.delay_timer;
        false
    }

    // WAIT KEY
    fn op_fx0a(&mut self, x: usize) -> bool {
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
        };

        false
    }

    // DT = VX
    fn op_fx15(&mut self, x: usize) -> bool {
        self.delay_timer = self.registers[x];
        false
    }

    // ST = VX
    fn op_fx18(&mut self, x: usize) -> bool {
        self.sound_timer = self.registers[x];
        false
    }

    // I += VX
    fn op_fx1e(&mut self, x: usize) -> bool {
        let vx = self.registers[x] as u16;
        self.i_register = self.i_register.wrapping_add(vx);
        false
    }

    // I = FONT
    fn op_fx29(&mut self, x: usize) -> bool {
        let c = self.registers[x] as u16;
        self.i_register = c * 5;
        false
    }
    
    // BCD
    fn op_fx33(&mut self, x: usize) -> bool {
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
        false
    }
    
    // STORE V0 - VX
    fn op_fx55(&mut self, x: usize) -> bool {
        let i = self.i_register as usize;
        for idx in 0..=x {
            self.ram[i + idx] = self.registers[idx];
        };
        false
    }

    // LOAD V0 - VX
    fn op_fx65(&mut self, x: usize) -> bool {
        let i = self.i_register as usize;
        for idx in 0..=x {
            self.registers[idx] = self.ram[i + idx];
        };
        false
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

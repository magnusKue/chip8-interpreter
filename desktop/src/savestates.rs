use chip8_core::*;

pub struct SaveState {
    programm_counter: u16,  // to keep count at which instruction we are
    ram: [u8; chip8_core::RAM_SIZE],
    registers: [u8; chip8_core::REGISTER_COUNT],
    i_register: u16,
    stack_pointer: u16,
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    stack: [u16; chip8_core::STACK_SIZE],
    delay_timer: u8,    // performs any action after finished
    sound_timer: u8,    // plays sound after finished
}

impl Default for SaveState {
    fn default() -> Self {
        SaveState {
            programm_counter: 0,  // to keep count at which instruction we are
            ram: [0; chip8_core::RAM_SIZE],
            registers: [0; chip8_core::REGISTER_COUNT],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            i_register: 0,
            stack_pointer: 0,
            stack: [0; chip8_core::STACK_SIZE],
            delay_timer: 0,    // performs any action after finished
            sound_timer: 0,    // plays sound after finished
        }
    }
}

pub fn make_save(emulator: &Emu) -> SaveState {
    println!("INFO: SaveState recorded!");

    SaveState {
        programm_counter: emulator.programm_counter,  
        ram: emulator.ram,
        registers: emulator.registers,
        i_register: emulator.i_register,
        screen: emulator.screen,
        stack_pointer: emulator.stack_pointer,
        stack: emulator.stack,
        delay_timer: emulator.delay_timer,     
        sound_timer: emulator.sound_timer, 
    }
}

pub fn load_save(save: &SaveState, emulator: &mut Emu) {
    println!("INFO: SaveState recorded!");

    emulator.programm_counter = save.programm_counter;
    emulator.ram = save.ram;
    emulator.registers = save.registers;
    emulator.i_register = save.i_register;
    emulator.screen = save.screen;
    emulator.stack_pointer = save.stack_pointer;
    emulator.stack = save.stack;
    emulator.delay_timer = save.delay_timer;
    emulator.sound_timer = save.sound_timer;
}


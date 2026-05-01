const ROWS: usize = 25;
const COLS: usize = 80;
const MAX_SNAKE: usize = ROWS * COLS;

const SNAKE: u8 = 0xDB;
const FOOD: u8 = 0xA2;
const EMPTY: u8 = 0x20;
const HORIZ: u8 = 0xC4;
const VERT: u8 = 0xB3;  
const TL: u8 = 0xDA;    
const TR: u8 = 0xBF;    
const BL: u8 = 0xC0;    
const BR: u8 = 0xD9;    

pub static mut SNAKE_BODY: [[usize; 2]; MAX_SNAKE] = [[0; 2]; MAX_SNAKE];
pub static mut SNAKE_LEN: usize = 0;
pub static mut FOOD_POS: [usize; 2] = [5, 10];
pub static mut DIRECTION: u8 = b'D'; 
pub static mut RAND_SEED: usize = 1;
pub static mut GAME_OVER: bool = false;
pub static mut TICK_RATE: usize = 10;

const MMIO: *mut u8 = 0xb8000 as *mut u8;
const COLOR: u8 = 0x0F;

pub fn write_char(row: usize, col: usize, ch: u8, color: u8) {
    unsafe {
        let pos = row * COLS + col;
        let ptr = (MMIO as usize + pos * 2) as *mut u8;
        *ptr = ch;
        *(ptr.add(1)) = color;
    }
}

pub fn read_char(row: usize, col: usize) -> u8 {
    unsafe {
        let pos = row * COLS + col;
        let ptr = (MMIO as usize + pos * 2) as *mut u8;
        *ptr
    }
}

pub fn clear_screen() {
    for r in 0..ROWS {
        for c in 0..COLS {
            write_char(r, c, EMPTY, COLOR);
        }
    }
}



pub fn draw_border() {
    for c in 1..COLS-1 {
        write_char(0, c, HORIZ, COLOR);
        write_char(ROWS-1, c, HORIZ, COLOR);
    }
    for r in 1..ROWS-1 {
        write_char(r, 0, VERT, COLOR);
        write_char(r, COLS-1, VERT, COLOR);
    }
    write_char(0, 0, TL, COLOR);
    write_char(0, COLS-1, TR, COLOR);
    write_char(ROWS-1, 0, BL, COLOR);
    write_char(ROWS-1, COLS-1, BR, COLOR);
}

pub fn next_rand(seed: usize) -> usize {
    let a = seed.wrapping_mul(6364136223846793005);
    let b = a.wrapping_add(1442695040888963407);
    b
}

pub fn place_food() {
    unsafe {
        loop {
            RAND_SEED = next_rand(RAND_SEED);
            let row = (RAND_SEED % (ROWS - 2)) + 1;
            RAND_SEED = next_rand(RAND_SEED);
            let col = (RAND_SEED % (COLS - 2)) + 1;
            let ch = read_char(row, col);
            if ch == EMPTY {
                FOOD_POS = [row, col];
                write_char(row, col, FOOD, COLOR);
                return;
            }
        }
    }
}

pub fn init() {
    draw_border();
    unsafe {
        SNAKE_LEN = 3;
        SNAKE_BODY[0] = [12, 42];
        SNAKE_BODY[1] = [12, 41];
        SNAKE_BODY[2] = [12, 40];
        for i in 0..SNAKE_LEN {
            write_char(SNAKE_BODY[i][0], SNAKE_BODY[i][1], SNAKE, COLOR);
        }

        write_char(FOOD_POS[0], FOOD_POS[1], FOOD, COLOR);
    }
}

pub fn update() {
    unsafe {
        if GAME_OVER { return; }

        let head = SNAKE_BODY[0];
        let new_head = match DIRECTION {
            b'W' => [head[0].wrapping_sub(1), head[1]],
            b'S' => [head[0] + 1, head[1]],
            b'A' => [head[0], head[1].wrapping_sub(1)],
            b'D' => [head[0], head[1] + 1],
            _ => [head[0], head[1] + 1],
        };

        let ch = read_char(new_head[0], new_head[1]);

        if ch == VERT || ch == HORIZ || ch == TL || ch == TR || ch == BL || ch == BR || ch == SNAKE {
            GAME_OVER = true;
            quit();
            return;
        }




        let ate_food = (ch == FOOD);

        if !ate_food {
            let tail = SNAKE_BODY[SNAKE_LEN - 1];
            write_char(tail[0], tail[1], EMPTY, COLOR);
        } else {
            if SNAKE_LEN < MAX_SNAKE {
                SNAKE_LEN += 1;
            }
            place_food();
        }

        let mut i = SNAKE_LEN - 1;
        while i > 0 {
            SNAKE_BODY[i] = SNAKE_BODY[i - 1];
            i -= 1;
        }

        SNAKE_BODY[0] = new_head;
        write_char(new_head[0], new_head[1], SNAKE, COLOR);
    }
}


pub fn quit() {
    unsafe {
        let mut port = x86_64::instructions::port::Port::new(0xf4);
        port.write(0x1u32); 
        loop {
            x86_64::instructions::hlt();
        }
    }
}

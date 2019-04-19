#![no_std]
#![no_main]

extern crate rs64_periph;
extern crate rs64_rt;
extern crate panic_halt;
extern crate volatile;

#[macro_use]
pub mod console;
mod cont;
mod fbcon;

use rs64_periph::pi;
use rs64_periph::vi;

const FRAMEBUFFER_PHYS_ADDR: usize = 0x0030_0000;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

enum Slide {
    Text(&'static str),
    Image(usize),
}

const SLIDES: [Slide;31] = [
    Slide::Text(TITLE),
    Slide::Text(OVERVIEW),
    Slide::Image(0),
    Slide::Image(1),
    Slide::Image(2),
    Slide::Image(3),
    Slide::Image(4),
    Slide::Text(BRINGUP_0),
    Slide::Text(BRINGUP_1),
    Slide::Text(BRINGUP_2),
    Slide::Text(BRINGUP_3),
    Slide::Text(BRINGUP_4),
    Slide::Text(BRINGUP_5),
    Slide::Text(BRINGUP_6),
    Slide::Text(BRINGUP_7),
    Slide::Text(BRINGUP_8),
    Slide::Text(RESULTS_0),
    Slide::Text(RESULTS_1),
    Slide::Text(RESULTS_2),
    Slide::Text(RESULTS_3),
    Slide::Text(RESULTS_4),
    Slide::Text(RESULTS_5),
    Slide::Text(RESULTS_6),
    Slide::Text(RESULTS_7),
    Slide::Text(RESULTS_8),
    Slide::Text(FUTURE_0),
    Slide::Text(FUTURE_1),
    Slide::Text(FUTURE_2),
    Slide::Text(FUTURE_3),
    Slide::Text(FUTURE_4),
    Slide::Text(QUESTIONS),
];

enum ProgressionState {
    WaitingForDown,
    WaitingForAUp,
    WaitingForBUp,
}

const A_KEY: u32 = 0x8000_0000;
const B_KEY: u32 = 0x4000_0000;

#[no_mangle]
pub unsafe extern "C" fn entry() {
    console::setup(FRAMEBUFFER_PHYS_ADDR, WIDTH, HEIGHT);

    cont::init();

    let mut cur_slide_num: usize = 0;

    let mut key_state = ProgressionState::WaitingForDown;

    loop {
        let cur_slide = &SLIDES[cur_slide_num];

        match cur_slide {
            &Slide::Text(text) => {
                console::clear();
                println!("{}", text);
                console::flush();
            }
            &Slide::Image(offset) => {
                let cart_data_base = 0x1010_1000;
                let image_size = WIDTH * HEIGHT * 2;
                let cart_image_base = (image_size * offset) + cart_data_base;

                pi::start_transfer_to_dram(FRAMEBUFFER_PHYS_ADDR, 
                        image_size, cart_image_base);

                pi::block_until_done();
            }
        }

        let keys = cont::scan().unwrap();

        let new_key_state = match &key_state {
            &ProgressionState::WaitingForDown => {
                if keys & A_KEY != 0 {
                    ProgressionState::WaitingForAUp
                } else if keys & B_KEY != 0 {
                    ProgressionState::WaitingForBUp
                } else {
                    ProgressionState::WaitingForDown
                }
            }

            &ProgressionState::WaitingForAUp => {
                if keys & A_KEY == 0 {
                    if cur_slide_num != SLIDES.len() -1 {
                        cur_slide_num += 1;
                    }
                    ProgressionState::WaitingForDown
                } else {
                    ProgressionState::WaitingForAUp
                }
            }

            &ProgressionState::WaitingForBUp => {
                if keys & B_KEY == 0 {
                    if cur_slide_num != 0 {
                        cur_slide_num -= 1;
                    }
                    ProgressionState::WaitingForDown
                } else {
                    ProgressionState::WaitingForBUp
                }
            }
        };

        key_state = new_key_state;

        //println!("{}", TITLE);

        vi::wait_for_vblank();
    }
}

const TITLE: &'static str = "









                                 
         Rust on the Nintendo 64 
                                 


                            
             Tristan Miller 
                            
              Apr 18, 2018  
                            
";

const OVERVIEW: &'static str = "



                Overview



    * Background

    * What is an N64?

    * Porting process

    * Future Directions

";

const BRINGUP_0: &'static str = "



                Bringup
";

const BRINGUP_1: &'static str = "



                Bringup



    * Look at existing minimal demos
";

const BRINGUP_2: &'static str = "



                Bringup



    * Look at existing minimal demos

    * Write tiniest possible piece 
      that runs
";

const BRINGUP_3: &'static str = "



                Bringup



    * Look at existing minimal demos

    * Write tiniest possible piece 
      that runs

    * Write tool to build ROM image
";

const BRINGUP_4: &'static str = "



                Bringup



    * Look at existing minimal demos

    * Write tiniest possible piece 
      that runs

    * Write tool to build ROM image

    * Target spec json +
      empty rust function +
      static library
";

const BRINGUP_5: &'static str = "



                Bringup



    * Look at existing minimal demos

    * Write tiniest possible piece 
      that runs

    * Write tool to build ROM image

    * Target spec json +
      empty rust function +
      static library
    
    * Linker (gnu ld to llvm lld)
";

const BRINGUP_6: &'static str = "



                Bringup



    * Look at existing minimal demos

    * Write tiniest possible piece 
      that runs

    * Write tool to build ROM image

    * Target spec json +
      empty rust function +
      static library
    
    * Linker (gnu ld to llvm lld)

    * Migrate tiniest to Rust
";

const BRINGUP_7: &'static str = "



                Bringup



    * Look at existing minimal demos

    * Write tiniest possible piece 
      that runs

    * Write tool to build ROM image

    * Target spec json +
      empty rust function +
      static library
    
    * Linker (gnu ld to llvm lld)

    * Migrate tiniest to Rust

    * Framebuffer console println
";

const BRINGUP_8: &'static str = "



                Bringup



    * Look at existing minimal demos

    * Write tiniest possible piece 
      that runs

    * Write tool to build ROM image

    * Target spec json +
      empty rust function +
      static library
    
    * Linker (gnu ld to llvm lld)

    * Migrate tiniest to Rust

    * Framebuffer console println

    * Profit
";

const RESULTS_0: &'static str = "



                Results


";

const RESULTS_1: &'static str = "



                Results



    * Rust is usable on weird
      embedded systems!
";

const RESULTS_2: &'static str = "



                Results



    * Rust is usable on weird
      embedded systems!
    
    * proc_macros are great
";

const RESULTS_3: &'static str = "



                Results



    * Rust is usable on weird
      embedded systems!
    
    * proc_macros are great
      (more tokens would be 
      really nice)
";

const RESULTS_4: &'static str = "



                Results



    * Rust is usable on weird
      embedded systems!
    
    * proc_macros are great
      (more tokens would be 
      really nice)

    * target spec json is really
      hard
";

const RESULTS_5: &'static str = "



                Results



    * Rust is usable on weird
      embedded systems!
    
    * proc_macros are great
      (more tokens would be 
      really nice)

    * target spec json is really
      hard (crosstool solution?)
";

const RESULTS_6: &'static str = "



                Results



    * Rust is usable on weird
      embedded systems!
    
    * proc_macros are great
      (more tokens would be 
      really nice)

    * target spec json is really
      hard (crosstool solution?)

    * naked functions with asm
      work great for initial
      code
";

const RESULTS_7: &'static str = "



                Results



    * Rust is usable on weird
      embedded systems!
    
    * proc_macros are great
      (more tokens would be 
      really nice)

    * target spec json is really
      hard (crosstool solution?)

    * naked functions with asm
      work great for initial
      code

    * ADTs are amazing for
      prototyping
";

const RESULTS_8: &'static str = "



                Results



    * Rust is usable on weird
      embedded systems!
    
    * proc_macros are great
      (more tokens would be 
      really nice)

    * target spec json is really
      hard (crosstool solution?)

    * naked functions with asm
      work great for initial
      code

    * ADTs are amazing for
      prototyping

    * Easy testing!
";

const FUTURE_0: &'static str = "



                Future

";

const FUTURE_1: &'static str = "



                Future



    * Crates registered
";

const FUTURE_2: &'static str = "



                Future



    * Crates registered

    * Better wrappers for 3D
      GFX
";

const FUTURE_3: &'static str = "



                Future



    * Crates registered

    * Better wrappers for 3D
      GFX

    * Make a game
";

const FUTURE_4: &'static str = "



                Future



    * Crates registered

    * Better wrappers for 3D
      GFX

    * Make a game

    * See what y'all make
";

const QUESTIONS: &'static str = "








              Questions?
";
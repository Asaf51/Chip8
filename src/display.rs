extern crate sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
const PIXEL_SIZE: usize = 20;

const DRAW_COLOR: (u8, u8, u8) = (0, 0, 0);
const BG_COLOR: (u8, u8, u8) = (255, 255, 255);

pub struct Display {
	pub vram: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
	pub needs_draw: bool,
	driver: DisplayDriver
}

struct DisplayDriver {
	canvas: Canvas<Window>
}

impl DisplayDriver {
	fn new() -> DisplayDriver {
		let display_ctx = sdl2::init().unwrap();
		let mut canvas = display_ctx.video()
			.unwrap()
			.window("Chip8 Emulator", (DISPLAY_WIDTH * PIXEL_SIZE) as u32, (DISPLAY_HEIGHT * PIXEL_SIZE) as u32)
			.position_centered()
			.opengl()
			.build()
			.unwrap()
			.into_canvas()
			.build()
			.unwrap();

		 canvas.set_draw_color(pixels::Color::RGB(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2));
		 canvas.clear();
		 canvas.present();

		 DisplayDriver {
		 	canvas
		 }
	}

	fn draw(&mut self, pixels: &[[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT]) {
		for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = x * PIXEL_SIZE;
                let y = y * PIXEL_SIZE;

                let draw_color = if col == 1 {
                	pixels::Color::RGB(DRAW_COLOR.0, DRAW_COLOR.1, DRAW_COLOR.2)
                } else {
                	pixels::Color::RGB(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2)
                };

                self.canvas.set_draw_color(draw_color);
                self.canvas
                	.fill_rect(Rect::new(x as i32, y as i32, PIXEL_SIZE as u32, PIXEL_SIZE as u32))
                	.expect("Could not draw to screen");
            }
        }

		self.canvas.present();
	}
}

impl Display {
	pub fn new() -> Display {
		Display {
			vram: [[0; 64]; 32],
			needs_draw: false,
			driver: DisplayDriver::new()
		}
	}

	pub fn clear(&mut self) {
		self.vram = [[0; 64]; 32];
		self.update();
	}

	pub fn update(&mut self) {
		self.driver.draw(&self.vram);
		self.needs_draw = false;
	}
}
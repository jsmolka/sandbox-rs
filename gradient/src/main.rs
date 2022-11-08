use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let sdl = sdl2::init()?;
    let sdl_video = sdl.video()?;

    let window = sdl_video
        .window("gradient", 256, 256)
        .position_centered()
        .opengl()
        .build()?;

    let mut canvas = window.into_canvas().build()?;

    let texture_creator = canvas.texture_creator();

    let mut texture =
        texture_creator.create_texture_streaming(PixelFormatEnum::ARGB8888, 256, 256)?;

    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..256 {
            for x in 0..256 {
                let index = pitch * y + 4 * x;
                buffer[index + 0] = y as u8;
                buffer[index + 1] = x as u8;
                buffer[index + 2] = 0;
                buffer[index + 3] = 0xFF;
            }
        }
    })?;

    canvas.clear();
    canvas.copy(&texture, None, None)?;
    canvas.present();

    let mut event_pump = sdl.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }

    Ok(())
}

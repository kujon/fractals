// use raqote::*;

use ::image::{ImageBuffer, Pixel, Rgb, Rgba};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston_window::*;
use rand::prelude::*;
use vecmath::*;

pub struct App {
    gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::math::Matrix2d;
        use graphics::*;

        // self.gl.draw(args.viewport(), |c, gl| {
        //     image(image: &<G as Graphics>::Texture, c.transform, gl);
        // });
    }
}

fn data_to_image_buffer(
    width: u32,
    height: u32,
    data: Vec<bool>,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    ImageBuffer::from_fn(width, height, |x, y| {
        let lit = data[(width * y + x) as usize];

        if lit {
            Rgba([0, 0, 0, 255])
        } else {
            Rgba([255, 255, 255, 255])
        }
    })
}

fn main() {
    let opengl = OpenGL::V3_2;
    let (width, height) = (400, 400);
    let mut window: PistonWindow = WindowSettings::new("piston: paint", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let random_data: Vec<bool> = (0..width * height * 4).map(|_| rand::random()).collect();
    let canvas = data_to_image_buffer(width, height, random_data);
    let mut texture: G2dTexture =
        Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }

        let random_data: Vec<bool> = (0..width * height * 4).map(|_| rand::random()).collect();
        let canvas = data_to_image_buffer(width, height, random_data);
        texture =
            Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();
    }

    // let data = vec![
    //     true, false, true, true, false, false, true, false, true, true, true, false, false, true,
    //     false, true,
    // ];
    // .into_iter()
    // .map(|val| if val { 0x00000000 } else { 0xffffffff })
    // .collect();

    // let mut dt = DrawTarget::from_vec(4, 4, data);

    // dt.write_png("foo.png");
}

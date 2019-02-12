extern crate image;
use image::{ImageBuffer, Pixel, Rgb};
extern crate num_complex;

fn main() {
    let width = 8000;
    let height = 6000;

    let mut imgbuf = ImageBuffer::new(width as u32, height as u32);

    let cx = -0.9;
    let cy = 0.27015;

    for x in 0..width {
        for y in 0..height {
            let c = num_complex::Complex::new(cx, cy);
            let zx = 3.0 * (x as f32 - 0.5 * width as f32) / width as f32;
            let zy = 2.0 * (y as f32 - 0.5 * height as f32) / height as f32;
            let mut z = num_complex::Complex::new(zx, zy);

            let mut i = 0;
            while i < 255 && z.norm() < 4.0 {
                z = z * z + c;
                i += 1;
            }

            let r = (i << 1) as u8;
            let g = (i << 8) as u8;
            let b = (i << 1) as u8;
            let pixel = Rgb::from_channels(r, g, b, 0);
            imgbuf.put_pixel(x as u32, y as u32, pixel);
        }
    }

    imgbuf.save("julia.png").unwrap();
}

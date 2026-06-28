use super::*;
use embedded_graphics_core::prelude::{Dimensions, DrawTarget, PixelColor};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Color {
    Black,
    White,
}

impl PixelColor for Color {
    type Raw = ();
}

pub struct Framebuffer<const FB_WIDTH: usize, const FB_HEIGHT: usize>
where
    [(); FB_WIDTH * FB_HEIGHT / 8]:,
{
    framebuffer: [u8; FB_WIDTH * FB_HEIGHT / 8],
}

impl<const FB_WIDTH: usize, const FB_HEIGHT: usize> Framebuffer<{ FB_WIDTH }, { FB_HEIGHT }>
where
    [(); FB_WIDTH * FB_HEIGHT / 8]:,
{
    pub fn new(color: Color) -> Self {
        let color_byte = match color {
            Color::Black => 0,
            Color::White => 0xFF,
        };
        Self {
            framebuffer: [color_byte; FB_WIDTH * FB_HEIGHT / 8],
        }
    }

    pub fn flush<C: IsDisplayConfiguration>(
        &mut self,
        display: &mut Display<C>,
        x_lo: i16,
        y_lo: i16,
    ) -> Result<(), Error<C>> {
        let mut rotated: [u8; FB_WIDTH * FB_HEIGHT / 8] = [0; FB_WIDTH * FB_HEIGHT / 8];
        for y in 0..FB_HEIGHT {
            for x in 0..FB_WIDTH {
                let src_idx = (y * FB_WIDTH + x) / 8;
                let bit_val = (self.framebuffer[src_idx] >> (7 - (x % 8))) & 1;
                if bit_val == 1 {
                    // 90° CCW: (x,y) → (new_x, new_y) = (y, FB_WIDTH−1−x)
                    let new_x = y;
                    let new_y = FB_WIDTH - 1 - x;
                    let dst_idx = (new_y * FB_HEIGHT + new_x) / 8;
                    let dst_bit = 7 - (new_x % 8);
                    rotated[dst_idx] |= 1 << dst_bit;
                }
            }
        }

        display.draw_image(
            &rotated,
            y_lo,
            (200 - x_lo) - FB_WIDTH as i16,
            y_lo + FB_HEIGHT as i16,
            ((200 - x_lo) - FB_WIDTH as i16) + FB_HEIGHT as i16,
        )
    }

    pub fn write_ram<C: IsDisplayConfiguration>(
        &mut self,
        display: &mut Display<C>,
        x_lo: i16,
        y_lo: i16,
    ) -> Result<(), Error<C>> {
        let mut rotated: [u8; FB_WIDTH * FB_HEIGHT / 8] = [0; FB_WIDTH * FB_HEIGHT / 8];
        for y in 0..FB_HEIGHT {
            for x in 0..FB_WIDTH {
                let src_idx = (y * FB_WIDTH + x) / 8;
                let bit_val = (self.framebuffer[src_idx] >> (7 - (x % 8))) & 1;
                if bit_val == 1 {
                    // 90° CCW: (x,y) → (new_x, new_y) = (y, FB_WIDTH−1−x)
                    let new_x = y;
                    let new_y = FB_WIDTH - 1 - x;
                    let dst_idx = (new_y * FB_HEIGHT + new_x) / 8;
                    let dst_bit = 7 - (new_x % 8);
                    rotated[dst_idx] |= 1 << dst_bit;
                }
            }
        }

        display.write_image(
            &rotated,
            y_lo,
            (200 - x_lo) - FB_WIDTH as i16,
            y_lo + FB_HEIGHT as i16,
            ((200 - x_lo) - FB_WIDTH as i16) + FB_HEIGHT as i16,
        )
    }
}

impl<const FB_WIDTH: usize, const FB_HEIGHT: usize> Dimensions
    for Framebuffer<{ FB_WIDTH }, { FB_HEIGHT }>
where
    [(); FB_WIDTH * FB_HEIGHT / 8]:,
{
    fn bounding_box(&self) -> embedded_graphics_core::primitives::Rectangle {
        embedded_graphics_core::primitives::Rectangle {
            top_left: embedded_graphics_core::geometry::Point { x: 0, y: 0 },
            size: embedded_graphics_core::geometry::Size {
                width: FB_HEIGHT as u32,
                height: FB_WIDTH as u32,
            },
        }
    }
}

impl<const FB_WIDTH: usize, const FB_HEIGHT: usize> DrawTarget
    for Framebuffer<{ FB_WIDTH }, { FB_HEIGHT }>
where
    [(); FB_WIDTH * FB_HEIGHT / 8]:,
{
    type Color = Color;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), ()>
    where
        I: IntoIterator<Item = embedded_graphics_core::Pixel<Color>>,
    {
        for embedded_graphics_core::Pixel(point, color) in pixels {
            let color_value = match color {
                Color::White => 1,
                Color::Black => 0,
            };
            let x = point.x as usize;
            let y = point.y as usize;
            if x >= FB_WIDTH || y >= FB_HEIGHT {
                continue;
            }
            let index = (y * FB_WIDTH + x) / 8;
            let bit = 7 - (x % 8);
            if color_value == 1 {
                self.framebuffer[index] |= 1 << bit;
            } else {
                self.framebuffer[index] &= !(1 << bit);
            }
        }
        Ok(())
    }
}

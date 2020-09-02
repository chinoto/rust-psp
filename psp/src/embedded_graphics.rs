//! Interop between the `psp` crate and the 2D `embedded-graphics` crate.

use crate::sys::{
    self, TexturePixelFormat, DisplayPixelFormat, DisplaySetBufSync
};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT, BUF_WIDTH};
use crate::vram_alloc;
use core::convert::{TryInto, TryFrom};
use embedded_graphics::{
    drawable::Pixel,
    geometry::{Size, Point},
    pixelcolor::{Rgb888, raw::RawU24},
    DrawTarget,
    prelude::*,
};

pub struct Framebuffer {
    vram_base: *mut u8,
    draw_buf: vram_alloc::VramMemChunk, 
}

impl Framebuffer {
    pub fn new() -> Self {
        unsafe {
            sys::sceDisplaySetMode(sys::DisplayMode::Lcd, 480, 272);
            let vram_base = (0x4000_0000u32 | sys::sceGeEdramGetAddr() as u32) as *mut u8;
            let mut allocator = vram_alloc::get_vram_allocator().unwrap();
            let draw_buf = allocator.alloc_texture_pixels(480, 272, TexturePixelFormat::Psm8888);
            sys::sceDisplaySetFrameBuf(
                vram_base as *const u8,
                BUF_WIDTH as usize,
                DisplayPixelFormat::Psm8888,
                DisplaySetBufSync::NextFrame,
            );
            Framebuffer { vram_base, draw_buf }
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            sys::sceDmacMemcpy(self.vram_base, self.draw_buf.as_mut_ptr_direct_to_vram(), 480*272*4); 
        }
    }

    #[inline]
    fn point_to_index(&self, point: Point) -> Option<usize> {
        if let Ok((x, y)) = <(u32, u32)>::try_from(point) {
            if x < BUF_WIDTH && y < self.size().height {
                return Some((x + y * BUF_WIDTH) as usize);
            }
        }
        None
    }

}

impl DrawTarget<Rgb888> for Framebuffer {
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, pixel: Pixel<Rgb888>) -> Result<(), Self::Error> {
        let Pixel(coord, color) = pixel;

        if let Ok((x @ 0..=SCREEN_WIDTH, y @ 0..=SCREEN_HEIGHT)) = coord.try_into() {
            unsafe {
                let ptr = (self.draw_buf.as_mut_ptr_direct_to_vram() as *mut u32)
                    .offset(x as isize)
                    .offset((y * BUF_WIDTH) as isize);

                *ptr = rgb_to_bgr(RawU24::from(color).into_inner());
            }
        }
        Ok(())
    }

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Rgb888>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if let Some(index) = self.point_to_index(point) {
                unsafe {
                    *(self.draw_buf.as_mut_ptr_direct_to_vram() as *mut u32).add(index) = rgb_to_bgr(RawU24::from(color).into_inner());
                }
            }
        }

        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
}

#[inline]
fn rgb_to_bgr(rgb: u32) -> u32 {
    core::intrinsics::bswap(rgb << 8 | rgb >> 24)
}

use crate::framebuffer::Framebuffer;

pub fn draw_line_bresenham(
    framebuffer: &mut Framebuffer,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
) {
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0
            && y0 >= 0
            && (x0 as u32) < framebuffer.width
            && (y0 as u32) < framebuffer.height
        {
            framebuffer.set_pixel(x0 as u32, y0 as u32);
        }

        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}
//! A camera object for ggez.
//! Currently ggez has no actual global camera state to use,
//! so this really just does the coordinate transforms for you.
//!
//! Basically it translates ggez's integer-valued coordinate
//! system with the origin at the top-left and Y increasing
//! downward, to a float-valued coordinate system with the
//! origin at the center of the screen and Y increasing upward.
//!
//! Because that makes sense, darn it.
//!
//! However, does not yet do any actual camera movements like
//! easing, pinning, etc.
//! But a great source for how such things work is this:
//! http://www.gamasutra.com/blogs/ItayKeren/20150511/243083/Scroll_Back_The_Theory_and_Practice_of_Cameras_in_SideScrollers.php

// TODO: Debug functions to draw world and camera grid!

use ggez;
use ggez::GameResult;
use ggez::graphics;
use na;
use super::Vector2;

// Hmm.  Could, instead, use a 2d transformation
// matrix, or create one of such.
pub struct Camera {
    screen_size: Vector2,
    view_size: Vector2,
    view_center: Vector2,
}

impl Camera {
    pub fn new(screen_width: u32, screen_height: u32, view_width: f64, view_height: f64) -> Self {
        let screen_size = Vector2::new(screen_width as f64, screen_height as f64);
        let view_size = Vector2::new(view_width as f64, view_height as f64);
        Camera {
            screen_size: screen_size,
            view_size: view_size,
            view_center: na::zero(),
        }
    }

    pub fn move_by(&mut self, by: Vector2) {
        self.view_center += by;
    }

    pub fn move_to(&mut self, to: Vector2) {
        self.view_center = to;
    }

    /// Translates a point in world-space to a point in
    /// screen-space.
    ///
    /// Does not do any clipping or anything, since it does
    /// not know how large the thing that might be drawn is;
    /// that's not its job.
    pub fn world_to_screen_coords(&self, from: Vector2) -> (i32, i32) {
        let pixels_per_unit = self.screen_size / self.view_size;
        let view_offset = from - self.view_center;
        let view_scale = view_offset * pixels_per_unit;


        let x = view_scale.x + self.screen_size.x / 2.0;
        let y = self.screen_size.y - (view_scale.y + self.screen_size.y / 2.0);
        (x as i32, y as i32)
    }


    // p_screen = max_p - p + max_p/2
    // p_screen - max_p/2 = max_p - p
    // p_screen - max_p/2 + max_p = -p
    // -p_screen - max_p/2 + max_p = p
    pub fn screen_to_world_coords(&self, from: (i32, i32)) -> Vector2 {
        let (sx, sy) = from;
        let sx = sx as f64;
        let sy = sy as f64;
        let flipped_x = sx - (self.screen_size.x / 2.0);
        let flipped_y = -sy + self.screen_size.y / 2.0;
        let screen_coords = Vector2::new(flipped_x, flipped_y);
        let units_per_pixel = self.view_size / self.screen_size;
        let view_scale = screen_coords * units_per_pixel;
        let view_offset = self.view_center + view_scale;

        view_offset

    }

    pub fn location(&self) -> Vector2 {
        self.view_center
    }

    fn calculate_dest_rect(&self, location: Vector2, dst_size: (f32, f32)) -> graphics::Rect {
        let (sx, sy) = self.world_to_screen_coords(location);
        let (sw, sh) = dst_size;
        graphics::Rect::new(sx as f32, sy as f32, sw as f32, sh as f32)
    }
}

pub trait CameraDraw
    where Self: graphics::Drawable
{
    fn draw_ex_camera(&mut self,
                      camera: &Camera,
                      location: Vector2,
                      context: &mut ggez::Context,
                      src: Option<graphics::Rect>,
                      dst_size: (f32, f32),
                      angle: f64,
                      center: Option<graphics::Point>,
                      flip_horizontal: bool,
                      flip_vertical: bool)
                      -> GameResult<()> {
        // let dest_rect = camera.calculate_dest_rect(location, dst_size);
        // self.draw_ex(context,
        //              src,
        //              Some(dest_rect),
        //              angle,
        //              center,
        //              flip_horizontal,
        //              flip_vertical)
        Ok(())
    }


    fn draw_camera(&mut self,
                   camera: &Camera,
                   location: Vector2,
                   context: &mut ggez::Context,
                   src: Option<graphics::Rect>,
                   dst_size: (u32, u32))
                   -> GameResult<()> {

        //let dest_rect = camera.calculate_dest_rect(location, dst_size);
        //self.draw(context, src, Some(dest_rect))
        Ok(())
    }
}


impl<T> CameraDraw for T where T: graphics::Drawable {}

#[cfg(test)]
mod tests {
    use super::super::Vector2;
    use super::*;

    #[test]
    fn test_coord_round_trip() {
        let mut c = Camera::new(640, 480, 40.0, 30.0);
        let p1 = (200, 300);
        {
            let p1_world = c.screen_to_world_coords(p1);
            assert_eq!(p1_world, Vector2::new(-7.5, -3.75));
            let p1_screen = c.world_to_screen_coords(p1_world);
            assert_eq!(p1, p1_screen);
        }


        let p2 = Vector2::new(20.0, 10.0);
        {
            let p2_screen = c.world_to_screen_coords(p2);
            assert_eq!(p2_screen, (640, 80));
            let p2_world = c.screen_to_world_coords(p2_screen);
            assert_eq!(p2_world, p2);
        }

        c.move_to(Vector2::new(5.0, 5.0));

        {
            let p1_world = c.screen_to_world_coords(p1);
            assert_eq!(p1_world, Vector2::new(-2.5, 1.25));
            let p1_screen = c.world_to_screen_coords(p1_world);
            assert_eq!(p1, p1_screen);
        }
        {
            let p2_screen = c.world_to_screen_coords(p2);
            assert_eq!(p2_screen, (560, 160));
            let p2_world = c.screen_to_world_coords(p2_screen);
            assert_eq!(p2_world, p2);
        }
    }
}

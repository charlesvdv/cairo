use x11::xlib;

use ffi;

use surface::Surface;

pub trait XlibSurface {
    fn create(dpy: &mut xlib::Display,
              drawable: xlib::Drawable,
              visual: &mut xlib::Visual,
              width: i32,
              height: i32)
              -> Surface;
    fn create_for_bitmap(dpy: &mut xlib::Display,
                         bitmap: xlib::Pixmap,
                         screen: &mut xlib::Screen,
                         width: i32,
                         height: i32)
                         -> Surface;
    fn set_size(&self, width: i32, height: i32);
    fn set_drawable(&self, drawable: xlib::Drawable, width: i32, height: i32);
    fn get_display(&self) -> &xlib::Display;
    fn get_drawable(&self) -> xlib::Drawable;
    fn get_screen(&self) -> &xlib::Screen;
    fn get_visual(&self) -> &xlib::Visual;
    fn get_depth(&self) -> i32;
    fn get_width(&self) -> i32;
    fn get_height(&self) -> i32;
}

impl XlibSurface for Surface {
    fn create(dpy: &mut xlib::Display,
              drawable: xlib::Drawable,
              visual: &mut xlib::Visual,
              width: i32,
              height: i32)
              -> Surface {
        unsafe {
            Surface::from_raw_full(ffi::cairo_xlib_surface_create(dpy,
                                                                  drawable,
                                                                  visual,
                                                                  width,
                                                                  height))
        }
    }

    fn create_for_bitmap(dpy: &mut xlib::Display,
                         bitmap: xlib::Pixmap,
                         screen: &mut xlib::Screen,
                         width: i32,
                         height: i32)
                         -> Surface {
        unsafe {
            Surface::from_raw_full(ffi::cairo_xlib_surface_create_for_bitmap(dpy,
                                                                             bitmap,
                                                                             screen,
                                                                             width,
                                                                             height))
        }
    }

    fn set_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::cairo_xlib_surface_set_size(self.to_raw_none(), width, height);
        }
    }

    fn set_drawable(&self, drawable: xlib::Drawable, width: i32, height: i32) {
        unsafe {
            ffi::cairo_xlib_surface_set_drawable(self.to_raw_none(),
                                                 drawable,
                                                 width,
                                                 height);
        }
    }

    fn get_display(&self) -> &xlib::Display {
        unsafe {
            &*ffi::cairo_xlib_surface_get_display(self.to_raw_none())
        }
    }

    fn get_drawable(&self) -> xlib::Drawable {
        unsafe {
            ffi::cairo_xlib_surface_get_drawable(self.to_raw_none())
        }
    }

    fn get_screen(&self) -> &xlib::Screen {
        unsafe {
            &*ffi::cairo_xlib_surface_get_screen(self.to_raw_none())
        }
    }

    fn get_visual(&self) -> &xlib::Visual {
        unsafe {
            &*ffi::cairo_xlib_surface_get_visual(self.to_raw_none())
        }
    }

    fn get_depth(&self) -> i32 {
        unsafe {
            ffi::cairo_xlib_surface_get_depth(self.to_raw_none())
        }
    }

    fn get_width(&self) -> i32 {
        unsafe {
            ffi::cairo_xlib_surface_get_width(self.to_raw_none())
        }
    }

    fn get_height(&self) -> i32 {
        unsafe {
            ffi::cairo_xlib_surface_get_height(self.to_raw_none())
        }
    }
}

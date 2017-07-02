// This file was generated by gir (9e3f4cc) from gir-files (0bcaef9)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct FontFamily(Object<ffi::PangoFontFamily>);

    match fn {
        get_type => || ffi::pango_font_family_get_type(),
    }
}

pub trait FontFamilyExt {
    fn get_name(&self) -> Option<String>;

    fn is_monospace(&self) -> bool;

    //fn list_faces(&self, faces: /*Unimplemented*/Vec<FontFace>) -> i32;
}

impl<O: IsA<FontFamily>> FontFamilyExt for O {
    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::pango_font_family_get_name(self.to_glib_none().0))
        }
    }

    fn is_monospace(&self) -> bool {
        unsafe {
            from_glib(ffi::pango_font_family_is_monospace(self.to_glib_none().0))
        }
    }

    //fn list_faces(&self, faces: /*Unimplemented*/Vec<FontFace>) -> i32 {
    //    unsafe { TODO: call ffi::pango_font_family_list_faces() }
    //}
}

// This file was generated by gir (c6b70b0) from gir-files (469db10)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct EngineShape(Object<ffi::PangoEngineShape, ffi::PangoEngineShapeClass>);

    match fn {
        get_type => || ffi::pango_engine_shape_get_type(),
    }
}

impl EngineShape {}
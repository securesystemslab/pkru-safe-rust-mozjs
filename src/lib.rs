/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */

#![crate_name = "mozjs"]
#![crate_type = "rlib"]

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, improper_ctypes)]

#![feature(plugin, custom_attribute)]
#![feature(macros_in_extern)]
#![plugin(mpk_protector)]
#![mpk_protector]

#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
extern crate mozjs_sys;
extern crate num_traits;

pub mod jsapi {
    pub use mozjs_sys::jsapi::*;
    pub use mozjs_sys::jsapi::JS::*;
    pub use mozjs_sys::jsapi::js::*;
    pub use mozjs_sys::jsapi::js::detail::*;
    pub use mozjs_sys::jsapi::JS::detail::*;
    pub use mozjs_sys::jsapi::js::shadow::{Object, ObjectGroup};
    pub use mozjs_sys::jsapi::js::Scalar::{Type};
    pub use mozjs_sys::jsapi::mozilla::{MallocSizeOf};
    pub use mozjs_sys::jsapi::glue::*;
}

#[macro_use]
pub mod rust;

mod consts;
pub mod conversions;
pub mod error;
pub mod glue;
pub mod panic;
pub mod typedarray;

pub use consts::*;
pub use mozjs_sys::jsval as jsval;

pub use jsval::JS_ARGV;
pub use jsval::JS_CALLEE;

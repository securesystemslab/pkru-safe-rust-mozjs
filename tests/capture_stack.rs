/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate mozjs;
extern crate libc;

use mozjs::jsapi::CallArgs;
use mozjs::jsapi::JSAutoRealm;
use mozjs::jsapi::JSContext;
use mozjs::jsapi::JS_DefineFunction;
use mozjs::jsapi::JS_NewGlobalObject;
use mozjs::jsapi::OnNewGlobalHookOption;
use mozjs::jsapi::RealmOptions;
use mozjs::jsapi::StackFormat;
use mozjs::jsapi::Value;
use mozjs::jsval::UndefinedValue;
use mozjs::rust::{JSEngine, Runtime, SIMPLE_GLOBAL_CLASS};

use std::ptr;

#[test]
fn capture_stack() {
    let engine = JSEngine::init().unwrap();
    let runtime = Runtime::new(engine);
    let context = runtime.cx();
    let h_option = OnNewGlobalHookOption::FireOnNewGlobalHook;
    let c_option = RealmOptions::default();

    unsafe {
        let global = JS_NewGlobalObject(context, &SIMPLE_GLOBAL_CLASS, ptr::null_mut(), h_option, &c_option);
        rooted!(in(context) let global_root = global);
        let global = global_root.handle();
        let _ac = JSAutoRealm::new(context, global.get());
        let function = JS_DefineFunction(context, global.into(), b"print_stack\0".as_ptr() as *const libc::c_char,
                                         Some(print_stack), 0, 0);
        assert!(!function.is_null());
        let javascript = "
            function foo(arg1) {
                var bar = function() {
                    print_stack();
                };
                bar();
            }

            foo(\"arg1-value\");
        ";
        rooted!(in(context) let mut rval = UndefinedValue());
        let _ = runtime.evaluate_script(global, javascript, "test.js", 0, rval.handle_mut());
    }
}

unsafe extern "C" fn print_stack(context: *mut JSContext, argc: u32, vp: *mut Value) -> bool {
    let args = CallArgs::from_vp(vp, argc);

    capture_stack!(in(context) let stack);
    let str_stack = stack.unwrap().as_string(None, StackFormat::SpiderMonkey).unwrap();
    println!("{}", str_stack);
    assert_eq!("bar@test.js:3:21\nfoo@test.js:5:17\n@test.js:8:13\n".to_string(), str_stack);

    args.rval().set(UndefinedValue());
    return true;
}

use rutin_tizen_sys::{ui_app_lifecycle_callback_s, ui_app_main};
use std::env::args_os;
use std::marker::Sized;
use std::os::raw::{c_char, c_int, c_void};
use std::os::unix::ffi::OsStrExt;
use std::ptr::null_mut;

pub trait UIApp: Sized {
    fn create(&mut self) -> bool;
    fn terminate(&mut self);
    fn pause(&mut self);
    fn resume(&mut self);

    fn main(&mut self) -> c_int {
        let args = args_os().collect::<Vec<_>>();
        let mut argv: Vec<*mut c_char> = Vec::new();
        for mut i in args {
            i.push("\0");
            argv.push(i.as_bytes().as_ptr() as *mut c_char);
        }
        argv.push(null_mut());

        let mut event_callback = ui_app_lifecycle_callback_s {
            create: Some(app_create::<Self>),
            terminate: Some(app_terminate::<Self>),
            pause: Some(app_pause::<Self>),
            resume: Some(app_resume::<Self>),
            app_control: None,
        };

        unsafe {
            ui_app_main(
                argv.len() as c_int,
                argv.as_mut_slice().as_mut_ptr(),
                &mut event_callback,
                self as *mut _ as *mut c_void,
            )
        }
    }
}

extern "C" fn app_create<T: UIApp>(data: *mut c_void) -> bool {
    let app = unsafe { &mut *(data as *mut T) };
    app.create()
}

extern "C" fn app_terminate<T: UIApp>(data: *mut c_void) {
    let app = unsafe { &mut *(data as *mut T) };
    app.terminate()
}

extern "C" fn app_pause<T: UIApp>(data: *mut c_void) {
    let app = unsafe { &mut *(data as *mut T) };
    app.pause()
}

extern "C" fn app_resume<T: UIApp>(data: *mut c_void) {
    let app = unsafe { &mut *(data as *mut T) };
    app.resume()
}

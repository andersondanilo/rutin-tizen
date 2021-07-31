use crate::efl::evas::Object;
use crate::rutin::rutin_debug;
use rutin_tizen_sys::Evas_Object;
use std::cell::{RefCell, RefMut};
use std::mem;
use std::os::raw::c_void;
use std::panic::catch_unwind;
use std::pin::Pin;

use rutin_tizen_sys::{
    Eext_Callback_Type, _Eext_Callback_Type_EEXT_CALLBACK_BACK,
    _Eext_Callback_Type_EEXT_CALLBACK_LAST, _Eext_Callback_Type_EEXT_CALLBACK_MORE,
    eext_object_event_callback_add,
};

#[derive(Copy, Clone)]
pub enum CallbackType {
    Back,
    More,
    Last,
}

impl From<CallbackType> for Eext_Callback_Type {
    fn from(cb_type: CallbackType) -> Eext_Callback_Type {
        match cb_type {
            CallbackType::Back => _Eext_Callback_Type_EEXT_CALLBACK_BACK,
            CallbackType::More => _Eext_Callback_Type_EEXT_CALLBACK_MORE,
            CallbackType::Last => _Eext_Callback_Type_EEXT_CALLBACK_LAST,
        }
    }
}

pub struct RegisteredExtCallback<'a> {
    pub callback_type: CallbackType,
    pub callback_fn: Pin<Box<Box<dyn FnMut() + 'a>>>,
    pub test_ptr: Option<*mut c_void>,
}

impl<'a> Drop for RegisteredExtCallback<'a> {
    fn drop(&mut self) {
        rutin_debug("registered ext callback DROPED!!");
    }
}

pub trait ObjectWithEventsExt<'a>: Object<'a> {
    fn ext_event_callback_add<F>(
        &mut self,
        callback_type: CallbackType,
        callback_fn: F,
    ) -> RegisteredExtCallback<'a>
    where
        F: FnMut(),
        F: 'a,
    {
        let mut rcb = RegisteredExtCallback {
            callback_type,
            callback_fn: Box::pin(Box::new(callback_fn)),
            test_ptr: None,
        };

        let callback_fn_ref = Pin::as_mut(&mut rcb.callback_fn);
        let box_ref = unsafe { Pin::get_unchecked_mut(callback_fn_ref) };
        let data = box_ref as *mut Box<dyn FnMut()> as *mut c_void;

        {
            unsafe {
                eext_object_event_callback_add(
                    self.eo_ptr(),
                    callback_type.clone().into(),
                    Some(event_callback_handler),
                    data,
                );
            }
        }

        rcb.test_ptr = Some(data);

        rcb
    }
}

impl<'a, O: Object<'a>> ObjectWithEventsExt<'a> for O {}

extern "C" fn event_callback_handler(
    data: *mut c_void,
    obj: *mut Evas_Object,
    event_info: *mut c_void,
) {
    if let Err(e) = catch_unwind(|| {
        let closure: &mut Box<dyn FnMut()> = unsafe { mem::transmute(data) };
        closure();
    }) {
        match e.downcast_ref::<&'static str>() {
            Some(s) => rutin_debug(&format!("panic: {:?}", s)),
            None => rutin_debug(&format!("panic: unknown {:?}", e)),
        };
        std::process::abort();
    }
}

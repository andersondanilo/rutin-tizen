use rutin_tizen_sys::elm_object_part_content_set;
use rutin_tizen_sys::elm_object_part_text_set;
use rutin_tizen_sys::evas_object_size_hint_weight_set;
use rutin_tizen_sys::EVAS_HINT_EXPAND;
use rutin_tizen_sys::EVAS_HINT_FILL;
use rutin_tizen_sys::{evas_free, evas_object_show, Eo};
use std::ffi::CString;

pub enum SizeHint {
    Expand,
    Fill,
    Other(f64),
}

impl From<SizeHint> for f64 {
    fn from(hint: SizeHint) -> f64 {
        match hint {
            SizeHint::Expand => EVAS_HINT_EXPAND,
            SizeHint::Fill => EVAS_HINT_FILL,
            SizeHint::Other(v) => v,
        }
    }
}

pub trait Object<'a> {
    fn eo_ptr(&mut self) -> *mut Eo;

    fn show(&mut self) {
        unsafe { evas_object_show(self.eo_ptr()) }
    }

    fn set_size_hint_weight(&mut self, x: SizeHint, y: SizeHint) {
        unsafe { evas_object_size_hint_weight_set(self.eo_ptr(), x.into(), y.into()) }
    }

    fn free(&mut self) {
        unsafe { evas_free(self.eo_ptr()) }
    }

    fn set_text(&mut self, text: &str) {
        let text_ptr = CString::new(text).unwrap().as_c_str().as_ptr();

        unsafe { elm_object_part_text_set(self.eo_ptr(), std::ptr::null(), text_ptr) }
    }

    fn set_content(&mut self, content: &mut dyn Object<'a>) {
        unsafe { elm_object_part_content_set(self.eo_ptr(), std::ptr::null(), content.eo_ptr()) }
    }
}

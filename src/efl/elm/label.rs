use crate::efl::evas::Object;
use rutin_tizen_sys::elm_label_add;
use rutin_tizen_sys::Eo;

pub struct Label<'a> {
    eo: *mut Eo,
    _phantom: std::marker::PhantomData<&'a Eo>,
}

impl<'a> Label<'a> {
    pub fn new(parent: &mut dyn Object<'a>) -> Option<Label<'a>> {
        let eo = unsafe { elm_label_add(parent.eo_ptr()) };

        if eo.is_null() {
            None
        } else {
            Some(Self {
                eo,
                _phantom: std::marker::PhantomData,
            })
        }
    }
}

impl<'a> Object<'a> for Label<'a> {
    fn eo_ptr(&mut self) -> *mut Eo {
        self.eo
    }
}

impl<'a> Drop for Label<'a> {
    fn drop(&mut self) {
        self.free()
    }
}

use super::prelude::*;
use rutin_tizen_sys::{
    elm_win_autodel_get, elm_win_autodel_set, elm_win_indicator_mode_set, elm_win_lower,
    elm_win_resize_object_add, elm_win_util_standard_add, elm_win_wm_rotation_supported_get,
    Elm_Win_Indicator_Mode, Elm_Win_Indicator_Mode_ELM_WIN_INDICATOR_HIDE,
    Elm_Win_Indicator_Mode_ELM_WIN_INDICATOR_SHOW,
    Elm_Win_Indicator_Mode_ELM_WIN_INDICATOR_UNKNOWN, Elm_Win_Indicator_Opacity_Mode,
    Elm_Win_Indicator_Opacity_Mode_ELM_WIN_INDICATOR_BG_TRANSPARENT,
    Elm_Win_Indicator_Opacity_Mode_ELM_WIN_INDICATOR_OPACITY_UNKNOWN,
    Elm_Win_Indicator_Opacity_Mode_ELM_WIN_INDICATOR_OPAQUE,
    Elm_Win_Indicator_Opacity_Mode_ELM_WIN_INDICATOR_TRANSLUCENT,
    Elm_Win_Indicator_Opacity_Mode_ELM_WIN_INDICATOR_TRANSPARENT, Eo,
};
use std::ffi::CString;

pub enum IndicatorMode {
    Unknown,
    Hide,
    Show,
}

impl From<IndicatorMode> for Elm_Win_Indicator_Mode {
    fn from(indicator_mode: IndicatorMode) -> Elm_Win_Indicator_Mode {
        match indicator_mode {
            IndicatorMode::Unknown => Elm_Win_Indicator_Mode_ELM_WIN_INDICATOR_UNKNOWN,
            IndicatorMode::Hide => Elm_Win_Indicator_Mode_ELM_WIN_INDICATOR_HIDE,
            IndicatorMode::Show => Elm_Win_Indicator_Mode_ELM_WIN_INDICATOR_SHOW,
        }
    }
}

pub enum IndicatorOpacityMode {
    Unknown,
    Opaque,
    Translucent,
    Transparent,
    BgTransparent,
}

impl From<IndicatorOpacityMode> for Elm_Win_Indicator_Opacity_Mode {
    fn from(indicator: IndicatorOpacityMode) -> Elm_Win_Indicator_Opacity_Mode {
        match indicator {
            IndicatorOpacityMode::Unknown => {
                Elm_Win_Indicator_Opacity_Mode_ELM_WIN_INDICATOR_OPACITY_UNKNOWN
            }
            IndicatorOpacityMode::Opaque => Elm_Win_Indicator_Opacity_Mode_ELM_WIN_INDICATOR_OPAQUE,
            IndicatorOpacityMode::Translucent => {
                Elm_Win_Indicator_Opacity_Mode_ELM_WIN_INDICATOR_TRANSLUCENT
            }
            IndicatorOpacityMode::Transparent => {
                Elm_Win_Indicator_Opacity_Mode_ELM_WIN_INDICATOR_TRANSPARENT
            }
            IndicatorOpacityMode::BgTransparent => {
                Elm_Win_Indicator_Opacity_Mode_ELM_WIN_INDICATOR_BG_TRANSPARENT
            }
        }
    }
}

pub struct Win<'a> {
    eo: *mut Eo,
    _phantom: std::marker::PhantomData<&'a Eo>,
}

impl<'a> Win<'a> {
    pub fn new(name: &str, title: &str) -> Option<Self> {
        let name = CString::new(name).unwrap();
        let title = CString::new(title).unwrap();

        let win = unsafe { elm_win_util_standard_add(name.as_ptr(), title.as_ptr()) };

        if win.is_null() {
            None
        } else {
            Some(Self {
                eo: win,
                _phantom: std::marker::PhantomData,
            })
        }
    }

    pub fn add_resize_object(&mut self, subobj: &mut dyn Object) {
        unsafe { elm_win_resize_object_add(self.eo_ptr(), subobj.eo_ptr()) }
    }

    pub fn set_indicator_mode(&mut self, indicator: IndicatorMode) {
        unsafe { elm_win_indicator_mode_set(self.eo_ptr(), indicator.into()) }
    }

    pub fn set_indicator_opacity(&mut self, indicator: IndicatorOpacityMode) {
        unsafe { elm_win_indicator_mode_set(self.eo_ptr(), indicator.into()) }
    }

    pub fn set_autodel(&mut self, value: bool) {
        unsafe {
            elm_win_autodel_set(self.eo_ptr(), if value { 1 } else { 0 });
        }
    }

    pub fn get_autodel(&mut self) -> bool {
        unsafe { elm_win_autodel_get(self.eo_ptr()) != 0 }
    }

    pub fn lower(&mut self) {
        unsafe { elm_win_lower(self.eo_ptr()) }
    }

    pub fn get_rotation_supported(&mut self) -> bool {
        unsafe { elm_win_wm_rotation_supported_get(self.eo_ptr()) != 0 }
    }
}

impl<'a> Object<'a> for Win<'a> {
    fn eo_ptr(&mut self) -> *mut Eo {
        self.eo
    }
}

impl<'a> Drop for Win<'a> {
    fn drop(&mut self) {
        self.free()
    }
}

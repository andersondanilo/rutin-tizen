use crate::system::dlog;
use crate::system::dlog::Priority;
use std::panic;

pub const LOG_TAG: &str = "RUTIN/LIB";

pub fn rutin_debug(msg: &str) {
    dlog::print(Priority::Debug, LOG_TAG, msg);
}

pub fn set_panic_dlog_hook() {
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            dlog::print(
                Priority::Error,
                LOG_TAG,
                &format!("panic occurred: {:?}", s),
            );
        } else {
            dlog::print(Priority::Error, LOG_TAG, "panic occurred: unknown");
        }

        if let Some(location) = panic_info.location() {
            rutin_debug(&format!(
                "panic occurred in file '{}' at line {}",
                location.file(),
                location.line(),
            ));
        }
    }));
}

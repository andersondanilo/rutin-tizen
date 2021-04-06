use rutin_tizen_sys::{
    dlog_print, log_priority, log_priority_DLOG_DEBUG, log_priority_DLOG_DEFAULT,
    log_priority_DLOG_ERROR, log_priority_DLOG_FATAL, log_priority_DLOG_INFO,
    log_priority_DLOG_PRIO_MAX, log_priority_DLOG_SILENT, log_priority_DLOG_UNKNOWN,
    log_priority_DLOG_VERBOSE, log_priority_DLOG_WARN,
};
use std::ffi::CString;

pub enum Priority {
    Unknown,
    Default,
    Verbose,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Silent,
    PrioMax,
}

impl From<Priority> for log_priority {
    fn from(priority: Priority) -> log_priority {
        match priority {
            Priority::Unknown => log_priority_DLOG_UNKNOWN,
            Priority::Default => log_priority_DLOG_DEFAULT,
            Priority::Verbose => log_priority_DLOG_VERBOSE,
            Priority::Debug => log_priority_DLOG_DEBUG,
            Priority::Info => log_priority_DLOG_INFO,
            Priority::Warn => log_priority_DLOG_WARN,
            Priority::Error => log_priority_DLOG_ERROR,
            Priority::Fatal => log_priority_DLOG_FATAL,
            Priority::Silent => log_priority_DLOG_SILENT,
            Priority::PrioMax => log_priority_DLOG_PRIO_MAX,
        }
    }
}

pub fn print(priority: Priority, tag: &str, message: &str) {
    let tag = CString::new(tag).unwrap();
    let message = CString::new(message).unwrap();

    unsafe {
        dlog_print(priority.into(), tag.as_ptr(), message.as_ptr());
    }
}

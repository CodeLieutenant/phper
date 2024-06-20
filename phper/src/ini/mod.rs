mod ini;
mod stage;

pub use ini::*;
pub use stage::Stage;

/// Get the global registered configuration value.
///
/// # Examples
///
/// ```no_run
/// use phper::ini::ini_get;
/// use std::ffi::CStr;
///
/// let _foo = ini_get::<bool>("FOO");
/// let _bar = ini_get::<Option<&CStr>>("BAR");
/// ```
#[inline]
pub fn ini_get<T: FromIniValue>(name: &str) -> T {
    T::from_ini_value(name)
}

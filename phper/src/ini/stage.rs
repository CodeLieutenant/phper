use phper_sys::{
    ZEND_INI_STAGE_ACTIVATE, ZEND_INI_STAGE_DEACTIVATE, ZEND_INI_STAGE_HTACCESS,
    ZEND_INI_STAGE_RUNTIME, ZEND_INI_STAGE_SHUTDOWN, ZEND_INI_STAGE_STARTUP,
};

/// Configuration for INI Stage.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Stage {
    /// INI Load Event -> Startup -> PHP Started
    Startup = ZEND_INI_STAGE_STARTUP as i32,
    /// INI Event -> PHP Shutting down
    Shutdown = ZEND_INI_STAGE_SHUTDOWN as i32,
    /// INI Event -> PHP Module Activated
    Activate = ZEND_INI_STAGE_ACTIVATE as i32,
    /// INI Event -> PHP Module Deactivated
    Deactivate = ZEND_INI_STAGE_DEACTIVATE as i32,
    /// INI Event -> Value changed with ini_set from PHP
    Runtime = ZEND_INI_STAGE_RUNTIME as i32,
    /// INI Event -> Value changed from .htaccess file with php_ini directive
    HtAccess = ZEND_INI_STAGE_HTACCESS as i32,
}

macro_rules! try_from_stage_int {
    ($arg:ty) => {
        impl TryFrom<$arg> for Stage {
            type Error = String;

            fn try_from(value: $arg) -> Result<Self, Self::Error> {
                match value as u32 {
                    ZEND_INI_STAGE_STARTUP => Ok(Stage::Startup),
                    ZEND_INI_STAGE_SHUTDOWN => Ok(Stage::Shutdown),
                    ZEND_INI_STAGE_ACTIVATE => Ok(Stage::Activate),
                    ZEND_INI_STAGE_DEACTIVATE => Ok(Self::Deactivate),
                    ZEND_INI_STAGE_RUNTIME => Ok(Stage::Runtime),
                    ZEND_INI_STAGE_HTACCESS => Ok(Stage::HtAccess),
                    _ => Err("Invalid Zend Stage for INI values".into()),
                }
            }
        }
    };
}

try_from_stage_int!(i8);
try_from_stage_int!(i16);
try_from_stage_int!(i32);
try_from_stage_int!(i64);
try_from_stage_int!(isize);
try_from_stage_int!(u8);
try_from_stage_int!(u16);
try_from_stage_int!(u32);
try_from_stage_int!(u64);
try_from_stage_int!(usize);

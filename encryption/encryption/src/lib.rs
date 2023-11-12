pub mod fnv;
pub mod xor;

pub use encryption_procmacro;

/// Encrypts string with XOR cipher
#[macro_export]
macro_rules! x {
    ($str:literal) => {
        $crate::xor::xor($crate::encryption_procmacro::x!($str))
    };
}

#[macro_export]
macro_rules! encrypt {
    () => {
        #[cfg_attr(not(debug_assertions), $crate::encryption_procmacro::encrypt)]
    };
}

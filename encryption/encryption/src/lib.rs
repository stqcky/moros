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

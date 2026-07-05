#[macro_use]
crate::macros;
macro_rules! debug {
    ($code:block) => {
        #[cfg(debug_assertions)]
        $code
    }
}
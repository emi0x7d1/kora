#[macro_export]
macro_rules! s {
    ($($lines:expr$(,)?)*) => {
        concat!($(
            $lines, "\n",
        )*)
    }
}

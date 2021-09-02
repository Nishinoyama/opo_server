pub mod player;
pub mod matching;
pub mod scoring;
pub mod tournament;
mod matching_builder;

/// Assert approximately equal.
///
/// Technically, assert that `|$left-$right|<$eps` is satisfying, where `|a|` is an absolute of `a`.
#[macro_export]
macro_rules! assert_ap {
    ($left:expr, $right:expr, $eps:expr $(,)?) => {
        assert!($left > $right - $eps);
        assert!($left < $right + $eps);
    };
    ($left:expr, $right:expr, $eps:expr, $($arg:tt)?) => {
        assert!($left > $right - $eps, $arg);
        assert!($left < $right + $eps, $arg);
    };
}

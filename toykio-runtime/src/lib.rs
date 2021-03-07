mod delay;
mod runtime;

pub use delay::*;
pub use runtime::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

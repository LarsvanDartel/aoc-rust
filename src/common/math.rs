pub fn log_floor<T, R>(mut n: T, base: T) -> R
where
    T: num_traits::NumAssign + std::cmp::PartialOrd<T> + Copy,
    R: num_traits::NumAssign,
{
    let mut l = R::zero();
    while n > T::zero() {
        n /= base;
        l += R::one();
    }
    l
}

use numlike::alg::PofieldElem;
use numlike::elem::Zero;
use numlike::limits::ExtendedBounds;

/// Defines a number type that is compatible with rstar.
///
/// rstar works out of the box with the following standard library types:
///  - i8, i16, i32, i64, i128, isize
///  - [Wrapping](core::num::Wrapping) versions of the above
///  - f32, f64
///
/// This type cannot be implemented directly since it has a blanket
/// implementation. Instead, implement all the required traits from the
/// [`numlike`](https://docs.rs/numlike) crate.
pub trait RTreeNum: ExtendedBounds + PofieldElem + Clone {}
impl<S> RTreeNum for S where S: ExtendedBounds + PofieldElem + Clone {}

/// Defines a point type that is compatible with rstar.
///
/// This trait should be used for interoperability with other point types, not to define custom objects
/// that can be inserted into r-trees. Use [`crate::rstar::RTreeObject`] or
/// [`crate::rstar::primitives::GeomWithData`] instead.
/// This trait defines points, not points with metadata.
///
/// `Point` is implemented out of the box for arrays like `[f32; 2]` or `[f64; 7]` (for any number of dimensions),
/// and for tuples like `(int, int)` and `(f64, f64, f64)` so tuples with only elements of the same type (up to dimension 9).
///
///
/// # Implementation example
/// Supporting a custom point type might look like this:
///
/// ```
/// use rstared::rstar::Point;
///
/// #[derive(Copy, Clone, PartialEq, Debug)]
/// struct IntegerPoint
/// {
///     x: i32,
///     y: i32
/// }
///
/// impl Point for IntegerPoint
/// {
///   type Scalar = i32;
///   const DIMENSIONS: usize = 2;
///
///   fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self
///   {
///     IntegerPoint {
///       x: generator(0),
///       y: generator(1)
///     }
///   }
///
///   fn nth(&self, index: usize) -> Self::Scalar
///   {
///     match index {
///       0 => self.x,
///       1 => self.y,
///       _ => unreachable!()
///     }
///   }
///
///   fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar
///   {
///     match index {
///       0 => &mut self.x,
///       1 => &mut self.y,
///       _ => unreachable!()
///     }
///   }
/// }
/// ```
pub trait Point: Clone + PartialEq + core::fmt::Debug {
    /// The number type used by this point type.
    type Scalar: RTreeNum;

    /// The number of dimensions of this point type.
    const DIMENSIONS: usize;

    /// Creates a new point value with given values for each dimension.
    ///
    /// The value that each dimension should be initialized with is given by the parameter `generator`.
    /// Calling `generator(n)` returns the value of dimension `n`, `n` will be in the range `0 .. Self::DIMENSIONS`,
    /// and will be called with values of `n` in ascending order.
    fn generate(generator: impl FnMut(usize) -> Self::Scalar) -> Self;

    /// Returns a single coordinate of this point.
    ///
    /// Returns the coordinate indicated by `index`. `index` is always smaller than `Self::DIMENSIONS`.
    fn nth(&self, index: usize) -> Self::Scalar;

    /// Mutable variant of [nth](#methods.nth).
    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar;
}

impl<T> PointExt for T where T: Point {}

/// Utility functions for Point
pub trait PointExt: Point {
    /// Returns a new Point with all components set to zero.
    fn new() -> Self {
        Self::from_value(Self::Scalar::ZERO)
    }

    /// Applies `f` to each pair of components of `self` and `other`.
    fn component_wise(
        &self,
        other: &Self,
        mut f: impl FnMut(Self::Scalar, Self::Scalar) -> Self::Scalar,
    ) -> Self {
        Self::generate(|i| f(self.nth(i), other.nth(i)))
    }

    /// Returns whether all pairs of components of `self` and `other` pass test closure `f`. Short circuits if any result is false.
    fn all_component_wise(
        &self,
        other: &Self,
        mut f: impl FnMut(Self::Scalar, Self::Scalar) -> bool,
    ) -> bool {
        (0..Self::DIMENSIONS).all(|i| f(self.nth(i), other.nth(i)))
    }

    /// Returns the dot product of `self` and `rhs`.
    fn dot(&self, rhs: &Self) -> Self::Scalar {
        self.component_wise(rhs, |l, r| l * r)
            .fold(Self::Scalar::ZERO, |acc, val| acc + val)
    }

    /// Folds (aka reduces or injects) the Point component wise using `f` and returns the result.
    /// fold() takes two arguments: an initial value, and a closure with two arguments: an 'accumulator', and the value of the current component.
    /// The closure returns the value that the accumulator should have for the next iteration.
    ///
    /// The `start_value` is the value the accumulator will have on the first call of the closure.
    ///
    /// After applying the closure to every component of the Point, fold() returns the accumulator.
    fn fold<T>(&self, start_value: T, mut f: impl FnMut(T, Self::Scalar) -> T) -> T {
        (0..Self::DIMENSIONS).fold(start_value, |accumulated, i| f(accumulated, self.nth(i)))
    }

    /// Returns a Point with every component set to `value`.
    fn from_value(value: Self::Scalar) -> Self {
        Self::generate(|_| value.clone())
    }

    /// Returns a Point with each component set to the smallest of each component pair of `self` and `other`.
    fn min_point(&self, other: &Self) -> Self {
        self.component_wise(other, min_inline)
    }

    /// Returns a Point with each component set to the biggest of each component pair of `self` and `other`.
    fn max_point(&self, other: &Self) -> Self {
        self.component_wise(other, max_inline)
    }

    /// Returns the squared length of this Point as if it was a vector.
    fn length_2(&self) -> Self::Scalar {
        self.fold(Self::Scalar::ZERO, |acc, cur| cur.clone() * cur + acc)
    }

    /// Substracts `other` from `self` component wise.
    fn sub(&self, other: &Self) -> Self {
        self.component_wise(other, |l, r| l - r)
    }

    /// Adds `other` to `self` component wise.
    fn add(&self, other: &Self) -> Self {
        self.component_wise(other, |l, r| l + r)
    }

    /// Multiplies `self` with `scalar` component wise.
    fn mul(&self, scalar: Self::Scalar) -> Self {
        self.map(|coordinate| coordinate * scalar.clone())
    }

    /// Applies `f` to `self` component wise.
    fn map(&self, mut f: impl FnMut(Self::Scalar) -> Self::Scalar) -> Self {
        Self::generate(|i| f(self.nth(i)))
    }

    /// Returns the squared distance between `self` and `other`.
    fn distance_2(&self, other: &Self) -> Self::Scalar {
        self.sub(other).length_2()
    }
}

#[inline]
pub(crate) fn min_inline<S>(a: S, b: S) -> S
where
    S: PartialOrd,
{
    if a < b { a } else { b }
}

#[inline]
pub(crate) fn max_inline<S>(a: S, b: S) -> S
where
    S: PartialOrd,
{
    if a > b { a } else { b }
}

impl<S, const N: usize> Point for [S; N]
where
    S: RTreeNum + core::fmt::Debug,
{
    type Scalar = S;

    const DIMENSIONS: usize = N;

    fn generate(mut generator: impl FnMut(usize) -> S) -> Self {
        // The same implementation used in std::array::from_fn
        // Since this is a const generic it gets unrolled
        let mut idx = 0;
        [(); N].map(|_| {
            let res = generator(idx);
            idx += 1;
            res
        })
    }

    #[inline]
    fn nth(&self, index: usize) -> Self::Scalar {
        self[index].clone()
    }

    #[inline]
    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        &mut self[index]
    }
}

macro_rules! count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + count_exprs!($($tail),*));
}

macro_rules! fixed_type {
    ($expr:expr, $type:ty) => {
        $type
    };
}

macro_rules! impl_point_for_tuple {
    ($($index:expr => $name:ident),+) => {
        impl<S> Point for ($(fixed_type!($index, S),)+)
        where
            S: RTreeNum + core::fmt::Debug
        {
            type Scalar = S;

            const DIMENSIONS: usize = count_exprs!($($index),*);

            fn generate(mut generator: impl FnMut(usize) -> S) -> Self {
                ($(generator($index),)+)
            }

            #[inline]
            fn nth(&self, index: usize) -> Self::Scalar {
                let ($($name,)+) = self;

                match index {
                    $($index => $name.clone(),)+
                    _ => unreachable!("index {} out of bounds for tuple", index),
                }
            }

            #[inline]
            fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
                let ($($name,)+) = self;

                match index {
                    $($index => $name,)+
                    _ => unreachable!("index {} out of bounds for tuple", index),
                }
            }
        }
    };
}

impl_point_for_tuple!(0 => a);
impl_point_for_tuple!(0 => a, 1 => b);
impl_point_for_tuple!(0 => a, 1 => b, 2 => c);
impl_point_for_tuple!(0 => a, 1 => b, 2 => c, 3 => d);
impl_point_for_tuple!(0 => a, 1 => b, 2 => c, 3 => d, 4 => e);
impl_point_for_tuple!(0 => a, 1 => b, 2 => c, 3 => d, 4 => e, 5 => f);
impl_point_for_tuple!(0 => a, 1 => b, 2 => c, 3 => d, 4 => e, 5 => f, 6 => g);
impl_point_for_tuple!(0 => a, 1 => b, 2 => c, 3 => d, 4 => e, 5 => f, 6 => g, 7 => h);
impl_point_for_tuple!(0 => a, 1 => b, 2 => c, 3 => d, 4 => e, 5 => f, 6 => g, 7 => h, 8 => i);
impl_point_for_tuple!(0 => a, 1 => b, 2 => c, 3 => d, 4 => e, 5 => f, 6 => g, 7 => h, 8 => i, 9 => j);

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::Wrapping;

    #[test]
    fn test_types() {
        fn assert_impl_rtreenum<S: RTreeNum>() {}

        assert_impl_rtreenum::<i8>();
        assert_impl_rtreenum::<i16>();
        assert_impl_rtreenum::<i32>();
        assert_impl_rtreenum::<i64>();
        assert_impl_rtreenum::<i128>();
        assert_impl_rtreenum::<isize>();
        assert_impl_rtreenum::<Wrapping<i8>>();
        assert_impl_rtreenum::<Wrapping<i16>>();
        assert_impl_rtreenum::<Wrapping<i32>>();
        assert_impl_rtreenum::<Wrapping<i64>>();
        assert_impl_rtreenum::<Wrapping<i128>>();
        assert_impl_rtreenum::<Wrapping<isize>>();
        assert_impl_rtreenum::<f32>();
        assert_impl_rtreenum::<f64>();
    }

    macro_rules! test_tuple_configuration {
        ($($index:expr),*) => {
            let a = ($($index),*);
            $(assert_eq!(a.nth($index), $index));*
        }
    }

    #[test]
    fn test_tuples() {
        // Test a couple of simple cases
        let simple_int = (0, 1, 2);
        assert_eq!(simple_int.nth(2), 2);
        let simple_float = (0.5, 0.67, 1234.56);
        assert_eq!(simple_float.nth(2), 1234.56);
        let long_int = (0, 1, 2, 3, 4, 5, 6, 7, 8);
        assert_eq!(long_int.nth(8), 8);

        // Generate the code to test every nth function for every Tuple length
        test_tuple_configuration!(0, 1);
        test_tuple_configuration!(0, 1, 2);
        test_tuple_configuration!(0, 1, 2, 3);
        test_tuple_configuration!(0, 1, 2, 3, 4);
        test_tuple_configuration!(0, 1, 2, 3, 4, 5);
        test_tuple_configuration!(0, 1, 2, 3, 4, 5, 6);
        test_tuple_configuration!(0, 1, 2, 3, 4, 5, 6, 7);
        test_tuple_configuration!(0, 1, 2, 3, 4, 5, 6, 7, 8);
    }
}

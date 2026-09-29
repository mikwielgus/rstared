//! Support for types from [`geo-types`](https://crates.io/crates/geo-types).
//!
//! Gated behind the `geo` feature. These impls live here because `rstar` is
//! vendored inside `rstared`; the crates.io `geo-types` `rstar_0_*` features
//! target a different `rstar` crate and cannot apply to these types.

use crate::rstar::primitives::Line as RstarLine;
use crate::rstar::{AABB, Envelope, Point as RTreePoint, PointDistance, RTreeNum, RTreeObject};

use geo_types::{Coord, CoordNum, Line, LineString, Point, Polygon, Rect};

impl<T> RTreePoint for Coord<T>
where
    T: CoordNum + RTreeNum,
{
    type Scalar = T;

    const DIMENSIONS: usize = 2;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        Coord {
            x: generator(0),
            y: generator(1),
        }
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.x,
            1 => self.y,
            _ => unreachable!(),
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => unreachable!(),
        }
    }
}

impl<T> RTreePoint for Point<T>
where
    T: CoordNum + RTreeNum,
{
    type Scalar = T;

    const DIMENSIONS: usize = 2;

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        Point::new(generator(0), generator(1))
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.x(),
            1 => self.y(),
            _ => unreachable!(),
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.0.x,
            1 => &mut self.0.y,
            _ => unreachable!(),
        }
    }
}

impl<T> RTreeObject for Line<T>
where
    T: CoordNum + RTreeNum,
{
    type Envelope = AABB<Point<T>>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.start_point(), self.end_point())
    }
}

impl<T> PointDistance for Line<T>
where
    T: CoordNum + RTreeNum,
{
    fn distance_2(&self, point: &Point<T>) -> T {
        RstarLine::new(self.start_point(), self.end_point()).distance_2(point)
    }
}

impl<T> RTreeObject for LineString<T>
where
    T: CoordNum + RTreeNum,
{
    type Envelope = AABB<Point<T>>;

    fn envelope(&self) -> Self::Envelope {
        let mut iter = self.points();

        let Some(first) = iter.next() else {
            return AABB::from_corners(
                Point::new(T::MIN_EXTENDED, T::MIN_EXTENDED),
                Point::new(T::MAX_EXTENDED, T::MAX_EXTENDED),
            );
        };

        let mut result = AABB::from_point(first);

        for point in iter {
            result.merge(&AABB::from_point(point));
        }

        result
    }
}

impl<T> PointDistance for LineString<T>
where
    T: CoordNum + RTreeNum,
{
    fn distance_2(&self, point: &Point<T>) -> T {
        if self.0.is_empty() {
            return T::ZERO;
        }

        self.lines()
            .map(|line| RstarLine::new(line.start_point(), line.end_point()).distance_2(point))
            .fold(T::MAX_EXTENDED, |accum, distance| {
                if distance < accum { distance } else { accum }
            })
    }
}

impl<T> RTreeObject for Polygon<T>
where
    T: CoordNum + RTreeNum,
{
    type Envelope = AABB<Point<T>>;

    fn envelope(&self) -> Self::Envelope {
        self.exterior().envelope()
    }
}

impl<T> RTreeObject for Rect<T>
where
    T: CoordNum + RTreeNum,
{
    type Envelope = AABB<Point<T>>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.min().into(), self.max().into())
    }
}

impl<T> PointDistance for Rect<T>
where
    T: CoordNum + RTreeNum,
{
    fn distance_2(&self, point: &Point<T>) -> T {
        self.envelope().distance_2(point)
    }
}

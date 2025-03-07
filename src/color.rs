use crate::tuple::Tuple;

pub trait Colors {
    fn black() -> Self;
    fn white() -> Self;
    fn red() -> Self;
    fn green() -> Self;
    fn blue() -> Self;
    fn purple() -> Self;
    fn orange() -> Self;
    fn yellow() -> Self;
}

impl Colors for Tuple {
    fn black() -> Self {
        Tuple::color(0.0, 0.0, 0.0)
    }

    fn white() -> Self {
        Tuple::color(1.0, 1.0, 1.0)
    }

    fn red() -> Self {
        Tuple::color(1.0, 0.0, 0.0)
    }

    fn green() -> Self {
        Tuple::color(0.0, 1.0, 0.0)
    }

    fn blue() -> Self {
        Tuple::color(0.0, 0.0, 1.0)
    }

    fn purple() -> Self {
        Tuple::color(0.5, 0.0, 0.5)
    }

    fn orange() -> Self {
        Tuple::color(1.0, 0.5, 0.0)
    }

    fn yellow() -> Self {
        Tuple::color(1.0, 1.0, 0.0)
    }
}
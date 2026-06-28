#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub lo: i16,
    pub hi: i16,
}

impl Span {
    /// Returns the size of the span, calculated as `hi - lo`.
    pub(crate) fn size(self) -> i16 {
        self.hi - self.lo
    }

    /// Computes the intersection of two spans.
    /// Returns `None` if there is no intersection, otherwise returns `Some(Span)`.
    pub(crate) fn intersection(self, other: Span) -> Option<Span> {
        let lo = self.lo.max(other.lo);
        let hi = self.hi.min(other.hi);

        if lo <= hi {
            Some(Span { lo, hi })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: Span,
    pub y: Span,
}

impl Rect {
    /// Computes the intersection of two rectangles.
    /// Returns `None` if there is no intersection, otherwise returns `Some(Rect)`.
    pub(crate) fn intersection(self, other: Rect) -> Option<Rect> {
        let x = self.x.intersection(other.x)?;
        let y = self.y.intersection(other.y)?;

        Some(Rect { x, y })
    }
}

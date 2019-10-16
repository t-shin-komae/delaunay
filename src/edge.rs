use std::cmp::PartialEq;
use crate::Point2D;
pub struct Edge {
    p1:Point2D,
    p2:Point2D,
}

impl Edge {
    pub fn new(p1:Point2D,p2:Point2D)->Self{
        Self{
            p1:p1,p2:p2,
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.p1 == other.p1 && self.p2 == other.p2) || (self.p1 == other.p2 && self.p2 == other.p1)
    }
}

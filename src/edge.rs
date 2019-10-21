use std::cmp::PartialEq;
use crate::Point2D;
use std::hash::{Hash,Hasher};
#[derive(Hash,Eq)]
pub struct Edge {
    pub p1:Point2D,
    pub p2:Point2D,
}

impl Edge {
    pub fn new(p1:Point2D,p2:Point2D)->Self{
        Self{
            p1:p1,p2:p2,
        }
    }
    pub fn has_point(&self,p:&Point2D) -> bool{
        self.p1 == *p || self.p2 == *p
    }
    pub fn into_points(&self)->[Point2D;2]{
        [self.p1,self.p2]
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.p1 == other.p1 && self.p2 == other.p2) || (self.p1 == other.p2 && self.p2 == other.p1)
    }
}


use std::cmp::PartialEq;
use crate::Point2D;
use std::hash::{Hash,Hasher};
/// 二次元上の辺を表す.
#[derive(Eq,Clone)]
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
    /// 辺が指定した点を含むかどうかを調べる.
    pub fn has_point(&self,p:&Point2D) -> bool{
        self.p1 == *p || self.p2 == *p
    }
    /// 辺から頂点を取り出す.
    pub fn get_points(&self)->[Point2D;2]{
        [self.p1,self.p2]
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.p1 == other.p1 && self.p2 == other.p2) || (self.p1 == other.p2 && self.p2 == other.p1)
    }
}
impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let padd = Point2D::new(self.p1.x+self.p2.x,self.p1.y+self.p2.y);
        //let psub = Point2D::new(self.p1.x*self.p2.x,self.p1.y*self.p2.y);
        padd.hash(state);
        //psub.hash(state);
    }
}
#[cfg(test)]
mod tests{
    #[test]
    fn test_edge_eq() {
        use super::*;
        let p1 = Point2D::new(1.,2.);
        let p2 = Point2D::new(5.,3.);
        assert!(Edge::new(p1,p2) == Edge::new(p1,p2));
        assert!(Edge::new(p1,p2) == Edge::new(p2,p1));
    }
}

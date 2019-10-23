pub (crate) use crate::{Mat3,MatOps};
use crate::Point2D;
use crate::Edge;
use std::hash::{Hash,Hasher};

/// 二次元上の三角形
///
/// 必ず右手周りにデータが保存される。
#[derive(Eq,Clone,Debug)]
pub struct Triangle {// 三次元空間上の点 必ず半時計回りに格納される
    pub(crate) p1:Point2D,
    pub(crate) p2:Point2D,
    pub(crate) p3:Point2D,
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        if self.p1 == other.p1 {
            self.p2 == other.p2 && self.p3 == other.p3
        }else if self.p1 == other.p2{
            self.p2 == other.p3 && self.p3 == other.p1
        }else if self.p1 == other.p3 {
            self.p2 == other.p1 && self.p3 == other.p2
        }else{
            false
        }
    }
}
impl Hash for Triangle{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let center = Point2D::new(self.p1.x+self.p2.x+self.p3.x,self.p1.y+self.p2.y+self.p3.y);
        center.hash(state);
    }
}
#[cfg(test)]
mod tests{
    #[test]
    fn test_tri_eq() {
        use super::*;
        let p1 = Point2D::new(1.3,2.4);
        let p2 = Point2D::new(1.8,1.4);
        let p3 = Point2D::new(2.7,2.4);
        for points in permutation3([p1,p2,p3]).iter(){
            Triangle::new(points[0],points[1],points[2]);
        }
    }
    fn permutation3<T:Copy>( a : [T;3]) -> [[T;3];6]{
        [
            [a[0],a[1],a[2]],
            [a[0],a[2],a[1]],
            [a[1],a[0],a[2]],
            [a[1],a[2],a[0]],
            [a[2],a[0],a[1]],
            [a[2],a[1],a[0]],
        ]
    }

    #[test]
    fn test_contain_edge() {
        use super::*;
        let p1 = Point2D::new(1.3,2.4);
        let p2 = Point2D::new(1.8,1.4);
        let p3 = Point2D::new(2.7,2.4);
        assert!(Triangle::new(p1,p2,p3).contain_edge(&Edge::new(p1,p2)))
    }
    #[test]
    fn test_contain_point() {
        use super::*;
        let p1 = Point2D::new(1.3,2.4);
        let p2 = Point2D::new(1.8,1.4);
        let p3 = Point2D::new(2.7,2.4);
        println!("{:?}",Triangle::new(p1,p2,p3));
        assert!(Triangle::new(p1,p2,p3).contain_point(&p2));
        assert!(Triangle::new(p1,p2,p3).contain_point(&p3));
        assert!(Triangle::new(p1,p2,p3).contain_point(&p1));
    }
}

impl Triangle{
    fn is_right_hand(p1:Point2D,p2:Point2D,p3:Point2D) -> bool {
        let v1 = p2-p1;
        let v2 = p3-p1;
        v1.x*v2.y - v1.y*v2.x > 0.
    }
    /// 三角形を生成する。どんな順番で引数を指定しても必ず右手周りにデータは格納される。
    pub fn new(p1:Point2D,p2:Point2D,p3:Point2D) -> Self {
        if Triangle::is_right_hand(p1, p2, p3) {
            Self{
                p1:p1,
                p2:p2,
                p3:p3,
            }
        }else{
            Self{
                p1:p1,
                p2:p3,
                p3:p2,
            }
        }
    }
    /// 三角形の辺を生成する。
    pub fn get_edges(&self) -> [Edge;3]{
        [
            Edge::new(self.p1,self.p2),
            Edge::new(self.p2,self.p3),
            Edge::new(self.p3,self.p1)
        ]
    }
    /// 三角形の頂点を生成する。
    pub fn get_points(&self) -> [Point2D;3]{
        [self.p1,self.p2,self.p3]
    }
    /// 三角形の外接円内に指定した点が含まれるかどうかの判定を行う.
    pub fn contain_in_circumscribed(&self,p:&Point2D)->bool{
        // http://www.thothchildren.com/chapter/5bdedb4341f88f267247fdd6
        let p1 = self.p1;
        let p2 = self.p2;
        let p3 = self.p3;
        let mat:Mat3 = [
            [p1.x-p.x,p1.y-p.y,(p1.x-p.x).powf(2.) + (p1.y-p.y).powf(2.)],
            [p2.x-p.x,p2.y-p.y,(p2.x-p.x).powf(2.) + (p2.y-p.y).powf(2.)],
            [p3.x-p.x,p3.y-p.y,(p3.x-p.x).powf(2.) + (p3.y-p.y).powf(2.)],
        ];
        mat.det()>0.
    }
    /// 三角形がその辺を含むかどうかを判定する.
    pub fn contain_edge(&self,edge:&Edge)-> bool{
        let (p1,p2) = (edge.p1,edge.p2);
        self.contain_point(&p1) && self.contain_point(&p2)
    }
    /// 三角形の頂点に指定した点が含まれるかどうかを判定する.
    pub fn contain_point(&self,point:&Point2D) -> bool{
        self.p1 == *point || self.p2==*point || self.p3==*point
    }

    /// 指定した辺に含まれていない点を一つ返す.
    ///
    /// 指定した辺が三角形に含まれていない場合、どの頂点を返すかは保証できない
    pub fn find_other_point_by_edge(&self,edge:&Edge) -> Option<Point2D>{
        for p in self.get_points().iter(){
            if !edge.has_point(p) {
                return Some(*p)
            }
        }
        return None
    }
}

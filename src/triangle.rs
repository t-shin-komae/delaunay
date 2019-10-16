pub (crate) use crate::{Mat3,MatOps};
use crate::Point2D;
use crate::Edge;
pub struct Triangle {// 三次元空間上の点 必ず半時計回りに格納される
    pub(crate) p1:Point2D,
    pub p2:Point2D,
    pub p3:Point2D,
}

impl Triangle{
    fn is_right_hand(p1:Point2D,p2:Point2D,p3:Point2D) -> bool {
        let v1 = p2-p1;
        let v2 = p3-p1;
        v1.x*v2.y - v1.y*v2.x > 0.
    }
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
    pub fn into_edges(&self) -> [Edge;3]{
        [
            Edge::new(self.p1,self.p2),
            Edge::new(self.p2,self.p3),
            Edge::new(self.p3,self.p1)
        ]
    }
    pub fn contain_in_circumscribed(&self,p:&Point2D)->bool{
        // http://www.thothchildren.com/chapter/5bdedb4341f88f267247fdd6
        let p1 = self.p1;
        let p2 = self.p2;
        let p3 = self.p3;
        let mat:Mat3 = [
            [p1.x-p.x,p1.y-p.y,(p1.x-p.x).powf(2.) + (p1.y-p.y).powf(2.)],
            [p2.x-p.x,p2.y-p.y,(p2.x-p.x).powf(2.) + (p2.y-p.y).powf(2.)],
            [p3.x-p.x,p1.y-p.y,(p1.x-p.x).powf(2.) + (p1.y-p.y).powf(2.)],
        ];
        mat.det()>0.
    }
}


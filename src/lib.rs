pub trait Point { // 3Dへの拡張性のため用意
    fn distance(self,other:Self) -> f32;
}

#[derive(Debug,Clone,Copy)]
pub struct Point2D { // 二次元の点
    x:f32,
    y:f32,
}

impl Point2D {
    pub fn new(x:f32,y:f32) -> Self {
        Self{x:x,y:y}
    }
}
impl std::ops::Sub for Point2D{
    type Output = Point2D;
    fn sub(self, other: Point2D) -> Self::Output{
        Point2D{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub struct Triangle {// 三次元空間上の点 必ず半時計回りに格納される
    p1:Point2D,
    p2:Point2D,
    p3:Point2D,
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
    pub fn contain_in_circumscribed(&self,p:Point2D)->bool{
        // http://www.thothchildren.com/chapter/5bdedb4341f88f267247fdd6
        let p1 = self.p1;
        let p2 = self.p2;
        let p3 = self.p3;
        let mat = [
            [p1.x-p.x,p1.y-p.y,(p1.x-p.x).powf(2.) + (p1.y-p.y).powf(2.)],
            [p2.x-p.x,p2.y-p.y,(p2.x-p.x).powf(2.) + (p2.y-p.y).powf(2.)],
            [p3.x-p.x,p1.y-p.y,(p1.x-p.x).powf(2.) + (p1.y-p.y).powf(2.)],
        ];
        mat.det()>0.
    }
}

impl Point for Point2D {
    fn distance(self,other: Self) -> f32 {
        (self.x-other.x).powf(2.) + (self.y-other.y).powf(2.)
    }
}

pub struct DelaunayTriangles{ // TODO ある点周りの三角形を求めやすいデータ構造が理想的
    triangles_set:Vec<Triangle>,
}

impl DelaunayTriangles {
    pub fn new() -> Self {
        Self{
            triangles_set:Vec::new()
        }
    }
}

type Mat3 = [[f32;3];3];
trait MatOps{
    fn det(&self) -> f32 ;
}
impl MatOps for Mat3{
    fn det(&self) -> f32 {
        let a = self;
        a[0][2]*a[1][0]*a[2][1] + a[0][1]*a[1][2]*a[2][0] + a[0][0]*a[1][1]*a[2][2]
            - a[0][0]*a[1][2]*a[2][1] - a[0][1]*a[1][0]*a[2][2] - a[0][2]*a[1][1]*a[2][0]
    }
}

fn large_rectangle(plist:&[Point2D]) -> (f32,f32,f32,f32){//全ての点を含む長方形 O(n)
    use std::f32::{INFINITY,NEG_INFINITY};
    let rec:(f32,f32,f32,f32) = plist.into_iter().fold((INFINITY,INFINITY,NEG_INFINITY,NEG_INFINITY),|acc,p|{ // x_min,y_min,x_max,y_max
        let (x_min,y_min,x_max,y_max);

        if p.x < acc.0 {
            x_min=p.x
        }else {
            x_min=acc.0
        }

        if p.y < acc.1 {
            y_min=p.y
        }else {
            y_min=acc.1
        }

        if p.x > acc.2 {
            x_max=p.x
        }else {
            x_max=acc.2
        }

        if p.y > acc.3 {
            y_max=p.y
        }else {
            y_max=acc.3
        }
        (x_min,y_min,x_max,y_max)
    });
    rec
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rec(){
        use super::*;
        let plist = [
            Point2D::new(3. ,2. ),
            Point2D::new(1. ,4. ),
            Point2D::new(3. ,5. ),
            Point2D::new(2. ,-2. ),
            Point2D::new(12. ,1. )
        ];
        assert_eq!(large_rectangle(&plist),(1.,-2.,12.,5.));
    }
    #[test]
    fn test_ext_tri(){// TODO 外部三角形用テスト
        use super::*;
        let plist = [
            Point2D::new(3. ,2. ),
            Point2D::new(1. ,4. ),
            Point2D::new(3. ,5. ),
            Point2D::new(2. ,-2. ),
            Point2D::new(12. ,1. )
        ];
        let tri = external_triangle(&plist);
        let v1 = (tri.p2.x-tri.p1.x,tri.p2.y-tri.p1.y);
        let v2 = (tri.p3.x-tri.p1.x,tri.p3.y-tri.p1.y);
        for p in &plist{ // TODO 逆行列を使ったがもう少しわかり易いテスト
            let pv = (p.x-tri.p1.x,p.y-tri.p1.y);
            let s = (v2.1*pv.0-v2.0*pv.1)/(v1.0*v2.1-v2.0*v1.1);
            let t = (-v1.1*pv.0+v1.0*pv.1)/(v1.0*v2.1-v2.0*v1.1);
            assert!((0.< s && s < 1.),"0<s<1: s={}",s);
            assert!((0. < t && t < 1.),"0<t<1: t={}",t) ;
            assert!((0. < s+t && s+t < 1.));
        }
    }
    fn test_contain_in_circumscribed(){
        use super::*;
        let tri = Triangle::new(
            Point2D::new(3. ,2. ),
            Point2D::new(1. ,4. ),
            Point2D::new(3. ,5. ));
        assert!(tri.contain_in_circumscribed(Point2D::new(3.,4.)));
        assert!(!tri.contain_in_circumscribed(Point2D::new(2.,1.)));
        assert!(!tri.contain_in_circumscribed(Point2D::new(2.,3.)));
    }
}


fn external_triangle(plist:&[Point2D]) -> Triangle {
    let rec = large_rectangle(&plist);
    let (width,height) = (rec.2-rec.0,rec.3-rec.1);
    
    let new_large_rec = (rec.0-width*0.025,rec.1-height*0.025,rec.2+width*0.025,rec.3+height*0.025);//少し大きな四角形を用意
    let new_width = width*1.05;
    let new_height = height*1.05;
    let lower_left = Point2D::new(new_large_rec.0 - new_height/1.73 , new_large_rec.1);
    let lower_right = Point2D::new(new_large_rec.2 + new_height/1.73 , new_large_rec.1);
    let top = Point2D::new((new_large_rec.0+new_large_rec.2)/2.,new_large_rec.3+new_width*1.74/2.);
    Triangle::new(lower_left, lower_right, top)
}

pub fn delaunay(plist:&[Point2D]) -> DelaunayTriangles{
    let dtri = DelaunayTriangles::new();
    dtri
}

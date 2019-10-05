pub trait Point { // 3Dへの拡張性のため用意
    fn distance(self,other:Self) -> f32;
}

pub struct Point2D { // 二次元の点
    x:f32,
    y:f32,
}

impl Point2D {
    pub fn new(x:f32,y:f32) -> Self {
        Self{x:x,y:y}
    }
}

pub struct Triangle<T:Point> {// 三次元空間上の点
    p1:T,
    p2:T,
    p3:T,
}

impl<T:Point> Triangle<T>{
    pub fn new(p1:T,p2:T,p3:T) -> Self {
        Self{
            p1:p1,
            p2:p2,
            p3:p3,
        }
    }
}

impl Point for Point2D {
    fn distance(self,other: Self) -> f32 {
        (self.x-other.x).powf(2.) + (self.y-other.y).powf(2.)
    }
}

pub struct DelaunayTriangles{ // TODO ある点周りの三角形を求めやすいデータ構造が理想的
    triangles_set:Vec<Triangle<Point2D>>,
}

impl DelaunayTriangles {
    pub fn new() -> Self {
        Self{
            triangles_set:Vec::new()
        }
    }
}

fn large_rectangle(plist:&[Point2D]) -> (f32,f32,f32,f32){//全ての点を含む長方形
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

    }
}


fn external_triangle(plist:&[Point2D]) -> Triangle<Point2D> {
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

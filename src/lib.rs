extern crate ordered_float;
pub mod point;
pub mod triangle;
pub mod utils;
pub mod edge;
pub use point::Point2D;
pub use triangle::Triangle;
pub use edge::Edge;
use std::collections::HashSet;
use utils::*;
pub struct DelaunayTriangles{ // TODO ある点周りの三角形を求めやすいデータ構造が理想的
    triangles_set:HashSet<Triangle>,
}

impl DelaunayTriangles {
    pub fn new(large_triangle:Triangle) -> Self {
        let mut triangles:HashSet<Triangle> = HashSet::new();
        triangles.insert(large_triangle);
        Self{
            triangles_set:triangles
        }
    }

    pub fn into_points(&self) -> HashSet<Point2D>{
        let mut points = HashSet::with_capacity(1000);
        self.triangles_set.iter().for_each(|tri|{
            points.insert(tri.p1);
            points.insert(tri.p2);
            points.insert(tri.p3);
        });
        points
    }

    pub fn add(&mut self,point:Point2D){
        let mut edges:Vec<Edge> = Vec::new();
        let target_tri = self.triangles_set.iter().find(|tri|tri.include(&point)).expect("Any triangles doesn't include this point").clone();
        
        let [e1,e2,e3] = target_tri.get_edges();
        edges.push(e1);
        edges.push(e2);
        edges.push(e3);
        edges.iter().map(|e| Triangle::new(e.p1,e.p2,point)).for_each(|tri|{
            self.triangles_set.insert(tri);
        });
        self.triangles_set.remove(&target_tri);
        
        // let mut edges:HashSet<Edge> =HashSet::new();
        // self.triangles_set
        //     .retain(|tri|{
        //         if tri.contain_in_circumscribed(&point){
        //             let [e1,e2,e3] = tri.get_edges();
        //             let mut dup_remove = |e:Edge|{
        //                 if !edges.insert(e.clone()){
        //                     edges.remove(&e);
        //                 }
        //             };
        //             dup_remove(e1);
        //             dup_remove(e2);
        //             dup_remove(e3);
        //             return false;
        //         }else{
        //             return true
        //         }
        //     }); // 線形探索なのでO(n)
            // .filter(|tri| tri.contain_in_circumscribed(&point)) // 線形探索なのでO(n)
            // .for_each(|tri| {
            //           let [e1,e2,e3] = tri.into_edges();
            //           edges.insert(e1);
            //           edges.insert(e2);
            //           edges.insert(e3);
            //     }
            // );
        // for tri in edges.iter().map(|e| Triangle::new(e.p1,e.p2,point)){
        //     self.triangles_set.insert(tri);
        // }
        // let mut edges:Vec<Edge> = edges.into_iter().collect();
        while let Some(edge) = edges.pop() {
            let mut edge_contained:Vec<Triangle> = Vec::new();
            self.triangles_set.retain(|tri| if tri.contain_edge(&edge){
                //エッジを含む
                edge_contained.push(tri.clone());
                false
            }else { // エッジを含まない
                true
            });// triangles_setからエッジを含む三角形を削除
            if edge_contained.len() == 2 {
                // TODO
                let other_p0 = edge_contained[0].find_other_point_by_edge(&edge).unwrap();
                let other_p1 = edge_contained[1].find_other_point_by_edge(&edge).unwrap();
                if edge_contained[0].contain_in_circumscribed(&other_p1){
                    let [p2,p3] = edge.get_points();
                    let new_tri1 = Triangle::new(other_p0,other_p1,p2);
                    let new_tri2 = Triangle::new(other_p0,other_p1,p3);
                    self.triangles_set.insert(new_tri1);
                    self.triangles_set.insert(new_tri2);
                    edges.push(Edge::new(other_p0,p2));
                    edges.push(Edge::new(other_p0,p3));
                    edges.push(Edge::new(other_p1,p2));
                    edges.push(Edge::new(other_p1,p3));
                }else{
                    self.triangles_set.insert(edge_contained.pop().unwrap());
                    self.triangles_set.insert(edge_contained.pop().unwrap());
                }
            }else if edge_contained.len()==1{
                self.triangles_set.insert(edge_contained.pop().unwrap());
            }
        }
    }
    pub fn print_triangles(&self) {
        for tri in self.triangles_set.iter(){
            println!("{},{},{},{},{},{}",tri.p1.x,tri.p1.y,tri.p2.x,tri.p2.y,tri.p3.x,tri.p3.y)
        }
    }
}


fn large_rectangle(plist:&[Point2D]) -> (f64,f64,f64,f64){//全ての点を含む長方形 O(n)
    use std::f64::{INFINITY,NEG_INFINITY};
    let rec:(f64,f64,f64,f64) = plist.into_iter().fold((INFINITY,INFINITY,NEG_INFINITY,NEG_INFINITY),|acc,p|{ // x_min,y_min,x_max,y_max
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
    #[test]
    fn test_contain_in_circumscribed(){
        use super::*;
        let tri = Triangle::new(
            Point2D::new(3. ,2. ),
            Point2D::new(1. ,4. ),
            Point2D::new(3. ,5. ));
        assert!(tri.contain_in_circumscribed(&Point2D::new(3.,4.)));
        assert!(!tri.contain_in_circumscribed(&Point2D::new(2.,1.)));
        assert!(!tri.contain_in_circumscribed(&Point2D::new(-2.,3.)));
    }
    #[test]
    fn test_add_point(){
        use super::*;
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut delau = DelaunayTriangles::new(Triangle::new(
            Point2D::new(-10. ,10. ),
            Point2D::new(0. ,-10. ),
            Point2D::new(10. ,10. )
            ));
        delau.add(Point2D::new(0.,0.));
        println!("Add origin");
        println!("{:?}",delau.triangles_set);
        let point_num = 50;
        let random_points:Vec<Point2D>= (0..point_num).map(|_|{
            Point2D::new(rng.gen::<f64>()*3. ,rng.gen::<f64>()*3.)
        }).collect();
        for p in random_points{
            println!("add point:{:?}",p);
            delau.add(p);
        }
        let all_points = delau.into_points();
        all_points.iter().for_each(|p|{
            delau.triangles_set.iter().for_each(|tri|{
                if tri.p1 != *p && tri.p2 != *p && tri.p3 != *p{
                    assert!(!tri.contain_in_circumscribed(&p));
                }
            });
        });
        delau.print_triangles();
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
    let dtri = DelaunayTriangles::new(external_triangle(plist));
    dtri
}

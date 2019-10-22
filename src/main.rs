extern crate rand;
use delaunay::point::Point2D;
use delaunay::*;
use rand::Rng;
use std::env;
use std::collections::HashSet;
use std::io::Write;
fn main() {
        let args: Vec<String> = env::args().collect();
        let num:usize = args[1].parse().unwrap();
        let mut rng = rand::thread_rng();
        let mut delau = DelaunayTriangles::new(Triangle::new(
            Point2D::new(-1000. ,1000. ),
            Point2D::new(0. ,-1000. ),
            Point2D::new(1000. ,1000. )
            ));
        delau.add(Point2D::new(0.,0.));
        //let point_num = num;
        let point_num = num;
        let random_points:HashSet<Point2D>= (0..point_num).map(|_|{
            Point2D::new(rng.gen::<f64>()*200. ,rng.gen::<f64>()*200.)
        }).collect();
        let mut i = 0;
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        for p in random_points{
            delau.add(p);
            i+=1;
            write!(handle,"\r{}",i).unwrap();
        }
        println!("");
}

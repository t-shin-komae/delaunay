use std::hash::{Hash,Hasher};
use std::cmp::Eq;
use rand::Rng;
use rand::distributions::{Distribution, Standard};
use ordered_float::NotNan;
pub trait Point { // 3Dへの拡張性のため用意
    fn distance(self,other:Self) -> f64;
}

#[derive(Debug,Clone,Copy)]
pub struct Point2D { // 二次元の点
    pub x:f64,
    pub y:f64,
}

impl Point2D {
    pub fn new(x:f64,y:f64) -> Self {
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

impl Point for Point2D {
    fn distance(self,other: Self) -> f64 {
        (self.x-other.x).powf(2.) + (self.y-other.y).powf(2.)
    }
}

impl Hash for Point2D {
    fn hash<H:Hasher>(&self,state:&mut H) {
        NotNan::new(self.x).expect("Floating Point Error\nFound NaN").hash(state);
        NotNan::new(self.y).expect("Floating Point Error\nFound NaN").hash(state);
    }
}
impl PartialEq for Point2D {
    fn eq(&self, other: &Self) -> bool {
        NotNan::new(self.x).expect("Floating Point Error\nFound NaN")==
        NotNan::new(other.x).expect("Floating Point Error\nFound NaN") 
        &&
        NotNan::new(self.y).expect("Floating Point Error\nFound NaN")==
        NotNan::new(other.y).expect("Floating Point Error\nFound NaN")
    }
}

impl Eq for Point2D {}

#[cfg(test)]
mod tests{
    #[test]
    fn test_point_eq() {
        use super::*;
        let p1 = Point2D::new(3.,2.) ;
        assert!(p1==p1);
    }
}

impl Distribution<Point2D> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point2D {
        let (rand_x, rand_y) = rng.gen();
        Point2D {
            x: rand_x,
            y: rand_y,
        }
    }
}

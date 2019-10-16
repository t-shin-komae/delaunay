pub trait Point { // 3Dへの拡張性のため用意
    fn distance(self,other:Self) -> f32;
}

#[derive(PartialEq,Debug,Clone,Copy)]
pub struct Point2D { // 二次元の点
    pub x:f32,
    pub y:f32,
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

impl Point for Point2D {
    fn distance(self,other: Self) -> f32 {
        (self.x-other.x).powf(2.) + (self.y-other.y).powf(2.)
    }
}


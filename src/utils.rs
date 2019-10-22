pub(crate)type Mat3 = [[f64;3];3];
pub (crate)trait MatOps{
    fn det(&self) -> f64 ;
}
impl MatOps for Mat3{
    fn det(&self) -> f64 {
        let a = self;
        a[0][2]*a[1][0]*a[2][1] + a[0][1]*a[1][2]*a[2][0] + a[0][0]*a[1][1]*a[2][2]
            - a[0][0]*a[1][2]*a[2][1] - a[0][1]*a[1][0]*a[2][2] - a[0][2]*a[1][1]*a[2][0]
    }
}


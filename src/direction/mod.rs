struct Direction {
    polar_angle:f64,
    azmuthal_angle:f64,
}

impl Direction {
    fn new(p,a)->Direction{
        Direction{polar_angle:p,azmuthal_angle:a}
    }
    mod Vector;
    fn to_vec(&self)->Vector{
        let s = sin(this.polar_angle);
        Vector::new(s*cos(this.azmuthal_angle),s*sin(this.azmuthal_angle),cos(this.polar_angle))
    }
}

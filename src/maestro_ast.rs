// derive(Debug) macro enables use of "{:?}" in println!
#[derive(Debug)]
pub enum Directive {
    SpacialMap { dimention: String, size: u32, offset: u32},
    TemporalMap { dimention: String, size: u32, offset: u32},
    Cluster {size: u32}
}
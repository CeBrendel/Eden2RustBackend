

pub trait Optimizer {
    const IS_MAXIMIZER: bool;
    type Opposite: Optimizer;
    fn compare(old: f32, new: f32) -> bool;
    fn compare_for_assign(old: f32, new: f32) -> f32;
}


pub struct Maximizer;

pub struct Minimizer;


impl Optimizer for Maximizer {
    const IS_MAXIMIZER: bool = true;
    type Opposite = Minimizer;
    #[inline(always)]
    fn compare(old: f32, new: f32) -> bool {
        new > old
    }
    #[inline(always)]
    fn compare_for_assign(old: f32, new: f32) -> f32 {
        old.max(new)
    }
}


impl Optimizer for Minimizer {
    const IS_MAXIMIZER: bool = false;
    type Opposite = Maximizer;
    #[inline(always)]
    fn compare(old: f32, new: f32) -> bool {
        new < old
    }
    #[inline(always)]
    fn compare_for_assign(old: f32, new: f32) -> f32 {
        old.min(new)
    }
}

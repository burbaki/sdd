pub mod page;
pub mod block;


pub trait FluctuareT {
    fn fluctuate(&self, write_count: u32, value: u8) -> u8;
}

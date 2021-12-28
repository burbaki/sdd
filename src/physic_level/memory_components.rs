pub mod block;
pub mod page;

pub trait FluctuareT {
    fn fluctuate(&self, write_count: u32, value: u8) -> u8;
}

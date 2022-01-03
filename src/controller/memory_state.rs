use crate::controller::CellState;
use crate::physic_level::memory::Address;
use std::collections::HashMap;
use std::ops::Range;
trait MemoryState {
    fn get_memory_state(&self, block_range: Range<usize>) -> HashMap<Address, CellState>;
    fn set_memory_state(&mut self, block: usize, page_range: Range<usize>, state: CellState);
}

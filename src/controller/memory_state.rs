use std::collections::HashMap;
use std::ops::Range;
use crate::controller::CellState;
use crate::physic_level::memory::Address;
trait MemoryState {
    fn get_memory_state(block_range: Range<usize>) -> HashMap<Address, CellState>;
    fn set_memory_state(block: usize, page_range: Range<usize>, state: CellState);
}
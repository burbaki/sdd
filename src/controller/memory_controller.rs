use crate::controller::CellType;
use crate::physic_level::memory::Address;
use crate::controller::byte_encoder::ByteEncoder;
use crate::metric::metric_storage::MetricStorage;
use crate::physic_level::memory::Memory;
trait MemoryController {
    fn write_bits(&mut self, bits: Vec<bool>, adress: Address, cell_type: CellType);
    fn read_bits(&self, adress: Address, cell_type: CellType) -> Vec<bool>;
}

pub struct MemoryControllerImpl {
    byte_encoder: Box<dyn ByteEncoder>,
    metric_storage: Box<dyn MetricStorage>,
    memory: Box<dyn Memory>
}
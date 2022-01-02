use crate::controller::CellType;
use crate::config::CELLS_PER_PAGE;
pub trait ByteEncoder {
    fn encode_bytes_to_page(&self, bytes: Vec<bool>, cell_type: CellType) -> [u8; CELLS_PER_PAGE];

    fn encode_page_to_bytes(&self, cell: [u8; CELLS_PER_PAGE], cell_type: CellType) -> Vec<bool>;

}
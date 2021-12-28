use crate::controller::CellType;
use crate::config::CELLS_PER_PAGE;
trait ByteEncoder {
    fn encode_bytes_to_page(bytes: Vec<bool>, cell_type: CellType) -> [u8; CELLS_PER_PAGE];

    fn encode_page_to_bytes(cell: [u8; CELLS_PER_PAGE], cell_type: CellType) -> Vec<bool>;

}
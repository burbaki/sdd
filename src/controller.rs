mod memory_state;
mod byte_encoder;
mod memory_controller;
pub enum CellType {
    Single,
    Double,
    Triple,
    Quadro,
    Penta,
}
impl CellType {
    pub fn multiplier(&self) -> u8 {
        match self {
            Single => 1,
            Double => 2,
            Triple => 3,
            Quadro => 4,
            Penta => 5,
        }
    }
}


enum CellState {
    Empty,
    Set(CellType),
    ResetPending,    
}
mod byte_encoder;
mod memory_controller;
mod memory_state;
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
            CellType::Single => 1,
            CellType::Double => 2,
            CellType::Triple => 3,
            CellType::Quadro => 4,
            CellType::Penta => 5,
        }
    }
}

enum CellState {
    Empty,
    Set(CellType),
    ResetPending,
}

enum OperationType {
    Read,
    Write,
    Delete,
}

fn operation_time(cell_type: CellType, operation_type: OperationType) -> u32 {
    match (cell_type, operation_type) {
        (CellType::Single, OperationType::Read) => 3,
        (CellType::Double, OperationType::Read) => 6,
        (CellType::Triple, OperationType::Read) => 12,
        (CellType::Quadro, OperationType::Read) => 25,
        (CellType::Penta, OperationType::Read) => 50,
        (CellType::Single, OperationType::Write) => 20,
        (CellType::Double, OperationType::Write) => 60,
        (CellType::Triple, OperationType::Write) => 150,
        (CellType::Quadro, OperationType::Write) => 300,
        (CellType::Penta, OperationType::Write) => 600,
        (CellType::Single, OperationType::Delete) => 350,
        (CellType::Double, OperationType::Delete) => 700,
        (CellType::Triple, OperationType::Delete) => 1400,
        (CellType::Quadro, OperationType::Delete) => 2200,
        (CellType::Penta, OperationType::Delete) => 3000,
    }
}

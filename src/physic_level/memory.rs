use crate::config::CELLS_PER_PAGE;
use crate::config::PAGES_PER_BLOCK;
use crate::physic_level::memory_components::*;
use rand::Rng;

pub trait Memory {
    fn read(&self, address: Address) -> &[u8; CELLS_PER_PAGE];
    fn program(&mut self, address: Address, data: [u8; CELLS_PER_PAGE]) -> ();
    fn reset(&mut self, lock_id: usize) -> ();
}

struct MemoryImpl {
    fluctuator: Box<dyn FluctuareT>,
    blocks: Vec<block::Block<CELLS_PER_PAGE, PAGES_PER_BLOCK>>,
}

impl MemoryImpl {
    fn new(fluctuator: Box<dyn FluctuareT>, blocks_amount: usize) -> MemoryImpl {
        let mut blocks = Vec::new();

        for _ in 1..blocks_amount {
            blocks.push(block::Block::new())
        }

        MemoryImpl {
            fluctuator: fluctuator,
            blocks: blocks,
        }
    }
}

impl Memory for MemoryImpl {
    fn read(&self, address: Address) -> &[u8; CELLS_PER_PAGE] {
        let Address(block_id, page_id) = address;
        self.blocks[block_id].read(page_id)
    }
    fn program(&mut self, address: Address, data: [u8; CELLS_PER_PAGE]) -> () {
        let Address(block_id, page_id) = address;
        self.blocks[block_id].program(page_id, data, &mut *self.fluctuator)
    }

    fn reset(&mut self, block_id: usize) -> () {
        self.blocks[block_id].reset()
    }
}

struct ProdFluctuate {}
impl FluctuareT for ProdFluctuate {
    fn fluctuate(&self, count: u32, value: u8) -> u8 {
        let fluctuation_size = 1000.0 / (1100.0 - count as f64);
        dbg!(fluctuation_size);
        let is_fluctuation_up = rand::thread_rng().gen_bool(0.5);
        let res = if is_fluctuation_up {
            value.checked_add((fluctuation_size) as u8)
        } else {
            value.checked_sub((fluctuation_size) as u8)
        };
        dbg!(res);
        match res {
            Some(v) => v,
            None => {
                if is_fluctuation_up {
                    255
                } else {
                    0
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryInto;
    #[test]
    fn fluctuate_should_return_fluctuated_value() -> () {
        let target: &dyn FluctuareT = &ProdFluctuate {};
        let count = 101;
        let value = 127;

        for _ in 1..10 {
            let res = target.fluctuate(count, value);
            dbg!(res);
            assert!(res == 126 || res == 128)
        }
    }
    #[test]
    fn fluctuate_should_not_overflow() -> () {
        let target: &dyn FluctuareT = &ProdFluctuate {};
        let count = 100;
        let value = 0;

        for _ in 1..10 {
            let res = target.fluctuate(count, value);
            assert!(res == 0 || res == 1)
        }
    }

    #[test]
    fn program_should_save_value() -> () {
        let mut target = MemoryImpl::new(Box::new(ZERO_FLU), 8);
        let mut cells = Vec::new();
        for i in 0..CELLS_PER_PAGE {
            cells.push(i as u8);
        }
        let cells_for_save: [u8; CELLS_PER_PAGE] = cells.try_into().unwrap();
        let address = Address(2, 12);
        target.program(address, cells_for_save);
        let res = target.read(address);

        assert_eq!(cells_for_save, *res)
    }
    #[test]
    fn reset_should_delete_values_in_block() -> () {
        let mut target = MemoryImpl::new(Box::new(ZERO_FLU), 8);
        let mut cells = Vec::new();
        for i in 0..CELLS_PER_PAGE {
            cells.push(i as u8);
        }
        let cells_for_save: [u8; CELLS_PER_PAGE] = cells.try_into().unwrap();
        let block = 2;
        for i in 0..PAGES_PER_BLOCK {
            target.program(Address(block, i), cells_for_save);
        }
        target.reset(block);
        for i in 0..PAGES_PER_BLOCK {
            let res = target.read(Address(block, i));
            assert_eq!([0 as u8; CELLS_PER_PAGE], *res)
        }
    }

    struct ZeroFluctuate;
    impl FluctuareT for ZeroFluctuate {
        fn fluctuate(&self, _: u32, value: u8) -> u8 {
            value
        }
    }

    const ZERO_FLU: ZeroFluctuate = ZeroFluctuate {};
}

// Address(block page)
#[derive(Clone, Copy)]
pub struct Address(usize, usize);

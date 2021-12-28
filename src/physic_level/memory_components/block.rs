use crate::physic_level::memory_components;
use crate::physic_level::memory_components::page::Page;
use std::convert::TryInto;

pub struct Block<const PS: usize, const BS: usize> {
    pages: [Page<PS>; BS],
}

impl<const PS: usize, const BS: usize> Block<PS, BS> {
    pub fn new() -> Block<PS, BS> {
        let mut pages_v = Vec::new();
        for _ in 0..BS {
            pages_v.push(Page::new())
        }

        Block {
            pages: pages_v.try_into().unwrap_or_else(|v: Vec<Page<PS>>| {
                panic!("Expected a Vec of length {} but it was {}", PS, v.len())
            }),
        }
    }

    pub fn read(&self, page_id: usize) -> &[u8; PS] {
        self.pages[page_id].read()
    }

    pub fn program(
        &mut self,
        page_id: usize,
        data: [u8; PS],
        f: &dyn memory_components::FluctuareT,
    ) -> () {
        self.pages[page_id].program(data, f)
    }

    pub fn reset(&mut self) -> () {
        for i in 0..self.pages.len() {
            self.pages[i].reset();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn program_should_save_value_in_given_page() -> () {
        let mut target = setup_target();
        let data: [u8; PAGE_SIZE] = [64, 64, 0, 0];
        let page_id = 1;

        target.program(page_id, data, &ZERO_FLU);

        let res = target.read(page_id);
        assert_eq!(data, *res);
    }
    #[test]
    fn reset_should_reset_all_pages() -> () {
        let mut target = setup_target();
        let data: [u8; PAGE_SIZE] = [64, 64, 0, 0];

        for i in 0..(PAGE_SIZE) {
            target.program(i, data, &ZERO_FLU);
        }

        for i in 0..(PAGE_SIZE) {
            assert_eq!(data, *target.read(i));
        }

        target.reset();
        for i in 0..(PAGE_SIZE) {
            assert_eq!([0; PAGE_SIZE], *target.read(i));
        }
    }

    fn setup_target() -> Block<PAGE_SIZE, BLOCK_SIZE> {
        Block::new()
    }

    const PAGE_SIZE: usize = 4;
    const BLOCK_SIZE: usize = 4;
    struct ZeroFluctuate;
    impl memory_components::FluctuareT for ZeroFluctuate {
        fn fluctuate(&self, _: u32, value: u8) -> u8 {
            value
        }
    }

    const ZERO_FLU: ZeroFluctuate = ZeroFluctuate {};
}

pub mod page {
    use crate::physic_level::memory_components;
    pub struct Page<const PC: usize> {
        cells: [u8; PC],
        pub write_count: u32,
    }
    impl<const PC: usize> Page<PC> {
        pub fn new() -> Page<PC> {
            Page {
                cells: [0; PC],
                write_count: 0,
            }
        }

        pub fn program<F: memory_components::FluctuareT>(&mut self, data: [u8; PC]) -> () {
            for (i, e) in data.iter().enumerate() {
                if self.cells[i] == 0 {
                    self.cells[i] = F::fluctuate(self.write_count, *e);
                } else {
                    panic!("Cannot program non-empty cell")
                }
            }
        }

        pub fn read(&self) -> &[u8; PC] {
            &self.cells
        }

        pub fn reset(&mut self) -> () {
            for i in 0..self.cells.len() {
                self.cells[i] = 0;
            }
            self.write_count = self.write_count + 1;
        }

        fn fluctuate_cell_value(write_count: u32, value: u8) -> u8 {
            value
        }
    }
    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn program_should_save_value() -> () {
            let mut target: Page<PAGE_SIZE> = Page::new();
            let data: [u8; PAGE_SIZE] = [64, 64, 0, 0];

            target.program::<ZeroFluctuate>(data);

            let res = target.read();
            assert_eq!(data, *res);
        }
        #[test]
        fn reset_should_reset_value_and_inc_count() -> () {
            let mut target: Page<PAGE_SIZE> = Page::new();
            let data: [u8; PAGE_SIZE] = [64, 64, 0, 0];

            let count_size_before_reset = target.write_count;

            target.program::<ZeroFluctuate>(data);
            assert_eq!(data, *target.read());

            target.reset();
            assert_eq!([0; PAGE_SIZE], *target.read());

            let count_size_after_reset = target.write_count;
            assert_eq!(1, count_size_after_reset - count_size_before_reset)
        }

        const PAGE_SIZE: usize = 4;

        struct ZeroFluctuate;
        impl memory_components::FluctuareT for ZeroFluctuate {
            fn fluctuate(_: u32, value: u8) -> u8 {
                value
            }
        }
    }
}

pub mod block {

    use crate::physic_level::memory_components;
    use crate::physic_level::memory_components::page::Page;
    use std::convert::TryInto;

    pub struct Block<const PC: usize, const BC: usize> {
        pages: [Page<PC>; BC],
    }

    impl<const PC: usize, const BC: usize> Block<PC, BC> {
        pub fn new() -> Block<PC, BC> {
            let mut pages_v = Vec::new();
            for _ in 0..BC {
                pages_v.push(Page::new())
            }

            Block {
                pages: pages_v.try_into().unwrap_or_else(|v: Vec<Page<PC>>| {
                    panic!("Expected a Vec of length {} but it was {}", PC, v.len())
                }),
            }
        }

        pub fn read(&self, page_id: usize) -> &[u8; PC] {
            self.pages[page_id].read()
        }

        pub fn program<F: memory_components::FluctuareT>(
            &mut self,
            page_id: usize,
            data: [u8; PC],
        ) -> () {
            self.pages[page_id].program::<F>(data)
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

            target.program::<ZeroFluctuate>(page_id, data);

            let res = target.read(page_id);
            assert_eq!(data, *res);
        }
        #[test]
        fn reset_should_reset_all_pages() -> () {
            let mut target = setup_target();
            let data: [u8; PAGE_SIZE] = [64, 64, 0, 0];

            for i in 0..(PAGE_SIZE) {
                target.program::<ZeroFluctuate>(i, data);
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
            fn fluctuate(_: u32, value: u8) -> u8 {
                value
            }
        }
    }
}

pub trait FluctuareT {
    fn fluctuate(write_count: u32, value: u8) -> u8;
}

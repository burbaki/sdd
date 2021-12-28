use crate::physic_level::memory_components;
pub struct Page<const PS: usize> {
    cells: [u8; PS],
    pub write_count: u32,
}
impl<const PS: usize> Page<PS> {
    pub fn new() -> Page<PS> {
        Page {
            cells: [0; PS],
            write_count: 0,
        }
    }

    pub fn program(&mut self, data: [u8; PS], f: &dyn memory_components::FluctuareT) -> () {
        for (i, e) in data.iter().enumerate() {
            if self.cells[i] == 0 {
                self.cells[i] = f.fluctuate(self.write_count, *e);
            } else {
                panic!("Cannot program non-empty cell")
            }
        }
    }

    pub fn read(&self) -> &[u8; PS] {
        &self.cells
    }

    pub fn reset(&mut self) -> () {
        for i in 0..self.cells.len() {
            self.cells[i] = 0;
        }
        self.write_count = self.write_count + 1;
    }    
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn program_should_save_value() -> () {
        let mut target: Page<PAGE_SIZE> = Page::new();
        let data: [u8; PAGE_SIZE] = [64, 64, 0, 0];

        target.program(data, &ZERO_FLU);

        let res = target.read();
        assert_eq!(data, *res);
    }
    #[test]
    fn reset_should_reset_value_and_inc_count() -> () {
        let mut target: Page<PAGE_SIZE> = Page::new();
        let data: [u8; PAGE_SIZE] = [64, 64, 0, 0];

        let count_size_before_reset = target.write_count;

        target.program(data, &ZERO_FLU);
        assert_eq!(data, *target.read());

        target.reset();
        assert_eq!([0; PAGE_SIZE], *target.read());

        let count_size_after_reset = target.write_count;
        assert_eq!(1, count_size_after_reset - count_size_before_reset)
    }

    const PAGE_SIZE: usize = 4;

    struct ZeroFluctuate;
    impl memory_components::FluctuareT for ZeroFluctuate {
        fn fluctuate(&self, _: u32, value: u8) -> u8 {
            value
        }
    }

    const ZERO_FLU: ZeroFluctuate = ZeroFluctuate {};
}

use crate::controller::CellType;
use std::convert::TryInto;

pub trait ByteEncoder<const PS: usize> {
    fn encode_bytes_to_page(&self, bits: Vec<bool>, cell_type: CellType) -> [u8; PS];

    fn decode_page_to_bytes(&self, cells: [u8; PS], cell_type: CellType) -> Vec<bool>;
}

struct ByteEncoderImpl {}

impl ByteEncoderImpl {
    fn new() -> ByteEncoderImpl {
        ByteEncoderImpl {}
    }
    fn bit_slice_to_int(slice: &[bool]) -> u8 {
        let mut res: u8 = 0;

        for (indx, s) in slice.iter().rev().enumerate() {
            res += if *s { 1 << indx } else { 0 }
        }
        res
    }

    fn int_to_bit_slice(val: u8, dimension: usize) -> Vec<bool> {
        let mut left = val;
        let mut temporary_vec = Vec::new();
        while left >= 1 {
            let remainder = left % 2;
            if dbg!(remainder) == 1 {
                temporary_vec.push(true)
            } else {
                temporary_vec.push(false)
            };
            left = left / 2;
        }

        let zero_should_add = dimension - temporary_vec.len();
        let mut res = vec![false; zero_should_add];
        temporary_vec.reverse();
        res.append(&mut temporary_vec);
        res
    }
}

impl<const PS: usize> ByteEncoder<PS> for ByteEncoderImpl {
    fn encode_bytes_to_page(&self, bits: Vec<bool>, cell_type: CellType) -> [u8; PS] {
        if dbg!(bits.len()) / dbg!(cell_type.multiplier()) as usize != PS {
            panic!("mismatch bits size and multiplier")
        }

        let chunked = bits.chunks(cell_type.multiplier() as usize);
        let mut temporary_vec = Vec::new();
        for chunk in chunked {
            let section_number = ByteEncoderImpl::bit_slice_to_int(chunk);
            let size_of_section = 255 / (1 << cell_type.multiplier()) + 1;
            let section_start = dbg!(section_number) * dbg!(size_of_section);
            let section_end = section_number * size_of_section + (size_of_section - 1);
            temporary_vec.push((dbg!(section_end) - dbg!(section_start)) / 2 + section_start)
        }

        temporary_vec
            .try_into()
            .unwrap_or_else(|_| panic!("vec size is wrong"))
    }

    fn decode_page_to_bytes(&self, cells: [u8; PS], cell_type: CellType) -> Vec<bool> {
        let sections_count = 1 << cell_type.multiplier();
        let size_of_section = 255 / sections_count + 1;
        let mut res = Vec::new();
        for cell in cells.iter() {
            let mut begin = 0;
            let mut end = size_of_section - 1;
            let mut section_number = 0;
            while !(*cell >= begin && *cell <= end) {
                section_number = section_number + 1;
                begin = section_number * size_of_section;
                end = section_number * size_of_section + (size_of_section - 1);
            }
            let mut bytes =
                ByteEncoderImpl::int_to_bit_slice(section_number, cell_type.multiplier() as usize);
            res.append(&mut bytes)
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bit_slice_to_int_should_convert_zero() {
        let in_arg = &[false];

        let res = ByteEncoderImpl::bit_slice_to_int(in_arg);

        assert_eq!(0, res);
    }
    #[test]
    fn bit_slice_to_int_should_convert_max_value() {
        let in_arg = &[true; 5];

        let res = ByteEncoderImpl::bit_slice_to_int(in_arg);

        assert_eq!(31, res);
    }
    #[test]
    fn bit_slice_to_int_should_convert_11() {
        let in_arg = &[true, false, true, true];

        let res = ByteEncoderImpl::bit_slice_to_int(in_arg);

        assert_eq!(11, res);
    }

    #[test]
    fn bit_slice_to_int_should_convert_16() {
        let in_arg = &[true, false, false, false, false];

        let res = ByteEncoderImpl::bit_slice_to_int(in_arg);

        assert_eq!(16, res);
    }

    #[test]
    fn int_to_bit_slice_should_convert_zero() {
        let in_arg = 0;
        let dimension: usize = 4;

        let res = ByteEncoderImpl::int_to_bit_slice(in_arg, dimension);

        assert_eq!(vec![false; dimension], res);
    }

    #[test]
    fn int_to_bit_slice_should_convert_value_in_middle() {
        let in_arg = 11;
        let dimension: usize = 4;

        let res = ByteEncoderImpl::int_to_bit_slice(in_arg, dimension);

        assert_eq!(vec![true, false, true, true], res);
    }

    #[test]
    fn int_to_bit_slice_should_convert_max_value() {
        let in_arg = 15;
        let dimension: usize = 4;

        let res = ByteEncoderImpl::int_to_bit_slice(in_arg, dimension);

        assert_eq!(vec![true; dimension], res);
    }

    #[test]
    fn bit_convertation_should_work() {
        let decimal = 17;
        let dimension = 5;

        let bits = ByteEncoderImpl::int_to_bit_slice(decimal, dimension);

        let decimal_res = ByteEncoderImpl::bit_slice_to_int(&bits[..]);

        assert_eq!(decimal, decimal_res);
    }

    #[test]
    fn encode_bytes_to_page_should_return_right_section_for_zero() {
        let target: ByteEncoderImpl = ByteEncoderImpl::new();

        let in_vec_1 = vec![false; 1];
        let res_1 = target.encode_bytes_to_page(in_vec_1, CellType::Single);
        assert_eq!([63], res_1);

        let in_vec_2 = vec![false; 2];
        let res_2 = target.encode_bytes_to_page(in_vec_2, CellType::Double);
        assert_eq!([31], res_2);

        let in_vec_3 = vec![false; 3];
        let res_3 = target.encode_bytes_to_page(in_vec_3, CellType::Triple);
        assert_eq!([15], res_3);

        let in_vec_4 = vec![false; 4];
        let res_4 = target.encode_bytes_to_page(in_vec_4, CellType::Quadro);
        assert_eq!([7], res_4);

        let in_vec_5 = vec![false; 5];
        let res_5 = target.encode_bytes_to_page(in_vec_5, CellType::Penta);
        assert_eq!([3], res_5);
    }
    #[test]
    fn encode_bytes_to_page_should_return_right_section_for_max() {
        let target: ByteEncoderImpl = ByteEncoderImpl::new();

        let in_vec_1 = vec![true; 1];
        let res_1 = target.encode_bytes_to_page(in_vec_1, CellType::Single);
        assert_eq!([191], res_1);

        let in_vec_2 = vec![true; 2];
        let res_2 = target.encode_bytes_to_page(in_vec_2, CellType::Double);
        assert_eq!([223], res_2);

        let in_vec_3 = vec![true; 3];
        let res_3 = target.encode_bytes_to_page(in_vec_3, CellType::Triple);
        assert_eq!([239], res_3);

        let in_vec_4 = vec![true; 4];
        let res_4 = target.encode_bytes_to_page(in_vec_4, CellType::Quadro);
        assert_eq!([247], res_4);

        let in_vec_5 = vec![true; 5];
        let res_5 = target.encode_bytes_to_page(in_vec_5, CellType::Penta);
        assert_eq!([251], res_5);
    }

    #[test]
    fn encode_bytes_to_page_should_return_right_section_for_middle_val() {
        let target: ByteEncoderImpl = ByteEncoderImpl::new();

        let in_vec_2 = vec![true, false];
        let res_2 = target.encode_bytes_to_page(in_vec_2, CellType::Double);
        assert_eq!([159], res_2);

        let in_vec_3 = vec![true, false, false];
        let res_3 = target.encode_bytes_to_page(in_vec_3, CellType::Triple);
        assert_eq!([143], res_3);

        let in_vec_4 = vec![true, false, false, false];
        let res_4 = target.encode_bytes_to_page(in_vec_4, CellType::Quadro);
        assert_eq!([135], res_4);

        let in_vec_5 = vec![true, false, false, false, false];
        let res_5 = target.encode_bytes_to_page(in_vec_5, CellType::Penta);
        assert_eq!([131], res_5);
    }
    #[test]
    #[should_panic(expected = "vec size is wrong")]
    fn encode_bytes_to_page_should_panic_when_wrong_vec_size() {
        let target: ByteEncoderImpl = ByteEncoderImpl::new();

        let in_vec_2 = vec![true; 11];
        let res_2: [u8; 3] = target.encode_bytes_to_page(in_vec_2, CellType::Triple);
    }

    #[test]
    fn decode_page_to_bytes_should_return_right_section_for_zero() {
        let target: ByteEncoderImpl = ByteEncoderImpl::new();

        let in_arr = [0; 1];
        let res_1 = target.decode_page_to_bytes(in_arr, CellType::Single);
        assert_eq!(vec![false], res_1);

        let res_2 = target.decode_page_to_bytes(in_arr, CellType::Double);
        assert_eq!(vec![false; 2], res_2);

        let res_3 = target.decode_page_to_bytes(in_arr, CellType::Triple);
        assert_eq!(vec![false; 3], res_3);

        let res_4 = target.decode_page_to_bytes(in_arr, CellType::Quadro);
        assert_eq!(vec![false; 4], res_4);

        let res_5 = target.decode_page_to_bytes(in_arr, CellType::Penta);
        assert_eq!(vec![false; 5], res_5);
    }

    #[test]
    fn decode_page_to_bytes_should_return_right_section_for_max() {
        let target: ByteEncoderImpl = ByteEncoderImpl::new();

        let in_arr = [254; 1];
        let res_1 = target.decode_page_to_bytes(in_arr, CellType::Single);
        assert_eq!(vec![true], res_1);

        let res_2 = target.decode_page_to_bytes(in_arr, CellType::Double);
        assert_eq!(vec![true; 2], res_2);

        let res_3 = target.decode_page_to_bytes(in_arr, CellType::Triple);
        assert_eq!(vec![true; 3], res_3);

        let res_4 = target.decode_page_to_bytes(in_arr, CellType::Quadro);
        assert_eq!(vec![true; 4], res_4);

        let res_5 = target.decode_page_to_bytes(in_arr, CellType::Penta);
        assert_eq!(vec![true; 5], res_5);
    }

    #[test]
    fn encoding_and_decoding_should_work_consistent() {
        let target: ByteEncoderImpl = ByteEncoderImpl::new();

        let bits = vec![
            false, false, true, false, true, false, false, true, true, true, false, false, true,
            false, true,
        ];

        let pages_5: [u8; 5] = target.encode_bytes_to_page(bits.clone(), CellType::Triple);
        let res_5 = target.decode_page_to_bytes(pages_5, CellType::Triple);
        assert_eq!(bits, res_5);

        let pages_3: [u8; 3] = target.encode_bytes_to_page(bits.clone(), CellType::Penta);
        let res_3 = target.decode_page_to_bytes(pages_3, CellType::Penta);
        assert_eq!(bits, res_3);
    }
}

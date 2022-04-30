use crate::util::s_array;

pub struct Array1<T> {
    array: s_array::SArray<T>,
}

impl<T: Copy> Array1<T> {
    pub fn new(size: u32) -> Array1<T> {
        Array1 {
            array: s_array::SArray::new(1, vec![size]),
        }
    }

    pub fn len(&self) -> u32 {
        self.array.get_len()
    }

    pub fn init(&mut self, value: T) {
        self.array.init_array(value);
    }

    pub fn get(&self, index: u32) -> T {
        self.array.get(vec![index])
    }

    pub fn set(&mut self, index: u32, value: T) {
        self.array.set(vec![index], value);
    }
}

pub struct Array2<T> {
    array: s_array::SArray<T>,
}

impl<T: Copy> Array2<T> {
    pub fn new(row: u32, column: u32) -> Array2<T> {
        Array2 {
            array: s_array::SArray::new(2, vec![row, column]),
        }
    }

    pub fn len(&self) -> u32 {
        self.array.get_len()
    }

    pub fn size(&self) -> [u32; 2] {
        let size = self.array.get_size();
        [size[0], size[1]]
    }

    pub fn init(&mut self, value: T) {
        self.array.init_array(value);
    }

    pub fn get(&self, row: u32, column: u32) -> T {
        self.array.get(vec![row, column])
    }

    pub fn set(&mut self, row: u32, column: u32, value: T) {
        self.array.set(vec![row, column], value);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut arr_1 = Array1::<u8>::new(10);
        arr_1.init(1);
        arr_1.set(8, 2);
        assert_eq!(arr_1.get(8), 2);
        assert_eq!(arr_1.get(0), 1);
        assert_eq!(arr_1.get(9), 1);
        assert_eq!(arr_1.len(), 10);

        let mut arr_2 = Array2::<u8>::new(10000, 10000);
        arr_2.init(100);
        arr_2.set(8000, 6752, 67);
        assert_eq!(arr_2.get(8000, 6752), 67);
        assert_eq!(arr_2.get(0, 0), 100);
        assert_eq!(arr_2.get(9999, 9999), 100);
        assert_eq!(arr_2.len(), 10000 * 10000);
        assert_eq!(arr_2.size(), [10000, 10000]);
    }
}
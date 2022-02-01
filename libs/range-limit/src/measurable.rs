
pub trait Measurable {
    fn size(&self) -> usize;
}

impl Measurable for String {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<T, const L: usize> Measurable for [T; L] {
    fn size(&self) -> usize {
        L
    }
}

impl<T> Measurable for Vec<T> {
    fn size(&self) -> usize {
        self.len()
    }
}


impl<'s> Measurable for &'s str {
    fn size(&self) -> usize {
        self.len()
    }
}


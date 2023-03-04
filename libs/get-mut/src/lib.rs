pub trait GetMut<T> {
    fn get_mut(&mut self)->&mut T;

    fn mut_by(&mut self, handle:impl FnOnce(&mut T)){
        handle(<Self as GetMut<T>>::get_mut(self))
    }
}
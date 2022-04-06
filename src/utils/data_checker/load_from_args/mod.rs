pub trait LoadFromArgs<'s, Arg: Sized> {
    fn load(&'s self) -> Arg;
}

impl<'t, T> LoadFromArgs<'t, &'t T> for T {
    fn load(&'t self) -> &'t T { self }
}

impl<T> LoadFromArgs<'_, ()> for T {
    fn load(&'_ self) {  }
}

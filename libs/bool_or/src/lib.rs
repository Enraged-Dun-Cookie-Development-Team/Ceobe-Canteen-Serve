pub trait FalseOr {
    fn false_or(self, f: impl FnOnce() -> ());
}

impl FalseOr for bool {
    fn false_or(self, f: impl FnOnce() -> ()) {
        if self == true {f()}
    }
}

pub trait TrueOr {
    fn true_or(self, f: impl FnOnce() -> ());
}

impl TrueOr for bool {
    fn true_or(self, f: impl FnOnce() -> ()) {
        if self == false {f()}
    }
}

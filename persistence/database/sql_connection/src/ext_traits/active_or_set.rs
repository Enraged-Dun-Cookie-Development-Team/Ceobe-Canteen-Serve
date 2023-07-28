use sea_orm::{ActiveValue, Value};

pub trait ActiveOrSet<V> {
    fn set_optional(&mut self, value: Option<V>);
}

impl<V: Into<Value>> ActiveOrSet<V> for ActiveValue<V> {
    fn set_optional(&mut self, value: Option<V>) {
        let Some(v) = value
        else {
            return;
        };

        *self = sea_orm::Set(v);
    }
}

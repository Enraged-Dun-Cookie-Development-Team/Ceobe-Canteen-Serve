use paste::paste;
use serde::{Deserialize, Serialize};

use super::{FetchViewValue, FetchOptionViewValue, OptionViewField, _private::SealTrait};

macro_rules! const_field_def {
    ($t:ty => $id:ident) => {
        paste!{
            #[derive(Debug, Clone, Default)]
            pub struct [<Const $id Field>]<const V:$t>;

            impl<const V:$t> OptionViewField<$t> for [<Const $id Field>]<V> {
                fn skip_serde(&self) -> bool {
                    false
                }
            }

            impl<const V:$t> SealTrait for [<Const $id Field>]<V> {}

            impl<const V: $t> FetchViewValue<$t> for [<Const $id Field>]<V>  {
                fn fetch(self)->$t {
                    V
                }
            }
            
            impl<const T: $t> FetchOptionViewValue<$t> for [<Const $id Field>]<T> {
                fn fetch_option(self) -> Option<$t> { Some(T) }
            }

            impl<'de, const V:$t> Deserialize<'de> for [<Const $id Field>]<V> {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                D: serde::Deserializer<'de> {
                    let _ = <Option<()> as Deserialize<'de>>::deserialize(deserializer)?;
                    Ok(Self)
                }
            }

            impl<const V:$t> Serialize for [<Const $id Field>]<V> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                S: serde::Serializer {
                    <$t as Serialize>::serialize(&V, serializer)
                }
            }
        }
    };
    [$($t:ty => $id:ident)*]=>{
        $(
            const_field_def!($t => $id);
        )*
    }
}

const_field_def![
    bool => Bool
    i8 => I8
    i16 => I16
    i32 => I32
    i64 => I64
    i128 => I128
    isize => ISize
    u8 => U8
    u16 => U16
    u32 => U32
    u64 => U64
    u128 => U128
    usize => USize
    char => Char
];

use sea_query::Iden;


pub struct UuidToBin;

pub struct BinToUuid;

impl Iden for UuidToBin {
    fn unquoted(&self,s: &mut dyn std::fmt::Write) {
        write!(s, "UUID_TO_BIN").unwrap();
    }
}

impl Iden for BinToUuid {
    fn unquoted(&self,s: &mut dyn std::fmt::Write) {
        write!(s, "BIN_TO_UUID").unwrap();
    }
}
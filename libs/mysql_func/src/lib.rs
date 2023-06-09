use sea_query::Iden;

// MariaDb 不可用
pub struct UuidToBin;

pub struct BinToUuid;

impl Iden for UuidToBin {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "UUID_TO_BIN").unwrap();
    }
}

impl Iden for BinToUuid {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "BIN_TO_UUID").unwrap();
    }
}

// 用于AS后面的UUID
pub struct UUID;
impl Iden for UUID {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "UUID").unwrap();
    }
}

pub struct MATCH;
impl Iden for MATCH {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "MATCH").unwrap();
    }
}

pub struct AGAINST;
impl Iden for AGAINST {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "AGAINST").unwrap();
    }
}

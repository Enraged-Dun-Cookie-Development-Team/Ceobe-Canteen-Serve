pub trait FUserConfig {
    fn username(&self) -> String;
    fn password(&self) -> String;
}

use migration::Migrator;

#[tokio::main]
async fn main() {
    use sea_orm_migration::cli::run_cli;
    run_cli(Migrator).await
}

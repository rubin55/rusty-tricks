use sea_orm::entity::prelude::*;
use sea_orm::sea_query::TableCreateStatement;
use sea_orm::{entity::*, Database, DbConn, DeriveEntityModel, Statement};
use sea_orm::{DbBackend, DeriveRelation, EnumIter, Schema};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "data")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub iteration: u32,
    pub data: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    // Connecting SQLite
    let db = Database::connect("sqlite://test.sqlite").await?;

    // Setup database schema
    setup_schema(&db).await;

    // Performing tests
    testcase(&db).await;

    Ok(())
}

async fn setup_schema(db: &DbConn) {
    // Setup schema helper
    let schema = Schema::new(DbBackend::Sqlite);

    let disable_journal_mode = Statement::from_string(DbBackend::Sqlite, r"PRAGMA journal_mode = OFF");
    db.query_one(disable_journal_mode).await.expect("could not disable journal mode");

    let set_synchronous_to_zero = Statement::from_string(DbBackend::Sqlite, r"PRAGMA synchronous = 0");
    db.query_one(set_synchronous_to_zero).await.expect("could not set synchronous to zero");

    let set_cache_size_to_million = Statement::from_string(DbBackend::Sqlite, r"PRAGMA cache_size = 1000000");
    db.query_one(set_cache_size_to_million).await.expect("could not set cache size to one million");

    let set_locking_mode_exclusive = Statement::from_string(DbBackend::Sqlite, r"PRAGMA locking_mode = EXCLUSIVE");
    db.query_one(set_locking_mode_exclusive).await.expect("could not set locking mode to exclusive");

    let set_temp_store_to_memory = Statement::from_string(DbBackend::Sqlite, r"PRAGMA temp_store = MEMORY");
    db.query_one(set_temp_store_to_memory).await.expect("could not set temp store to memory");

    // Derive from entity
    let create_table: TableCreateStatement = schema.create_table_from_entity(Entity);

    // Execute create table statement
    let _result = db
        .execute(db.get_database_backend().build(&create_table))
        .await;
}

async fn testcase(db: &DbConn)  {
    for iteration in 1..=1000000 {
        // Tell us about it
        println!("Running iteration: {}", iteration);

        // Create a new row
        let model = ActiveModel {
            id: Set(Uuid::now_v7()),
            iteration: Set(iteration),
            data: Set("some-data".parse().unwrap()),
        };

        // Execute insert statement
        let _result = Entity::insert(model)
            .exec(db)
            .await
            .expect("could not insert model");
    }
}
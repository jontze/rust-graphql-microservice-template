use chrono::NaiveDateTime;
use juniper::GraphQLObject;

#[derive(GraphQLObject, sqlx::FromRow)]
#[graphql(description = "A single Database Row")]
pub struct ExampleStruct {
    pub id: i32,
    pub name: String,
    pub updated_at: NaiveDateTime,
}

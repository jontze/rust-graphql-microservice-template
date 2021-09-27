use juniper::{graphql_object, FieldResult};
use sqlx::Row;

use super::context::GraphContext;
use super::model::ExampleStruct;

pub struct QueryRoot;

#[graphql_object(Context = GraphContext)]
impl QueryRoot {
    #[graphql(description = "List of all examples")]
    async fn examples(ctx: &GraphContext) -> FieldResult<Vec<ExampleStruct>> {
        let mut examples: Vec<ExampleStruct> = vec![];
        let rows = sqlx::query("SELECT * FROM examples")
            .fetch_all(&ctx.pool)
            .await?;
        for row in rows {
            examples.push(ExampleStruct {
                id: row.get(0),
                name: row.get(1),
                updated_at: row.get(2),
            })
        }
        Ok(examples)
    }

    #[graphql(description = "Get a ExampleEntity by Id")]
    async fn example(id: i32, ctx: &GraphContext) -> FieldResult<ExampleStruct> {
        let row = sqlx::query("SELECT * FROM examples WHERE id = $1")
            .bind(id)
            .fetch_one(&ctx.pool)
            .await?;
        Ok(ExampleStruct {
            id: row.get(0),
            name: row.get(1),
            updated_at: row.get(2),
        })
    }
}

pub struct MutationRoot;

#[graphql_object(Context = GraphContext)]
impl MutationRoot {
    #[graphql(description = "Create a ExampleEntity")]
    async fn createExample(name: String, ctx: &GraphContext) -> FieldResult<ExampleStruct> {
        let row = sqlx::query("INSERT INTO examples (name) VALUES ($1) RETURNING *")
            .bind(name)
            .fetch_one(&ctx.pool)
            .await?;
        Ok(ExampleStruct {
            id: row.get(0),
            name: row.get(1),
            updated_at: row.get(2),
        })
    }
}

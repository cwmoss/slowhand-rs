/*
use std::error::Error;

use async_graphql::{dynamic::*, http::GraphiQLSource};


#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let query = Object::new("Query").field(Field::new(
        "howdy",
        TypeRef::named_nn(TypeRef::STRING),
        |_| FieldFuture::new(async { "partner" }),
    ));

    // create the schema
    let schema = Schema::build(query, None, None).register(query).finish()?;

    // start the http server
    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
    println!("GraphiQL: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;
    Ok(())
}

pub fn build_schema() {}

// pub fn resolve() {}

pub fn resolve(ctx: ResolverContext<'_>) -> FieldFuture<'_> {
    FieldFuture::new(async move {
        dbg!("getBookField via resolve", ctx.parent_value, ctx.path_node);
        let book = ctx.parent_value.try_downcast_ref::<Doc>()?;
        Ok(Some(Value::from(book.d["name"].to_owned())))
    })
}

    */

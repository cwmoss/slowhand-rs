use super::AppState;
use crate::dataset::{self, Dataset};
use std::collections::HashMap;
use std::sync::Arc;

// use async_graphql::{EmptyMutation, EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;

use axum::{
    body::{Body, Bytes},
    extract::{DefaultBodyLimit, FromRequest, Multipart, Path, Query, Request, State},
    http::{header, HeaderValue, StatusCode},
    middleware,
    response::{Html, IntoResponse, Json, Redirect, Response},
    routing::{get, post},
    serve::Listener,
    Router,
};
// use starwars::{QueryRoot, StarWars};

use async_graphql::{dynamic::*, Value, ID};
// use futures_util::StreamExt;

#[derive(Clone)]
pub struct Doc {
    _id: ID,
    _type: String,
    d: HashMap<String, Value>,
}

pub async fn get_routes(state: AppState) -> Router<Arc<AppState>> {
    let r = Router::new().route(
        "/{dsname}",
        get(graphiql).post_service(get_graphql_service(&state, "egal".to_string()).await),
    );
    r
}

async fn graphiql(Path(dsname): Path<String>) -> impl IntoResponse {
    let endpoint = format!("/graphql/{}", dsname);
    Html(
        async_graphql::http::GraphiQLSource::build()
            .endpoint(&endpoint)
            .finish(),
    )
}
// Path(dsname): Path<String>
async fn get_graphql_service(
    app_state: &AppState,
    dsname: String,
) -> GraphQL<async_graphql::dynamic::Schema> {
    dbg!("service dsname", &dsname);
    let ds = dataset::Dataset::load(dsname, &app_state.conf.projects, &app_state.conf.var).await;

    let mut book = Object::new("Book")
        .description("A book that will be stored.")
        .field(Field::new("_id", TypeRef::named_nn(TypeRef::ID), |ctx| {
            FieldFuture::new(async move {
                let book = ctx.parent_value.try_downcast_ref::<Doc>()?;
                Ok(Some(Value::from(book._id.to_owned())))
            })
        }));
    book = book
        .field(Field::new(
            "name",
            TypeRef::named_nn(TypeRef::STRING),
            resolve,
        ))
        .field(Field::new(
            "author",
            TypeRef::named_nn(TypeRef::STRING),
            |ctx| {
                FieldFuture::new(async move {
                    let book = ctx.parent_value.try_downcast_ref::<Doc>()?;
                    Ok(Some(Value::from(book.d["author"].to_owned())))
                })
            },
        ))
        .key("_id");

    let mut query_root = Object::new("Query").field(Field::new(
        "getBooks",
        TypeRef::named_list(book.type_name()),
        |ctx| {
            FieldFuture::new(async move {
                //let store = ctx.data_unchecked::<Storage>().lock().await;
                //let books: Vec<Book> = store.iter().map(|(_, book)| book.clone()).collect();
                let books = [Doc {
                    _id: ID::from("b01"),
                    _type: "book".to_string(),
                    d: HashMap::from([
                        ("name".to_string(), Value::from("buch 1")),
                        ("author".to_string(), Value::from("bimmel")),
                    ]),
                }];
                Ok(Some(FieldValue::list(
                    books.into_iter().map(FieldValue::owned_any),
                )))
            })
        },
    ));
    query_root = query_root.field(
        Field::new("getBook", TypeRef::named(book.type_name()), |ctx| {
            FieldFuture::new(async move {
                let id = ctx.args.try_get("_id")?;
                let book_id = match id.string() {
                    Ok(id) => id.to_string(),
                    Err(_) => id.u64()?.to_string(),
                };
                let book = Some(Doc {
                    _id: ID::from("b01"),
                    _type: "book".to_string(),
                    d: HashMap::from([
                        ("name".to_string(), Value::from("buch 1")),
                        ("author".to_string(), Value::from("P. Bimmel")),
                    ]),
                });
                dbg!("getBook", ctx.parent_value, ctx.path_node);
                // let book_id = book_id.parse::<usize>()?;
                // let store = ctx.data_unchecked::<Storage>().lock().await;
                // let book = store.get(book_id).cloned();
                Ok(book.map(FieldValue::owned_any))
            })
        })
        .argument(InputValue::new("_id", TypeRef::named_nn(TypeRef::ID))),
    );

    let q = "Query";
    let mut schemab = Schema::build(query_root.type_name(), None, None)
        //.register(mutation_type)
        ;
    schemab = schemab
        .register(book)
        // .register(book_changed)
        .register(query_root)
       // .entity_resolver(resolve_entity)
        //.register(subscription_root)
        ;

    //.register(mutatation_root)
    //.data(Storage::default())
    let schema = schemab.finish().unwrap();
    GraphQL::new(schema)
}

pub fn resolve(ctx: ResolverContext<'_>) -> FieldFuture<'_> {
    FieldFuture::new(async move {
        dbg!(
            "getBookField via resolve",
            ctx.parent_value,
            ctx.path_node,
            ctx.field()
        );
        let book = ctx.parent_value.try_downcast_ref::<Doc>()?;
        Ok(Some(Value::from(book.d["name"].to_owned())))
    })
}

pub fn resolve_entity(ctx: ResolverContext<'_>) -> FieldFuture<'_> {
    FieldFuture::new(async move {
        dbg!(
            "entity resolver",
            ctx.parent_value,
            ctx.path_node,
            ctx.field()
        );
        let book = ctx.parent_value.try_downcast_ref::<Doc>()?;
        Ok(Some(Value::from(book.d["name"].to_owned())))
    })
}

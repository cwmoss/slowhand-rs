use crate::schema::{self, BaseType, FieldType, Preview};
use crate::store;
use async_graphql::{dynamic::*, Value, ID};
use async_graphql::{value, Name};
use std::any::Any;
use std::collections::HashMap;
use std::f32::INFINITY;

impl schema::Schema {
    pub fn build_gql_schema(&self) -> SchemaBuilder {
        let mut schemab = Schema::build("Query", None, None)
        //.register(mutation_type)
        ;

        for gobj in self.get_default_interfaces() {
            schemab = schemab.register(gobj);
        }
        for gobj in self.get_default_enums() {
            schemab = schemab.register(gobj);
        }
        for gobj in self.get_default_types() {
            schemab = schemab.register(gobj);
        }
        for gobj in self.get_default_inputs() {
            schemab = schemab.register(gobj);
        }
        let mut query_root = Object::new("Query");

        for (_, obj) in &self.objects {
            let gobj = obj.to_graphql();
            let gtype = gobj.type_name().to_owned();
            schemab = schemab.register(gobj);
            match obj.base_type {
                BaseType::Doc => {
                    query_root = query_root
                        .field(obj.to_graphql_entity_query(&gtype))
                        .field(obj.to_graphql_list_query(&gtype));
                }
                _ => (),
            }
        }

        schemab = schemab.register(query_root);
        /*for doc in self.documents() {
            query_root = query_root
                .field(doc.to_graphql_entity_query())
                .field(doc.to_graphql_list_query());
        }*/
        schemab
    }

    fn get_default_types(&self) -> Vec<Object> {
        vec![
            Object::new("slug")
                .description("A slug")
                .interface_object()
                .field(Field::new(
                    "current",
                    TypeRef::named(TypeRef::STRING),
                    resolve_object_field,
                )),
            Object::new("ref")
                .description("reference another document")
                .field(Field::new(
                    "_rev",
                    TypeRef::named(TypeRef::ID),
                    resolve_field,
                )),
            Object::new("hotspot")
                .description("reference another document")
                .field(Field::new(
                    "x",
                    TypeRef::named(TypeRef::FLOAT),
                    resolve_field,
                ))
                .field(Field::new(
                    "y",
                    TypeRef::named(TypeRef::FLOAT),
                    resolve_field,
                ))
                .field(Field::new(
                    "width",
                    TypeRef::named(TypeRef::FLOAT),
                    resolve_field,
                ))
                .field(Field::new(
                    "height",
                    TypeRef::named(TypeRef::FLOAT),
                    resolve_field,
                )),
        ]
    }

    fn get_default_interfaces(&self) -> Vec<Interface> {
        vec![Interface::new("Document")
            .field(InterfaceField::new("_id", TypeRef::named_nn(TypeRef::ID)))
            .field(InterfaceField::new(
                "_type",
                TypeRef::named_nn(TypeRef::STRING),
            ))
            .field(InterfaceField::new(
                "_rev",
                TypeRef::named_nn(TypeRef::STRING),
            ))]
    }

    fn get_default_enums(&self) -> Vec<Enum> {
        vec![
            Enum::new("SortOperator")
                .item(EnumItem::new("ASC"))
                .item(EnumItem::new("DESC")),
            Enum::new("FilterOperator")
                .item(EnumItem::new("eq"))
                .item(EnumItem::new("ne")),
        ]
    }

    /*
            input FilterCondition{
        eq: String
        ne: String
        in: [String]
        nin: [String]
        like: String
        ilike: String
    } */
    fn get_default_inputs(&self) -> Vec<InputObject> {
        vec![InputObject::new("FilterCondition")
            .field(InputValue::new("eq", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("ne", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("like", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("ilike", TypeRef::named(TypeRef::STRING)))
            .field(InputValue::new("in", TypeRef::named_list(TypeRef::STRING)))
            .field(InputValue::new("nin", TypeRef::named_list(TypeRef::STRING)))]
    }
}

/*
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
*/
impl schema::Object {
    pub fn to_graphql(&self) -> Object {
        let mut gobj = Object::new(&self.name).description(&self.description);
        for f in &self.fields {
            gobj = gobj.field(f.to_graphql());
        }
        match self.base_type {
            schema::BaseType::Doc => {
                gobj = gobj
                    .implement("Document")
                    .field(Field::new(
                        "_id",
                        TypeRef::named_nn(TypeRef::ID),
                        resolve_field,
                    ))
                    .field(Field::new(
                        "_type",
                        TypeRef::named_nn(TypeRef::STRING),
                        resolve_field,
                    ))
                    .field(Field::new(
                        "_rev",
                        TypeRef::named_nn(TypeRef::STRING),
                        resolve_field,
                    ))
                    //.field(Field::new("_id", TypeRef::named_nn(TypeRef::ID), resolve))
                    //.field(Field::new("_id", TypeRef::named_nn(TypeRef::ID), resolve))
                    .key("_id");
            }
            _ => (),
        };
        gobj
    }

    pub fn to_graphql_entity_query(&self, gql_type: &str) -> Field {
        Field::new(&self.name, TypeRef::named(gql_type), resolve_entity)
            .argument(InputValue::new("id", TypeRef::named_nn(TypeRef::ID)))
    }

    pub fn to_graphql_list_query(&self, gql_type: &str) -> Field {
        Field::new(
            format!("all{}", &self.name),
            TypeRef::named(gql_type),
            resolve_entity_list,
        )
    }
}

impl schema::Field {
    pub fn to_graphql(&self) -> Field {
        let resolve = match self.field_type {
            FieldType::Object(_) => resolve_object,
            FieldType::Slug => resolve_object,
            _ => resolve_field,
        };
        let f = Field::new(&self.name, self.field_type.to_graphql(), resolve);
        f
    }
}

impl schema::FieldType {
    pub fn to_graphql(&self) -> TypeRef {
        match self {
            Self::Boolean => TypeRef::named(TypeRef::BOOLEAN),
            Self::Slug => TypeRef::named("slug"),
            // Self::Array => TypeRef::named_list(""),
            Self::Number => TypeRef::named(TypeRef::INT),
            Self::Object(name) => {
                dbg!("custom object", name);
                TypeRef::named(name.to_string())
            }
            _ => TypeRef::named(TypeRef::STRING),
        }
    }
}
#[derive(Clone)]
pub struct Docx {
    _id: ID,
    _type: String,
    d: HashMap<String, Value>,
}

pub fn resolve_field(ctx: ResolverContext<'_>) -> FieldFuture<'_> {
    FieldFuture::new(async move {
        let name = ctx.field().name();
        dbg!(
            "field resolver",
            // &ctx.type_id(),
            ctx.parent_value,
            ctx.path_node,
            ctx.field(),
            ctx.parent_value.as_value(),
            name
        );
        // let doc: &store::Doc = ctx.parent_value.try_downcast_ref::<store::Doc>()?;
        let doc: &Value = ctx.parent_value.try_downcast_ref::<Value>()?;
        let val = match doc {
            Value::Object(o) => o.get(name).unwrap().to_string(),
            _ => "missing".to_string(),
        };
        /*
        dbg!(&doc.d[name]);
        if doc.d[name].is_object() {
            // Value::from(doc.d[name].as_object().unwrap())
            return Ok(Some(FieldValue::borrowed_any(
                doc.d[name].as_object().unwrap(),
            )));
        }

        // FieldValue::borrowed_any(doc.d[name]);
        // FieldValue::borrowed_any(doc.d[name].as_object().unwrap());
        let val = match name {
            "_id" => Value::from(doc._id.to_string()),
            "_type" => Value::from(doc._type.to_string()),
            _ => Value::from(doc.d[name].to_string()),
        };
        */
        Ok(Some(Value::from(val.as_str())))
        // Ok(Some(FieldValue::borrowed_any(&val)))
    })
}

pub fn resolve_object_field(ctx: ResolverContext<'_>) -> FieldFuture<'_> {
    FieldFuture::new(async move {
        let name = ctx.field().name();
        dbg!(
            "object field resolver",
            // &ctx.type_id(),
            ctx.parent_value,
            ctx.path_node,
            ctx.field(),
            ctx.parent_value.as_value(),
            "O",
            name
        );
        //let doc: &serde_json::Value = ctx.parent_value.try_downcast_ref::<serde_json::Value>()?;
        let doc = ctx.parent_value.try_downcast_ref::<Value>()?;
        dbg!("obj-doc", doc);
        let val = "derslug";
        Ok(Some(Value::from(val.to_string())))
    })
}

pub fn resolve_object(ctx: ResolverContext<'_>) -> FieldFuture<'_> {
    FieldFuture::new(async move {
        let name = ctx.field().name();
        dbg!(
            "object resolver",
            ctx.parent_value,
            ctx.path_node,
            ctx.field(),
            name
        );
        let doc: &Value = ctx.parent_value.try_downcast_ref::<Value>()?;
        //let obj:  = ctx
        //    .parent_value
        //     .try_downcast_ref::<HashMap<String, Value>>();
        // Ok(Some(Value::from(doc[name].as_object()))) //   get(name))))
        Ok(Some(Value::from(doc.to_string())))
    })
}

pub fn resolve_entity(ctx: ResolverContext<'_>) -> FieldFuture<'_> {
    FieldFuture::new(async move {
        let etype = ctx.field().name();
        dbg!(
            "entity resolver",
            ctx.parent_value,
            ctx.path_node,
            ctx.field(),
            etype
        );
        let test = r#"{"_createdAt":"2018-06-13T08:57:45Z","_id":"movie_10681","_rev":"KpnZHIwJumnkYWqCgtzWPy","_type":"movie","_updatedAt":"2022-07-25T14:50:57Z","castMembers":[{"_key":"3d1bfb748327aefa5a65203b99f59a04","_type":"castMember","characterName":"WALL·E / M-O (voice)","externalCreditId":"52fe43a29251416c75018111","externalId":670,"person":{"_ref":"person_ben-burtt","_type":"reference"}},{"_key":"41828c36150640fc29e7cdd9dfc13f12","_type":"castMember","characterName":"EVE (voice)","externalCreditId":"52fe43a29251416c75018115","externalId":72754,"person":{"_ref":"person_elissa-knight","_type":"reference"}},{"_key":"1b9bdcda57c5918af15c8a6ad319cfbd","_type":"castMember","characterName":"Captain (voice)","externalCreditId":"52fe43a29251416c75018119","externalId":60074,"person":{"_ref":"person_jeff-garlin","_type":"reference"}},{"_key":"f8c989adf3feb59e416e1d7e96df6670","_type":"castMember","characterName":"Shelby Forthright, BnL CEO","externalCreditId":"52fe43a29251416c7501811d","externalId":20753,"person":{"_ref":"person_fred-willard","_type":"reference"}},{"_key":"4814c30c05316f3550645053a0f62847","_type":"castMember","characterName":"AUTO (voice)","externalCreditId":"561be27a9251415a6e0017f1","externalId":72755,"person":{"_ref":"person_macintalk","_type":"reference"}},{"_key":"542b94e0004bf96b9c8b7b5e521ccc1e","_type":"castMember","characterName":"John (voice)","externalCreditId":"53c61bc6c3a3686251001af8","externalId":7907,"person":{"_ref":"person_john-ratzenberger","_type":"reference"}},{"_key":"0a48d7ee91efc5eec83ce141448b44a1","_type":"castMember","characterName":"Mary (voice)","externalCreditId":"5587d7c49251415aa900050f","externalId":11074,"person":{"_ref":"person_kathy-najimy","_type":"reference"}},{"_key":"98f90a4ada3591bd381fe99f31bd6ad1","_type":"castMember","characterName":"Ship's Computer (voice)","externalCreditId":"59bf4a4bc3a368307500cb35","externalId":10205,"person":{"_ref":"person_sigourney-weaver","_type":"reference"}},{"_key":"6f8a9f91c0577413e83a6021718fed0c","_type":"castMember","characterName":"Steward Bots (voice)","externalCreditId":"561be2ad9251415a67001bd1","externalId":59357,"person":{"_ref":"person_teddy-newton","_type":"reference"}},{"_key":"0730eb131b932ab3f128e10f8cedaf37","_type":"castMember","characterName":"Axiom Passenger (voice)","externalCreditId":"561be2c59251415a640016d0","externalId":78317,"person":{"_ref":"person_bob-bergen","_type":"reference"}}],"crewMembers":[{"_key":"c1fa227f38952ce26d9a66cbe1813672","_type":"crewMember","department":"Directing","externalCreditId":"52fe43a29251416c750180d1","externalId":7,"job":"Director","person":{"_ref":"person_andrew-stanton","_type":"reference"}},{"_key":"c84554bf8e0db3fb0609da8d9c6ea2dc","_type":"crewMember","department":"Sound","externalCreditId":"52fe43a29251416c750180d7","externalId":153,"job":"Original Music Composer","person":{"_ref":"person_thomas-newman","_type":"reference"}},{"_key":"67d094506ca687f8fb5705b4e21983d5","_type":"crewMember","department":"Production","externalCreditId":"52fe43a29251416c750180dd","externalId":72752,"job":"Producer","person":{"_ref":"person_jim-morris","_type":"reference"}},{"_key":"850ebbddf83a571616a8540f4e4ba278","_type":"crewMember","department":"Production","externalCreditId":"52fe43a29251416c750180e3","externalId":72753,"job":"Producer","person":{"_ref":"person_lindsey-collins","_type":"reference"}},{"_key":"0f054d5a713dc2e490ff51631132e3ac","_type":"crewMember","department":"Production","externalCreditId":"52fe43a29251416c750180e9","externalId":7887,"job":"Executive Producer","person":{"_ref":"person_thomas-porter","_type":"reference"}}],"externalId":10681,"overview":[{"_key":"8a1fd7b434db11443bf33bc3a2428b64","_type":"block","children":[{"_key":"v6xmjPqs","_type":"span","marks":[],"text":"WALL·E is the last robot left on an Earth that has been overrun with garbage and all humans have fled to outer space. For 700 years he has continued to try and clean up the mess, but has developed some rather interesting human-like qualities. When a ship arrives with a sleek new type of robot, WALL·E thinks he's finally found a friend and stows away on the ship when it leaves."}],"markDefs":[],"style":"normal"}],"popularity":19.020593,"poster":{"_sanityAsset":"image@file://./images/69ad5d60ff19c456954513e8c67e9563c780d5e1-780x1170.jpg","_type":"image","crop":{"bottom":0,"left":0,"right":0,"top":0.3859065420560748},"hotspot":{"height":0.08710280373831777,"width":0.2149532710280373,"x":0.4750778816199377,"y":0.6141121495327106}},"releaseDate":"2008-06-22T00:00:00Z","slug":{"_type":"slug","current":"walle","source":"title"},"title":"WALL·E"}
"#;
        // let found: store::Doc = serde_json::from_str(test).ok().unwrap();
        let found: serde_json::Value = serde_json::from_str(test).ok().unwrap();
        let found = Value::from_json(found).unwrap();
        //let found = FieldValue::from_json(test);
        let res = Some(found);
        // let book = ctx.parent_value.try_downcast_ref::<Doc>()?;
        // Ok(Some(value!(found)))
        Ok(res.map(FieldValue::owned_any))
        //Ok(FieldValue::borrowed_any(&res))
    })
}

pub fn resolve_entity_list(ctx: ResolverContext<'_>) -> FieldFuture<'_> {
    FieldFuture::new(async move {
        dbg!(
            "entity LIST resolver",
            ctx.parent_value,
            ctx.path_node,
            ctx.field()
        );
        let book = ctx.parent_value.try_downcast_ref::<Docx>()?;
        Ok(Some(Value::from(book.d["name"].to_owned())))
    })
}

/*

function rename_gtype($type){
    $type = str_replace('.', '__', $type);
    return ucfirst($type);
}

function rename_gmutation($type, $meth){
    return $meth.rename_gtype($type);
}

function rename_gmutation_input($type, $meth){
    return rename_gtype(rename_gmutation($type, $meth).'Input');
}

*/

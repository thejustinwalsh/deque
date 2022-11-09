use std::collections::BTreeMap;

use crate::db::SurrealError;

use super::RouterBuilder;
use rspc::{ErrorCode, ExecError, Type};
use serde::{Deserialize, Serialize};
use surrealdb::{
    sql::{Object, Value},
    Error,
};

pub fn mount() -> RouterBuilder {
    RouterBuilder::new()
        .mutation("create", |t| {
            t(|db, name: String| async move {
                let mut vars = BTreeMap::new();
                vars.insert(String::from("name"), Value::from(name));

                let r = db
                    .datastore
                    .execute(
                        "CREATE collections 
                            SET name = $name,
                                created_at = time::now(),
                                updated_at = time::now() 
                         RETURN id;",
                        &db.session,
                        Some(vars),
                        false,
                    )
                    .await
                    .map_err(|err| -> rspc::Error {
                        ExecError::UnsupportedMethod(err.to_string()).into()
                    })
                    .map(|res| res.into_iter().next().unwrap().result.unwrap().to_string());
                r
            })
        })
        .query("read", |t| {
            t(|db, _id: String| async move {
                todo!("read collection");
            })
        })
        .mutation("update", |t| {
            #[derive(Type, Deserialize)]
            struct CollectionUpdateArgs {
                id: String,
                name: String,
            }

            t(|db, args: CollectionUpdateArgs| async move {
                todo!("update collection");
            })
        })
        .mutation("delete", |t| {
            t(|db, _id: String| async move {
                todo!("delete collection");
            })
        })
        .query("list", |t| {
            #[derive(Type, Serialize)]
            struct Collection {
                id: String,
                name: String,
            }

            #[derive(Type, Serialize)]
            struct Collections(pub Vec<Collection>);

            impl TryFrom<Object> for Collection {
                type Error = Error;
                fn try_from(mut val: Object) -> Result<Collection, Error> {
                    Ok(Collection {
                        id: val.remove("id").ok_or_else(|| Error::Ignore)?.as_string(),
                        name: val.remove("name").ok_or_else(|| Error::Ignore)?.as_string(),
                    })
                }
            }

            impl TryFrom<Value> for Collection {
                type Error = Error;

                fn try_from(value: Value) -> Result<Self, Self::Error> {
                    match value {
                        Value::Object(obj) => Ok(Collection::try_from(obj)?),
                        _ => Err(Error::Ignore),
                    }
                }
            }

            impl TryFrom<Value> for Collections {
                type Error = Error;

                fn try_from(value: Value) -> Result<Self, Self::Error> {
                    match value {
                        Value::Array(arr) => arr
                            .0
                            .into_iter()
                            .map(|v| Collection::try_from(v))
                            .collect::<Result<Vec<Collection>, Error>>()
                            .map(|v| Collections(v)),
                        _ => Err(Error::Ignore),
                    }
                }
            }

            t(|db, _: ()| async move {
                db.datastore
                    .execute("SELECT * FROM collections;", &db.session, None, false)
                    .await
                    .map(|res| {
                        Collections::try_from(res.first().unwrap().output().unwrap().to_owned())
                            .map_err(|err| {
                                rspc::Error::with_cause(
                                    ErrorCode::InternalServerError,
                                    err.to_string().into(),
                                    err,
                                )
                            })
                    })
                    .unwrap()
            })
        })
}

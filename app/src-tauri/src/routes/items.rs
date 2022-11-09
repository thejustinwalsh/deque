use super::RouterBuilder;
use rspc::{ExecError, Type};
use serde::Deserialize;
use std::collections::BTreeMap;
use surrealdb::sql::Value;

pub fn mount() -> RouterBuilder {
    RouterBuilder::new()
        .mutation("create", |t| {
            #[derive(Type, Deserialize)]
            struct ItemCreateArgs {
                title: String,
                collection_id: String,
            }
            t(|db, args: ItemCreateArgs| async move {
                let mut vars = BTreeMap::new();
                vars.insert(String::from("title"), Value::from(args.title));
                vars.insert(
                    String::from("collection_id"),
                    Value::from(args.collection_id),
                );

                db.datastore
                    .execute("CREATE item", &db.session, Some(vars), true)
                    .await
                    .map_err(|err| -> rspc::Error {
                        ExecError::UnsupportedMethod(err.to_string()).into()
                    })
                    .map(|_| ())
            })
        })
        .query("read", |t| {
            t(|db, id: String| async move {
                todo!("read item");
            })
        })
        .mutation("update", |t| {
            #[derive(Type, Deserialize)]
            struct ItemUpdateArgs {
                id: String,
                title: String,
                completed: bool,
            }

            t(|db, args: ItemUpdateArgs| async move {
                todo!("update item");
            })
        })
        .mutation("delete", |t| {
            t(|db, id: String| async move {
                todo!("delete item");
            })
        })
        .query("list", |t| {
            t(|db, collection_id: String| async move { vec!["test"] })
        })
}

use super::db::collection;
use super::RouterBuilder;
use rspc::Type;
use serde::Deserialize;

pub fn mount() -> RouterBuilder {
    RouterBuilder::new()
        .mutation("create", |t| {
            t(|db, name| async move {
                db.collection()
                    .create(name, vec![])
                    .exec()
                    .await
                    .map_err(Into::into)
            })
        })
        .query("read", |t| {
            t(|db, id| async move {
                db.collection()
                    .find_unique(collection::id::equals(id))
                    .exec()
                    .await
                    .map_err(Into::into)
            })
        })
        .mutation("update", |t| {
            #[derive(Type, Deserialize)]
            struct CollectionUpdateArgs {
                id: i32,
                name: String,
            }

            t(|db, args: CollectionUpdateArgs| async move {
                db.collection()
                    .update(
                        collection::id::equals(args.id),
                        vec![collection::name::set(args.name)],
                    )
                    .exec()
                    .await
                    .map_err(Into::into)
            })
        })
        .mutation("delete", |t| {
            t(|db, id| async move {
                db.collection()
                    .delete(collection::id::equals(id))
                    .exec()
                    .await
                    .map_err(Into::into)
            })
        })
        .query("list", |t| {
            t(|db, _: ()| async move {
                db.collection()
                    .find_many(vec![])
                    .exec()
                    .await
                    .map_err(Into::into)
            })
        })
}

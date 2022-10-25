use super::db::item;
use super::RouterBuilder;
use rspc::Type;
use serde::Deserialize;

pub fn mount() -> RouterBuilder {
    RouterBuilder::new()
        .mutation("create", |t| {
            #[derive(Type, Deserialize)]
            struct ItemCreateArgs {
                title: String,
                collection_id: i32,
            }
            t(|db, args: ItemCreateArgs| async move {
                db.item()
                    .create(
                        args.title,
                        vec![item::collection_id::set(Some(args.collection_id))],
                    )
                    .exec()
                    .await
                    .map_err(Into::into)
            })
        })
        .query("read", |t| {
            t(|db, id| async move {
                db.item()
                    .find_unique(item::id::equals(id))
                    .exec()
                    .await
                    .map_err(Into::into)
            })
        })
        .mutation("update", |t| {
            #[derive(Type, Deserialize)]
            struct ItemUpdateArgs {
                id: i32,
                title: String,
                completed: bool,
            }

            t(|db, args: ItemUpdateArgs| async move {
                db.item()
                    .update(
                        item::id::equals(args.id),
                        vec![
                            item::title::set(args.title),
                            item::completed::set(args.completed),
                        ],
                    )
                    .exec()
                    .await
                    .map_err(Into::into)
            })
        })
        .mutation("delete", |t| {
            t(|db, id| async move {
                db.item()
                    .delete(item::id::equals(id))
                    .exec()
                    .await
                    .map_err(Into::into)
            })
        })
        .query("list", |t| {
            t(|db, collection_id| async move {
                db.item()
                    .find_many(vec![item::collection_id::equals(collection_id)])
                    .exec()
                    .await
                    .map_err(Into::into)
            })
        })
}

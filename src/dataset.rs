use crate::schema::Schema;
use crate::store::Store;

struct Dataset {
    name: String,
    schema: Schema,
    store: Store,
}

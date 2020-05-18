#[derive(juniper::GraphQLObject)]
struct Obj {
    field: String,
}

struct Object {}

#[juniper::graphql_object]
impl Object {
    async fn test(&self, test: Obj) -> String {
        String::new()
    }
}

fn main() {}

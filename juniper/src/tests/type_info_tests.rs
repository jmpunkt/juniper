use indexmap::IndexMap;

use crate::{
    executor::{ExecutionResult, Executor, Registry, Variables},
    schema::{meta::MetaType, model::RootNode},
    types::{
        base::{Arguments, GraphQLType},
        scalars::{EmptyMutation, EmptySubscription},
    },
    value::{ScalarValue, Value},
    BoxFuture,
};

pub struct NodeTypeInfo {
    name: String,
    attribute_names: Vec<String>,
}

pub struct Node {
    attributes: IndexMap<String, String>,
}

impl<S> GraphQLType<S> for Node
where
    S: ScalarValue,
{
    type Context = ();
    type TypeInfo = NodeTypeInfo;

    fn name(info: &Self::TypeInfo) -> Option<&str> {
        Some(&info.name)
    }

    fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r, S>) -> MetaType<'r, S>
    where
        S: 'r,
    {
        let fields = info
            .attribute_names
            .iter()
            .map(|name| registry.field::<String>(name, &()))
            .collect::<Vec<_>>();

        registry
            .build_object_type::<Node>(info, &fields)
            .into_meta()
    }

    fn resolve_field<'me, 'ty, 'field, 'args, 'ref_err, 'err, 'fut>(
        &'me self,
        _info: &'ty Self::TypeInfo,
        field_name: &'field str,
        _arguments: &'args Arguments<'args, S>,
        executor: &'ref_err Executor<'ref_err, 'err, Self::Context, S>,
    ) -> BoxFuture<'fut, ExecutionResult<S>>
    where
        'me: 'fut,
        'ty: 'fut,
        'args: 'fut,
        'ref_err: 'fut,
        'err: 'fut,
        'field: 'fut,
        S: 'fut,
    {
        futures::future::FutureExt::boxed(
            executor.resolve(&(), self.attributes.get(field_name).unwrap()),
        )
    }
}

#[tokio::test]
async fn test_node() {
    let doc = r#"
        {
            foo,
            bar,
            baz
        }"#;
    let node_info = NodeTypeInfo {
        name: "MyNode".to_string(),
        attribute_names: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
    };
    let mut node = Node {
        attributes: IndexMap::new(),
    };
    node.attributes.insert("foo".to_string(), "1".to_string());
    node.attributes.insert("bar".to_string(), "2".to_string());
    node.attributes.insert("baz".to_string(), "3".to_string());
    let schema: RootNode<_, _, _> = RootNode::new_with_info(
        node,
        EmptyMutation::new(),
        EmptySubscription::new(),
        node_info,
        (),
        (),
    );

    assert_eq!(
        crate::execute(doc, None, &schema, &Variables::new(), &()).await,
        Ok((
            Value::object(
                vec![
                    ("foo", Value::scalar("1")),
                    ("bar", Value::scalar("2")),
                    ("baz", Value::scalar("3")),
                ]
                .into_iter()
                .collect()
            ),
            vec![]
        ))
    );
}

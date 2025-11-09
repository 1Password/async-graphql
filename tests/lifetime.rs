#![allow(clippy::diverging_sub_expression)]

use async_graphql::*;
use static_assertions_next::_core::marker::PhantomData;

#[derive(SimpleObject)]
#[allow(dead_code)]
struct ObjA<'a> {
    value: &'a i32,
}

#[expect(dead_code)]
struct ObjB<'a>(PhantomData<&'a i32>);

#[Object]
#[allow(unreachable_code)]
impl<'a> ObjB<'a> {
    async fn value(&self) -> &'a i32 {
        todo!()
    }
}

#[derive(Union)]
#[expect(dead_code)]
enum MyUnion1<'a> {
    ObjA(ObjA<'a>),
}

#[derive(Interface)]
#[graphql(field(name = "value", ty = "&&'a i32"))]
#[expect(dead_code)]
enum MyInterface<'a> {
    ObjA(ObjA<'a>),
}

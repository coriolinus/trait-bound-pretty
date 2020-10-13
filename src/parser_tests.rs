use crate::{Pretty, parser::*};

use rstest::rstest;

#[rstest(
    input,
    case("Arc"),
    case("sc_client_db::Backend"),
    case("sc_service::client::Client")
)]
fn parse_name(input: &str) {
    let result = ItemParser::new().parse(input);
    println!("{:#?}", result);
    assert!(result.is_ok());
    println!("{}", result.unwrap().pretty());
}

#[rstest(
    input,
    case("Token"),
    case("Arc<Foo>"),
    case("Bar<Bat, Baz>"),
    case("super::sub::Thing<Asd, fgh::Jkl>"),
    case("Level1<Level2<Level3<level4::Thing, level4::Other>>>")
)]
fn parse_item(input: &str) {
    let result = ItemParser::new().parse(input);
    println!("{:#?}", result);
    assert!(result.is_ok());
    println!("{}", result.unwrap().pretty());
}

#[rstest(
    input,
    case("Item<'a>"),
    case("Token<'input>"),
    case("super::sub::Thing<Asd<'gfd>, fgh::Jkl<'zxc>>")
)]
fn parse_item_with_lifetime(input: &str) {
    let result = ItemParser::new().parse(input);
    println!("{:#?}", result);
    assert!(result.is_ok());
    println!("{}", result.unwrap().pretty());
}

#[rstest(input, case("Vec<&'a str>"), case("Arc<Mutex<&'a mut Foo>>"))]
fn parse_item_with_borrowed_reference(input: &str) {
    let result = ItemParser::new().parse(input);
    println!("{:#?}", result);
    assert!(result.is_ok());
    println!("{}", result.unwrap().pretty());
}

#[rstest(input, case("Box<(Foo, Bar)>"), case("Vec<(Item<'a>, &'static str)>"))]
fn parse_item_tuple(input: &str) {
    let result = ItemParser::new().parse(input);
    println!("{:#?}", result);
    assert!(result.is_ok());
    println!("{}", result.unwrap().pretty());
}

#[test]
fn can_parse_non_static_str() {
    let input = format!("{}", "Vec<(Item<'a>, &'static str)>");
    let result = ItemParser::new().parse(&input);
    println!("{:#?}", result);
    assert!(result.is_ok());
    println!("{}", result.unwrap().pretty());
}

#[test]
fn can_parse_example_e0277() {
    let input = "error[E0277]: the trait bound `Arc<sc_service::client::Client<sc_client_db::Backend<sp_runtime::generic::Block<sp_runtime::generic::Header<u32, BlakeTwo256>, OpaqueExtrinsic>>, LocalCallExecutor<sc_client_db::Backend<sp_runtime::generic::Block<sp_runtime::generic::Header<u32, BlakeTwo256>, OpaqueExtrinsic>>, NativeExecutor<Executor>>, sp_runtime::generic::Block<sp_runtime::generic::Header<u32, BlakeTwo256>, OpaqueExtrinsic>, RuntimeApi>>: HeaderBackend<sp_runtime::generic::Block<sp_runtime::generic::Header<u32, BlakeTwo256>, OpaqueExtrinsic>>` is not satisfied";
    let result = E0277Parser::new().parse(input);
    println!("{:#?}", result);
    assert!(result.is_ok());
    println!("{}", result.unwrap().pretty());
}

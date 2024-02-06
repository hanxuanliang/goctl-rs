# goctl-rs

> A simple Rust version of [goctl](https://github.com/zeromicro/go-zero/tree/master/tools/goctl)

## Support

- [x] type struct

- [x] many type struct

- [x] nest type struct

- [x] service block

- [x] API block

## Structure

1. [token.rs](./src/token.rs) - Tokenize the input string
2. [parser.rs](./src/parser.rs) - Parse API Syntax
3. [service.rs](./src/service.rs) - Parse [Service](https://go-zero.dev/docs/tutorials#service-%E8%AF%AD%E5%8F%A5) Block
4. [struct_ref.rs](./src/struct_ref.rs) - Parse Struct Block

## Questions

> Problems encountered in development

1. Why is parse_many_struct not used in [parse_struct_stmt1](./src/struct_ref.rs), but parse_struct_to_vec is used instead?

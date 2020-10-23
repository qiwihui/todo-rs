Todo List in Rust
===

相关博客：

- [用 Rust Actix-web 写一个 Todo 应用（一）](https://github.com/qiwihui/blog/issues/105)

```shell
echo DATABASE_URL=postgres://actix:actix@localhost:5432/actix >> .env
cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration generate create_db
diesel migration run
```

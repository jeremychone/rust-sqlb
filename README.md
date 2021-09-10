
**IMPORTANT - Still experimental and extremely incomplete. All 0.0.x version will be experimental and probably break APIs in each release**

**sqlb** is intended to be a simple and progressive SQLBuilder for Rust, independent from database SQL executor.

- **Simple** - Focused on providing an expressive, composable, and typed way to build parameterized SQL statements. The goal is NOT to abstract SQL but to make it expressive, composable, and typed pragmatically.
- **Progressive** - From arbitrary data in and out (list of names/values), to eventually, struct and mapping rules. 
- **Focused** - Not an ORM, Not a database "executor/driver." Executor wrappers will be provided as features. The first wrapper will be for [sqlx](https://github.com/launchbadge/sqlx), and eventually one for [tokio-postgres](https://docs.rs/tokio-postgres/0.7.2/tokio_postgres/). But since the core `sqlb` api is about creating a parameterized sql (`builder.sql() -> String`) and a list of values (`builder.vals() -> Vec<val>`), integrating with any other database connectivity libraries should be trivial. 

> NOTE: SQL Builders are typically not to be used directly by application business logic, but rather to be wrapped in some sort of Application Data Access Layer (DAOs, DM, patterns). 

Scope for first 0.1.x releases: 

- Support for the PostgreSQL dialect. If more public interest, a dialect API will be evaluated. 
- No macros, so `Get_Fields` needs to be written by hand. As the APIs/Models mature, macros will be implemented to avoid boilerplate code.
- Currently, the Value system `Val` is extremely rudimentary, and more thought is needed to find the right model there. Feedback welcome.


## Early API Example (just conceptual for now)

Insert: 

```rust
let sb = sqlb::insert("todo").data(patch_data.fields());
let sb = sb.returning(&["id", "title"]);
let (_id, title) = sqlb::sqlx_exec::fetch_as_one::<(i64, String), _>(&db_pool, &sb).await?;
```        

Select: 

```rust
let sb = sqlb::select("todo").columns(&["id", "title"]).order_by("!id");
let todos: Vec<Todo> = sqlb::sqlx_exec::fetch_as_all(&db_pool, &sb).await?;
assert_eq!(1, todos.len());
```

The data setup: 

```rust
#[derive(sqlx::FromRow)] // Optional: to be able to use the sqlx_exec::fetch_as...
pub struct Todo {
    pub id: i64,
    pub title: String,
}

pub struct TodoPatch {
    pub title: Option<String>,
}

impl GetFields for TodoPatch {
    fn fields(&self) -> Vec<Field> {
        let mut fields = Vec::new();
        if let Some(title) = &self.title {
            fields.push(Field::from_string("title", title));
        }
        fields
    }
}
```


## For sqlb Dev

Start a postgressql

```sh
# In terminal 1 - start postges
docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=welcome  postgres:13

# In terminal 2 - launch psql on the postges instance above
docker exec -it -u postgres pg psql

# In terminal 3 - run the first rudementary tests (single thread mode to avoid db inconsistencies)
cargo test --tests -- --test-threads=1
# or watch test_sb_insert
cargo watch -q -c -x 'test --test test_sb_insert -- --test-threads=1'
# or watch test_sb_update
cargo watch -q -c -x 'test --test test_sb_update -- --test-threads=1'
# or watch test_sb_delete
cargo watch -q -c -x 'test --test test_sb_delete -- --test-threads=1'

```
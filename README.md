
**IMPORTANT - 0.0.x versions will be experimental and probably break APIs in each release.**

**sqlb** is (will be) be a simple, expressive, and progressive SQLBuilder for Rust.

- **Simple** - Focused on providing an expressive, composable, and reasonnably typed scheme to build and execute (via sqlx for now) parameterized SQL statements. The goal is NOT to abstract SQL but to make it expressive and composable using Rust programmatic constructs.
	- **Not** a database **executor/driver** (SQLX and tokio-postgres to name a few are excellent)
	- **Not** an ORM, althought eventually, one could build an ORM on top of it. 
- **Progressive** - From arbitrary data in and out (list of names/values), to eventually, struct and mapping rules. 
- **Focused** - Not an ORM, Not a database "executor/driver." 
	- **[sqlx](https://crates.io/crates/sqlx)** - The first "database executor" provided will be [sqlx](https://github.com/launchbadge/sqlx). 
	- **PostgreSQL** first, and then, as interest drives, multiple databases via sqlx. 
	- **[tokio-postgres](https://docs.rs/tokio-postgres/0.7.2/tokio_postgres/)** might be part of the plan as well, as it provides some interesting benefits visavis of query concurrency. 
	

> NOTE: SQL Builders are typically not used directly by application business logic, but rather to be wrapped in some Application Data Access Layer (DAOs, DM, patterns). So rather than exposing an ORM API to the business logic, "Data Access Object" or "Model Access Object" interfaces are implemented via an SQLBuilder and then provide secure and constrained API and types to the rest of the application code. 


Goals for first **0.1.x** releases: 

- **sqlx** - Will probably be SQLX centric. 
- **PostgreSQL** - Will probably support SQLX only. Contributions for another database (via sqlx) welcome
- **Macros** - Might not have many macros. The goal is to have a clean API first and then provide macros to reduce boilerplates. 


## Early API Example (just conceptual for now)

```rust
// INSERT - Insert a new Todo from a Partial todo
let sb = sqlb::insert("todo").data(patch_data.fields());
let sb = sb.returning(&["id", "title"]);
let (_id, title) = sqlb::sqlx_exec::fetch_as_one::<(i64, String), _>(&db_pool, &sb).await?;

// SELECT - Get all todos
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
			fields.push(("title", title).into());
		}
		fields
	}
}
```


## For sqlb Dev

Start a PostgreSQL

```sh
# In terminal 1 - start postges
docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=welcome  postgres:14

# In terminal 2 - (optional) launch psql on the Postgres instance above
docker exec -it -u postgres pg psql

# In terminal 3 - MUST run with `--test-threads=1` to avoid database access conflicts
cargo test -- --test-threads=1

# or watch a particular test target
cargo watch -q -c -x 'test --test test_sb_insert -- --test-threads=1'
```
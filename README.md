**sqlb** is a simple and expressive SQLBuilder for Rust for [sqlx](https://crates.io/crates/sqlx) focused on PostgreSQL (for now). 


- **Simple** - Focused on providing an expressive, composable, and reasonably typed scheme to build and execute (via sqlx for now) parameterized SQL statements. The goal is NOT to abstract SQL but to make it expressive and composable using Rust programmatic constructs.
	- **NOT** a database **executor/driver** (Uses [sqlx](https://crates.io/crates/sqlx) as sql executore)
	- **NOT** an **ORM**, just a sql builder.
	- **NOT** a full replacement for [sqlx](https://crates.io/crates/sqlx). Dropping into sqlx when sqlb is too limiting is a valid pattern.
- **Expressive** - From arbitrary typed data in and out (list of names/values) to struct and mapping rules. 
- **Focused** 
	- **[sqlx](https://crates.io/crates/sqlx)** - The first "database executor" provided will be [sqlx](https://github.com/launchbadge/sqlx). 
	- **PostgreSQL** - First database support will be Postgres (via sqlx). Additional database support may be added based on interest and pull requests.
- `sqlb` goal is to have a highly ergonomic API at a minimum performance cost. However, using sqlx directly for high bach commands or more advanced usecases is an encouraged approach. 
- **Prepared Statement ONLY!**	


> WARNING: During the `0.y.z` period API changes will result in `.y` increments. 

> NOTE: SQL Builders are typically not used directly by application business logic, but rather to be wrapped in some Application Model Access Layer (e.g., DAOs or MCs - Model Controller - patterns). Even when using ORMs, it is often a good code design to wrap those access via some model access layers. 


Goals for first **0.x.x** releases: 

- **sqlx** - Only plan to be on top of [sqlx](https://crates.io/crates/sqlx).
- **PostgreSQL** - Focus only on PostgreSQL.
- **Macros** - Adding macros to keep things DRY (but they are optional. All can be implemented via trait objects)
- **limitations** - Currently, to make types work with `sqlb` they must implement`sqlb::SqlxBindable` trait. The aim is to implement `SqlxBindable` for all `sqlx` types and allowing app code to implement `SqlxBindable` for their specific types. If there are any external types that should be supported but are not currently, please feel free to log a ticket.


## Early API Example (just conceptual for now)

```rust
// `sqlx::FromRow` allows to do sqlx_exec::fetch_as...
// `sqlb::Fields` allows to have:
//   - `toto.fields()` (name, value)[] (only direct or NOT Not values)
//   - `Todo::field_names()` here would return `["id", "title"]`
#[derive(sqlx::FromRow, sqlb::Fields)] 
pub struct Todo {
    id: i64,

    title: String,
	#[field(name="description")]
	desc: Option<String>,

	#[field(skip)]
	someting_else: String,
}

#[derive(sqlb::Fields)] 
pub struct TodoForCreate {
	title: String,
	desc: Option<String>,

	#[field(skip)]
	someting_else: String,	
}

#[derive(sqlb::Fields)] 
pub struct TodoForUpdate {
	title: Option<String>,
	desc: Option<String>,
}

// -- Get the field names
let field_names = Todo::field_names();
// ["id", "title", "description"]

// -- Create new row
let todo_c = TodoForCreate { title: "title 01".to_string(), desc: "desc 01".to_string() };
// will update all fields specified in TodoForCreate
let sb = sqlb::insert().table("todo").data(todo_c.all_fields());
let sb = sb.returning(&["id", "title"]);
let (_id, title) = sb.fetch_one::<_, (i64, String)>(&db_pool).await?;

// -- Select 
let sb = sqlb::select().table("todo").columns(Todo::field_names()).order_by("!id");
let todos: Vec<Todo> = sb.fetch_as_all(&db_pool).await?;

// -- Update
let todo_u - TodoForUpdate { desc: "Updated desc 01".to_string()};
let sb = sqlb::update().table("todo").data(todo_u.not_none_fields()).and_where_eq("id", 123);
let row_affected = sb.exec(&db_pool).await?;
// will not update .title because of the use of `.not_none_fields()`. 
```

## Latest Major Changes

`!` breaking change, `+` addition, `-` fix.

- `0.3.1` 
	- `!` BREAKING CHANGE - `HasFields.fields` has been rename to `HasFields.not_none_fields()`.
	- `!` BREAKING CHANGE - `HasFields.not_none_fields()` and `HasFields.all_fields()` consume the `self` (to avoid uncessary clone).
	- `+` - `HasFields.all_fields()` - returns all fields (even the one where value are None).
	- `+` - `HasFields::field_names(): &'static [&'static]` - list of field names (i.e., column names).
	- `+` - Added `SqlxBindable` for the `Option<T>` (not a blanket impl at this point).
	- `0.3.0` been deprecated since did not have the `...fields(self)` behavior. 
- `0.2.0` 
	- Changing the generic order to match `sqlx`. From `.fetch_one::<(i64, String), _>` to `.fetch_one::<_, (i64, String)>`
- `0.0.7` 
	- `sqlb::insert().table("todo")` (in 0.0.7) rather than `sqlb::insert("toto")` (<=0.0.6) (for all SqlBuilders)


## For sqlb Dev

Start a PostgreSQL

```sh
# In terminal 1 - start postges
docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=welcome  postgres:14

# In terminal 2 - (optional) launch psql on the Postgres instance above
docker exec -it -u postgres pg psql

# In terminal 3 -
cargo test

# or watch a particular test target
cargo watch -q -c -x 'test --test test_sb_insert
```
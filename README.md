# IMPORTANT - sqlb 0.5.x has a TOTALLY NEW STRATEGY and IMPLEMENTATION

- It is now not built on top of `sqlx` anymore (no sqlx dependencies)
- But on top of [sea-query](https://crates.io/crates/sea-query)
- So, instead of providing a `sea-query` alternative solution, it now adds on top of `sea-query`

The result is a bit more verbose, but it takes full advantage of `sea-query`:

- Mature and expressive SQL builder API, with joins and more.
- Multi-driver support (`postgres/tokio-postgres`, `rusqlite`, `sqlx/*` `diesel/*`)

So, `sqlb` just adds the missing mile:

- `HasFields` trait on struct, to provide a list of columns, and columns/values for struct instance.
- `#[derive(Fields)]` proc derive macro to implement `HasFields`
- `fields.zip()` and `fields.unzip()` to conveniently transform a field list to the columns/values expected by `sea-query` query builders.

And more to come. 

See [Rust web-app production code blueprint on rust10x.com](https://rust10x.com/web-app) for an example of how this is used in a production code envornement

## Quick Example

- When annotating a struct with `#[Derive(sqlb::Fields)]`

```rust
// sqlb::Fields 
#[derive(sqlb::Fields, Debug, Default)]
struct TodoForCreate {
	title: String,
	done: Option<bool>, // if None, not set, so db default (false)
	#[field(name = "description")]
	desc: Option<String>, // if None, not set, so db default (false)
}
```

- The following functions/methods are available:

| Function/Method                | Returns                                                  |
|--------------------------------|----------------------------------------------------------|
| `TodoForCreate::field_names()` | `["title", "done", "description"]`                       |
| `TodoForCreate::field_idens()` | `Vec<sea_query::DynIden>` (for sea-query select)         |
| `todo_object.all_fields()`     | `Fields` object allowing the following                   |
| `fields.zip()`                 | `(Vec<DynIden>, Vec<SimpleExpr>)` for sea-query insert   |
| `fields.unzip()`               | `Iterator of (DynIden, SimpleExpr)` for sea-query update |
| `fields.push(Field::new(...))` | To add dynamic name/value to be inserted/updated         |



## Full examples

See [alpha_v050/examples](https://github.com/jeremychone/rust-sqlb/tree/alpha_v050/examples)

```sh
cargo run -p example-tokio-postgres
cargo run -p example-sqlx-postgres
```

### Notes for other databases

> Note: Currently, `sqlb` is completely DB-unaware, meaning that the examples provided above could be adapted to MySQL or SQLite by simply changing the DB Driver and Sea-Query binding dependency. For reference, see [sea-query examples](https://github.com/SeaQL/sea-query/tree/master/examples).

## Changelog

`!` breaking change, `^` enhancement, `+` addition, `-` fix.

- `0.5.0-alpha.x` FULL REWRITE AND DIFFERENT STRATEGY - SEA-QUERY based now
- `0.4.0` - 2023-11-21
	- `^` Updated to `sqlx 0.7`
- `0.3.3 .. 0.3.8` - 2023-08-03
	- `+` generic types for `bindable!` macro. [PR from KaiserBh](https://github.com/jeremychone/rust-sqlb/pull/10)
	- `+` `chrono` binding under the feature `chrono_support`. [PR from KaiserBh](https://github.com/jeremychone/rust-sqlb/pull/10)
	- Thanks to [eboody](https://github.com/eboody) for the potential sqlx conflict (see [PR 3](https://github.com/jeremychone/rust-sqlb/pull/3)).
- `0.3.2 .. 0.3.7`
	- `+` Add support for partial and fully qualified table and column names. #8
	- `+` Add `SqlxBindable` blanket implementation for `Option<T>`. #7
	- `+` Add `.limit(..)` and `.offset(..)` for `Select`.
	- `+` Add `.count()` for `Select`.
	- `+` Add `#[field(skip)]` and `#[field(name="other_name")]` to skip or rename properties.
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
docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=welcome  postgres:15

# In terminal 2 - (optional) launch psql on the Postgres instance above
docker exec -it -u postgres pg psql

# In terminal 3 -
cargo test

# or watch a particular test target
cargo watch -q -c -x 'test --test test_sb_insert
```

<br />
[sqlb github repo](https://github.com/jeremychone/rust-sqlb)

Small experiment to see what UUIDv7 looks/works like.

```shell
git clone https://github.com/rubin55/rusty-tricks
cd rusty-tricks/sqlite
touch test.sqlite
cargo run
# will create a million-row test.sqlite with an
# iteration order, some-data and uuidv7 id field
```

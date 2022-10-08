# r_jq
A Rust based json query utility patterned after the famous jq utility.
This app has no real value other than I wanted to be able to query
[serde_json] structures, in a similar manner to jq.

The examples that follow use the included [rjq](./rjq) command line utility.

# Supported Capabilities

## Basic Filters

### Identity `.`

````sh
> echo '"Hello World!"' | rjq '.'
"Hello World!"

````
### Object Identifier-Index `.foo`, `.foo.bar`

````sh
> echo '{"foo": 42, "bar": "less interesting data"}'| rjq '.foo'
42
````

````sh
> echo '{"notfoo": true, "alsonotfoo": false}' | rjq '.foo'
null
````

### Generic Object Index `.[<string>]`

````sh
> echo '{"foo": 42}' | rjq '.["foo"]'
42
````

### Array Index `.[2]`
Array indexes return the nth element of an array.  

If n >= 0, array[n] is returned.

If the n < 0, array[array.len - n] is returned.

If the index is out of bounds, `null` is returned.


````sh
> echo '[{"name":"JSON", "good":true}, {"name":"XML", "good":false}]'
Output	{"name":"JSON", "good":true}
'rjq '.[0]'
{"name":"JSON", "good":true}
````

````sh
> echo '[1,2,3]' | rjq '.[-2]'
2
````

### Array Slice
### Array/Object Value Iterator
### Comma
### Pipe

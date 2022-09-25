# r_jq
A Rust based json query utility patterned after the famous jq utility.
This app has no real value other than I wanted to be able to query
[serde_json] structures, in a similar manner to jq.

# Supported Capabilities

## Basic Filters

### Identity `.`

````sh
> echo '"Hello World!"' | r_jq '.'
"Hello World!"

````
### Object Identifier-Index `.foo`, `.foo.bar`

````sh
> echo '{"foo": 42, "bar": "less interesting data"}'| r_jq '.foo'
42
````


````sh
> echo '{"notfoo": true, "alsonotfoo": false}' | r_jq '.foo'
null
````


### Generic Object Index `.[<string>]`

````sh
> echo '{"foo": 42}' | r_jq '.["foo"]'
42
````



### Array Index `.[2]`

````sh
> echo '[{"name":"JSON", "good":true}, {"name":"XML", "good":false}]'
Output	{"name":"JSON", "good":true}
'r_jq '.[0]'
{"name":"JSON", "good":true}
````

````sh
> echo '[1,2,3]' | r_jq '.[-2'
2
````

### Array Slice
### Array/Object Value Iterator
### Comma
### Pipe

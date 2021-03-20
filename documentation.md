# Documentation

* [HTTP Requests](#S-requests)
* [Macro attributes](#S-attributes)
* [Supported configuration field types](#S-types)

# <a name="S-requests"></a>HTTP Requests

### **GET** `/config`
**Status code**: `200`\
**Body**: List of all configuration fields\
**Example**: `curl localhost:8081/config/`

<br />

### **GET** `/config/<configuration field>`
**Status code**: `200` if the field exists, otherwise `404`\
**Body**: The value of the specified configuration field\
**Example**: `curl localhost:8081/config/logfile`

<br />

### **PUT** `/config/<configuration field> <new value>` 
**Status code**: `200` if the new value has been set, `404` if the field doesn't exist or `400` if the new value is invalid\
**Body**: An error message if the new value is invalid\
**Example**: `curl -X PUT localhost:8081/config/filename -d "file.txt"`

# <a name="S-attributes"></a>Macro attributes

Attribute name | Value | Position | Usage | Effect
-------------- | ----- | -------- | ------ | ------
path | `String` | `struct` | `#[choices(path = "myconfig")]` | sets the root path of the configuration HTTP service
json | | `struct` | `#[choices(json)]` | requests and responses content is in json
on_set | `Expression` | `field` | `#[choices(on_set = print_value)]` | invokes an expression in the form `expr(&v)` where `v` is the new value (note: the old value is replaced after this call returns)
skip | | `field` | `#[choices(skip)]` | do not treat this field as a 'configuration field'
hide_get | | `field` | `#[choices(hide_get)]` | do not generate the HTTP GET for this field
hide_put | | `field` | `#[choices(hide_put)]` | do not generate the HTTP PUT for this field
validator | `Expression` | `field` | `#[choices(validator = check_value)]` | invokes an expression in the form `expr(&v) -> ChoicesResult<()>` where `v` is the new value; the field's value is updated only if the result is `Ok`

# <a name="S-types"></a>Supported configuration field types

Type | Text | Json | Notes
---- |:----:|:----:| -----
`bool` | :heavy_check_mark: | :heavy_check_mark: | 
`char` | :heavy_check_mark: | :heavy_check_mark: | 
`i128` | :heavy_check_mark: | :heavy_check_mark: | 
`i16` | :heavy_check_mark: | :heavy_check_mark: | 
`i32` | :heavy_check_mark: | :heavy_check_mark: | 
`i64` | :heavy_check_mark: | :heavy_check_mark: | 
`i8` | :heavy_check_mark: | :heavy_check_mark: | 
`isize` | :heavy_check_mark: | :heavy_check_mark: | 
`u128` | :heavy_check_mark: | :heavy_check_mark: | 
`u16` | :heavy_check_mark: | :heavy_check_mark: | 
`u32` | :heavy_check_mark: | :heavy_check_mark: | 
`u64` | :heavy_check_mark: | :heavy_check_mark: | 
`u8` | :heavy_check_mark: | :heavy_check_mark: | 
`usize` | :heavy_check_mark: | :heavy_check_mark: | 
`f32` | :heavy_check_mark: | :heavy_check_mark: | 
`f64` | :heavy_check_mark: | :heavy_check_mark: | 
`String` | :heavy_check_mark: | :heavy_check_mark: | 
`Option<T>` | :heavy_check_mark: | :heavy_check_mark: | `T` must be supported 
user defined `Type` and `Type<T, ...>` | :heavy_check_mark: | | user must implement the traits `ChoicesInput` and `ChoicesOutput` 
any `Type` and `Type<T, ...>` | | :heavy_check_mark: | type must be serializable and deserializable with `serde` 

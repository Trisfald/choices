# Documentation

* [HTTP Methods](#S-methods)
* [Supported types](#S-types)

# <a name="S-methods"></a>HTTP Methods

### **GET** `/config`
#### **Status code**: `200`
#### **Body**: List of all configuration fields
#### **Example**: `curl localhost:8081/config/`

<br />

### **GET** `/config/<configuration field>`
#### **Status code**: `200` if the field exists, otherwise `405`
#### **Body**: The value of the specified configuration field 
#### **Example**: `curl localhost:8081/config/logfile`

<br />

### **PUT** `/config/<configuration field> <new value>` 
#### **Status code**: `200` if the new value has been set, `405` if the field doesn't exist or `400` if the new value is invalid
#### **Body**: An error message if the new value is invalid
#### **Example**: `curl -X PUT localhost:8081/config/filename -d "file.txt"`

# <a name="S-types"></a>Supported types

Type | Default (raw strings) | Json | Notes
---- | --------------------- | ---- | -----
`bool` | :heavy_check_mark: | | 
`char` | :heavy_check_mark: | | 
`i128` | :heavy_check_mark: | | 
`i16` | :heavy_check_mark: | | 
`i32` | :heavy_check_mark: | | 
`i64` | :heavy_check_mark: | | 
`i8` | :heavy_check_mark: | | 
`isize` | :heavy_check_mark: | | 
`u128` | :heavy_check_mark: | | 
`u16` | :heavy_check_mark: | | 
`u32` | :heavy_check_mark: | | 
`u64` | :heavy_check_mark: | | 
`u8` | :heavy_check_mark: | | 
`usize` | :heavy_check_mark: | | 
`f32` | :heavy_check_mark: | | 
`f64` | :heavy_check_mark: | | 
`String` | :heavy_check_mark: | | 
`Option<T>` | :heavy_check_mark: | | `T` must be supported 

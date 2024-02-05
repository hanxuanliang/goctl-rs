# goctl-rs

> A simple Rust version of [goctl](https://github.com/zeromicro/go-zero/tree/master/tools/goctl)

## Support

- [x] type struct

- [x] many type struct

- [x] nest type struct

- [x] service block

## Structure

1. [token.rs](./src/token.rs) - Tokenize the input string
2. [service.rs](./src/service.rs) - Parse [Service](https://go-zero.dev/docs/tutorials#service-%E8%AF%AD%E5%8F%A5) Block
3. [struct_ref.rs](./src/struct_ref.rs) - Parse Struct Block

## API Synax

```go
// type struct
type GetFormReq struct {
    Name  string `form:"name,omitempty"`
    Age   int64  `form:"age" json:"age"`
}

// many type struct
type GetFormReq struct {
    Name  string `form:"name,omitempty"`
    Age   int64  `form:"age" json:"age"`
}
type GetFormResp struct {
    Total int64 `json:"total"`
}

// nest type struct
type (
	GetFormReq struct {
    	Name  string `form:"name,omitempty"`
    	Age   int64  `form:"age" json:"age"`
	}
	GetFormResp struct {
    	Total int64 `json:"total"`
	}
)
```


# goctl-rs

> A simple Rust version of [goctl](https://github.com/zeromicro/go-zero/tree/master/tools/goctl)

## Dependencies

```toml
[dependencies]
indexmap = "2.2.2"
logos = "0.13.0"
nom = "7.1.3"
```

## Support

- [x] type struct

- [x] many type struct

- [x] nest type struct


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


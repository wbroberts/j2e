# j2e

A simple CLI tool to convert json files into env files. I created it to make managing env files with json blobs easier to manage and update.

## How to Use

```bash
j2e --help
# or
j2e variables.json .env.local
```
### Example

```json
{
    "string_val": "hello",
    "int_val": 42,
    "obj_val": {
        "foo": "bar"
    },
    "arr_val": [
        "foo",
        "bar"
    ]
}
```

```env
INT_VAL=42
OBJ_VAL={"foo":"bar"}
ARR_VAL=["foo","bar"]
STRING_VAL=hello
```

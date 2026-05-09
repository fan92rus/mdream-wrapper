# mdream-wrapper

C ABI wrapper around [mdream](https://github.com/harlan-zw/mdream) for P/Invoke from .NET.

## Build

```bash
cargo build --release
```

Output: `target/release/mdream_wrapper.dll`

## Exports

| Function | Signature | Description |
|---|---|---|
| `mdream_convert` | `(html: *const c_char, origin: *const c_char) -> *mut c_char` | Convert HTML to Markdown. Returns null-terminated UTF-8 string. |
| `mdream_free` | `(ptr: *mut c_char)` | Free string returned by `mdream_convert`. |

## C# Usage

```csharp
[DllImport("mdream_wrapper", CharSet = CharSet.Ansi)]
static extern IntPtr mdream_convert(string html, string? origin);

[DllImport("mdream_wrapper")]
static extern void mdream_free(IntPtr ptr);

var ptr = mdream_convert("<h1>Hello</h1>", null);
var markdown = Marshal.PtrToStringUTF8(ptr);
mdream_free(ptr);
```

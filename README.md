# mdream-wrapper

C# wrapper with native [mdream](https://github.com/harlan-zw/mdream) (HTML to Markdown) via P/Invoke. Distributed as a NuGet package — zero Node.js dependency.

## Install

```bash
dotnet nuget add source https://nuget.pkg.github.com/fan92rus/index.json -n github -u fan92rus -p <TOKEN>
dotnet add package MdreamWrapper
```

## Usage

```csharp
using MdreamWrapper;

var converter = new MdreamConverter();
var result = await converter.ConvertAsync(
    "<h1>Hello</h1><p>Some <strong>HTML</strong></p>",
    originUrl: "https://example.com");

Console.WriteLine(result.Markdown);
// # Hello
//
// Some **HTML**
```

Or use the native API directly:

```csharp
using MdreamWrapper;

var markdown = MdreamNative.Convert("<h1>Hello</h1>", "https://example.com");
Console.WriteLine(markdown);
```

## API

### `MdreamConverter`

High-level async wrapper.

| Method | Description |
|---|---|
| `ConvertAsync(html, timeout?, originUrl?)` | Returns `MdreamResult` with `Success`, `Markdown`, `Error` |

### `MdreamNative`

Static P/Invoke wrapper — synchronous, fastest.

| Method | Description |
|---|---|
| `Convert(html, origin?)` | Returns `string?` (null on error) |

### `MdreamResult`

| Property | Type | Description |
|---|---|---|
| `Success` | `bool` | Whether conversion succeeded |
| `Markdown` | `string?` | Result markdown |
| `Error` | `string?` | Error message if failed |

## Building from source

Requires [Rust](https://rustup.rs) and .NET 9 SDK.

```bash
# Native DLL
cargo build --release

# NuGet package (local)
dotnet build lib/MdreamWrapper.csproj -c Release -o artifacts/managed/net9.0
mkdir -p artifacts/win-x64
cp target/release/mdream_wrapper.dll artifacts/win-x64/
nuget pack MdreamWrapper.nuspec -OutputDirectory dist
```

## CI

Pushing a `v*` tag triggers GitHub Actions: builds native + managed, packs NuGet, publishes to GitHub Packages.

```bash
git tag v1.3.0
git push origin v1.3.0
```

using System.Runtime.InteropServices;

namespace MdreamWrapper;

public class MdreamResult
{
    public bool Success { get; init; }
    public string? Markdown { get; init; }
    public string? Error { get; init; }
}

public static class MdreamNative
{
    private const string DllName = "mdream_wrapper";

    [DllImport(DllName, CharSet = CharSet.Ansi)]
    private static extern IntPtr mdream_convert(string html, string? origin);

    [DllImport(DllName)]
    private static extern void mdream_free(IntPtr ptr);

    public static string? Convert(string html, string? origin = null)
    {
        var ptr = mdream_convert(html, origin);
        if (ptr == IntPtr.Zero) return null;

        try
        {
            return Marshal.PtrToStringUTF8(ptr);
        }
        finally
        {
            mdream_free(ptr);
        }
    }
}

public class MdreamConverter
{
    public Task<MdreamResult> ConvertAsync(
        string html,
        TimeSpan? timeout = null,
        string? originUrl = null)
    {
        return Task.Run(() =>
        {
            var result = MdreamNative.Convert(html, originUrl);

            return result is not null
                ? new MdreamResult { Success = true, Markdown = result }
                : new MdreamResult { Success = false, Error = "mdream_convert returned null (invalid input or DLL not found)" };
        });
    }
}

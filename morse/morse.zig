const std = @import("std");
const page_allocator = std.heap.page_allocator;

const characters = [_][]const u8{
    ".-", // A
    "-...", // B
    "-.-.", // C
    "-..", // D
    ".", // E
    "..-.", // F
    "--.", // G
    "....", // H
    "..", // I
    ".---", // J
    "-.-", // K
    ".-..", // L
    "--", // M
    "-.", // N
    "---", // O
    ".--.", // P
    "--.-", // Q
    ".-.", // R
    "...", // S
    "-", // T
    "..-", // U
    "...-", // V
    ".--", // W
    "-..-", // X
    "-.--", // Y
    "--..", // Z
    "-----", // 0
    ".----", // 1
    "..---", // 2
    "...--", // 3
    "....-", // 4
    ".....", // 5
    "-....", // 6
    "--...", // 7
    "---..", // 8
    "----.", // 9
    "-.-.--", // !
    "--..--", // ,
    "-....-", // -
    ".-.-.-", // .
    "-..-.", // /
    "---...", // :
    "-.-.-.", // ;
    "-...-", // =
    "..--..", // ?
};

fn getFileContent(filename: []const u8) ![]const u8 {
    const file = try std.fs.cwd().openFile(filename, .{});
    defer file.close();

    const file_size = try file.getEndPos();
    const buffer = try page_allocator.alloc(u8, file_size);

    _ = try file.readAll(buffer);

    return buffer;
}

fn getCharCode(i: u8) u8 {
    const character: u8 =
        if (i < 26) 'A' + i else if (i >= 26 and i <= 36) '0' + (i - 26) else switch (i) {
            36 => '!',
            37 => ',',
            38 => '-',
            39 => '.',
            40 => '/',
            41 => ':',
            42 => ';',
            43 => '=',
            44 => '?',
            else => return '`',
        };

    return character;
}

fn binaryToAlpha(source: []const u8) ![]u8 {
    var result = std.ArrayList(u8).init(page_allocator);
    defer result.deinit();

    var iterator = std.mem.tokenizeAny(u8, source, " \n");
    while (iterator.next()) |morse_code| {
        if (std.mem.eql(u8, morse_code, "/")) {
            try result.append(' ');
            continue;
        }

        for (characters, 0..) |code, i| {
            if (std.mem.eql(u8, morse_code, code)) {
                try result.append(getCharCode(@intCast(i)));
                break;
            }
        }
    }

    return try result.toOwnedSlice();
}

pub fn main() !void {
    const filename = "morse.txt";
    const target_file = try getFileContent(filename);
    defer page_allocator.free(target_file);

    const result = try binaryToAlpha(target_file);

    try std.io.getStdOut().writer().print("{s}\n", .{result});
}

const std = @import("std");
const data = @embedFile("input.txt");

fn findFirstFourteen(findString: *const [4095:0]u8, window: u8) u64 {
    const one: u32 = 1;

    var check: u32 = 0;
    var count: u32 = 0;

    var i: u64 = window - 1;
    while (i < findString.len) {
        const end = (i + 1) - window;

        var w: u64 = i;
        while (w >= end) {
            const marker: u32 = one << @truncate(findString[w] & 31);

            if ((check & marker) == 0) {
                check = check | marker;
                count += 1;
            } else {
                i = w + window;
                check = 0;
                count = 0;
                break;
            }

            if (count == window) {
                return i + 1;
            }

            w -= 1;
        }
    }

    return 0;
}

pub fn main() !void {
    const before = std.time.nanoTimestamp();
    const start = findFirstFourteen(data, 14);
    const after = std.time.nanoTimestamp();

    const timeTaken = after - before;

    std.debug.print("findStart found {d} and took {d} nanoseconds\n", .{ start, timeTaken });
}

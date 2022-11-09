interface A {
    (a = 1, b = 0xabc, c = 0O123, d = 0123, d = 0b0111): string;
    (e = 12, f = 12.3, g = 12.3e-2, h = .3e2, i = 15e3): string;
    (a = true, b = false, c = null): void;
}
interface A extends B, C {
    // color?: string;
    // width?(a: number);
    (source: string = "abc", subString: string): boolean;
    setTime(d: Date): Date;
    new(hour: number, minute: number): ClockInterface;
}
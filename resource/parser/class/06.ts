abstract class B implements A {
    abstract prop: string;
    abstract readonly ro: string;
    abstract get readonlyProp(): string;
    abstract m(): string;
    abstract get mismatch(): string;
    abstract set mismatch(val: number); // error, not same type

    readonly ro = "readonly please";
    abstract notAllowed: string;
    get concreteWithNoBody(): string;

    get num() { return "nope, wrong"; }

    abstract get p1(): string;
    set p1(val: string) { };
    get p2(): string { return "should work"; }
    abstract set p2(val: string);
}
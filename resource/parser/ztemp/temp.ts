import Namespace from "./b";
export var x = new Namespace.Foo(1, 2, 3);

// @Filename: b.d.ts
export class Foo {
    member: string;
}
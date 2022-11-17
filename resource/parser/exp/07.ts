((a, b, c) => {
    abstract class A { }
    class B extends A { }
    new B();
    return A;
})(a, b, c);

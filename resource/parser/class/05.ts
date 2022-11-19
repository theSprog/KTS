abstract class AbstractClass {
    constructor(str: string, other: AbstractClass) {
        super(str, other);
        this.method(parseInt(str));
        let val = this.prop.toLowerCase();

        if (!str) {
            this.prop = "Hello World";
        }
        this.cb(str);

        // OK, reference is inside function
        const innerFunction = () => {
            return this.prop;
        }

        // OK, references are to another instance
        other.cb(other.prop);
    }

    abstract prop: string;
    abstract cb: (s: string) => void;

    abstract method(num: number): void;

    other = this.prop;
    fn = () => this.prop;
    cb = (s: string) => { };

    method2() {
        this.prop = this.prop + "!";
    }
}
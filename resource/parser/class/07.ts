class LanguageSpec {
    public set SetterFirst(a: number) { }
    public get SetterFirst() { return ""; }

    public get GetterFirst(): string { return ""; }
    public set GetterFirst(aStr) { aStr = 0; }
}
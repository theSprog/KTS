interface A {
    next: () => {
        value: any
        done: boolean
    }
}

interface A {
    [a: string]: {
        value: any
        done: boolean
    }
}
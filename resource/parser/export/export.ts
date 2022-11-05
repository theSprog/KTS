// from block
export { variable1 as name1, variable2 as name2, nameN } from "abc mod";
export default { name1, name2, nameN } from "xxx";
export * from "abc mod";
export { };
export { ; };
// // statement
// export default AddTwoNumbers;
// export const foo: number = Math.PI * Math.sqrt(2);

// // statement -> block
// export { AddNumbers, SubtractTwoNumbers };
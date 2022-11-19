export const zzz = 123;
export default zzz;

// @filename: a.ts
export default interface zzz {
    x: string;
}

import zzz from "./b";

function setkv(key: any, val: any) {
    this.items[key] = val;
}

export { zzz as default2 };

// // @filename: index.ts
import zzz from "./a";

import originalZZZ from "./b";
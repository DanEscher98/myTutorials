"use strict";
function myTest(args) {
    if (args.name) {
        return "Hello ".concat(args.name);
    }
    return "Hello Word";
}
console.log(myTest({ id: 1, name: "Daniel" }));

const f = function () { };
var g = f;
g.prototype.m = function () {
    this;
};
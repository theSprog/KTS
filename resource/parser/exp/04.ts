// a ? b : c
// a ? b : c ? d : e
// a ? b : c ? d : e ? f : g
// a ? b ? c : d : e
// a ? b : c ? d ? e : f : g ? h : i
// (a ? b : c) ? d ? e : f : g ? h : i
// a ? b : c ? (d ? e : f) : g ? h : i

// a > b ? c + d : e[f ? g : h]
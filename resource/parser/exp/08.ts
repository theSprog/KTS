definition.description = comments.map(
    comment => comment.kind === "lineBreak" ?
        comment.text :
        comment.text.trim().replace("\n"))
    .join("");

doc => {
    if (this.userValidationKeywords[name]) {
        definition[name] = this.parseValue(text);
    } else {
        otherAnnotations[doc.name] = true;
    }
}
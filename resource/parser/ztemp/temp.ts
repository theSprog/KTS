export function compile(fileNames: string[], options: ts.CompilerOptions): void {
    var program = ts.createProgram(fileNames, options);
    var emitResult = program.emit();

    var allDiagnostics = ts.getPreEmitDiagnostics(program);

    allDiagnostics.forEach(diagnostic => {
        var message = ts.flattenDiagnosticMessageText(diagnostic.messageText, '\n');
        if (!diagnostic.file) {
            console.log(message);
            return;
        }
        console.log();
    });

    var exitCode = emitResult.emitSkipped ? 1 : 0;
    process.exit(exitCode);
}

compile(process.argv.slice(2));
var child_process = require('child_process');
var path = require('path');

exports.handler = function(event, context, callback) {
    child_process.execFile('/bin/cp', ['--no-target-directory', path.resolve("./txtof"), '/tmp/txtof']);
    child_process.execFile('/bin/chmod', ['777', '/tmp/txtof']);

    var child = child_process.spawn('/tmp/txtof');
    child.stdin.write(event["data"]);
   
    var out = "";
    child.stdout.on('data', function (data) { out += data; });
    
    child.on('close', function(code) {
        if(code !== 0) return context.done(out);
        context.done(null, out);
    });

    child.stdin.end();
}

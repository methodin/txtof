var child_process = require('child_process');
var path = require('path');

exports.handler = function(event, context, callback) {
    child_process.execFile('/bin/cp', ['--no-target-directory', path.resolve("./txtof"), '/tmp/txtof']);
    child_process.execFile('/bin/chmod', ['777', '/tmp/txtof']);

    var json = JSON.parse(event['body']);
    var child = child_process.spawn('/tmp/txtof');
    var out = "";

    child.stdin.write(json["data"]);
    child.stdout.on('data', function (data) { out += data; });
    child.on('close', function(code) {
        if(code !== 0) {
            return context.done(out);
        }
        
        var responseBody = {
            html: out
        };

        var response = {
            statusCode: 200,
            body: JSON.stringify(responseBody)
        };

        context.done(null, response);
    });
    child.stdin.end();
}


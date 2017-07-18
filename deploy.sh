#~/bin.bash
cargo build --release
zip -j txtof.zip lambda.js target/release/txtof
aws lambda update-function-code --function-name txtof --zip-file fileb://txtof.zip 
rm txtof.zip

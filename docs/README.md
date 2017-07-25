# hello
Simple text-based form generator for prototyping

# Usage

txtof accepts standard input (e.g. via | or <) as well as an optional parameter for a template path

## Lambda

This code is built to run on AWS Lambda as well as through the console. You can tweak the lambda.js function contained in the project and make sure you have the handler set to **lambda.handler** in your Lambda config.

You can also use the Lambda environment veriable section to pass in a comma-separated template setup like below using hte key **template**:

```
<div class="container">,<div class="row">,<div class="col">,<div>,</div>,</div>,</div>,</div>,<label>{{value}}</label>,<input type="text" class="form-control" value="{{value}}"/>,<label class="form-check-label"><input type="checkbox" class="form-check-input"/>{{value}}</label>,<label class="form-check-label"><input type="radio" class="form-check-input"/>{{value}}</label>,<textarea class="form-control">{{value}}</textarea>,<button class="btn btn-primary">{{value}}</button>,<select class="form-control">{{#each value}}<option>{{this}}</option>{{/each}}</select>,<hr/>
```

## Command examples

Running simple via pipe

```sh
echo "| (button)" | txtof

<div><span> <button>button</button><br/></span></div>
```

Converting data from an input txt file

```sh
txtof < example.txt

<div><span> <button>button</button><br/></span></div>
```
Using a custom template

```sh
txtof ~/boostrap4.tmpl < example.txt

<div class="container">
<div class="row"><div class="col"><div> <button class="btn btn-primary">button</button></div></div></div>
</div>
```
# Templates

The template system runs on handlebars but also has some static rows. The ordering of the rows is important as each type of template maps to a specific line. Spacing is arbitrary and only for readability. Templates do not span more than one line.

A bootstrap4 example is below:

```html
<div class="container">
    <div class="row">
        <div class="col">
            <div>
            </div>
        </div>
    </div>
</div>
<label>{{value}}</label>
<input type="text" class="form-control" value="{{value}}"/>
<label class="form-check-label"><input type="checkbox" class="form-check-input"/>{{value}}</label>
<label class="form-check-label"><input type="radio" class="form-check-input"/>{{value}}</label>
<textarea class="form-control">{{value}}</textarea>
<button class="btn btn-primary">{{value}}</button>
<select class="form-control">{{#each value}}<option>{{this}}</option>{{/each}}</select>
<hr/>
```

You can also use the environment variable **template** to pass in a template. In this form the template should be delimitted by a comma rather than a newline

# Full example

A complete feature-set of supported features is below:

```
<h1>Welcome to my form!</h1>

| {What is your name?} | {Fill out options}
| [Enter a name]       | [o Yes, please] [o No, thanks]
|                      | [/ Check One] [/ Check Two]

| {Select an option}
| <Select an option..., option 1, option 2, option 3>

| {Enter some text}
| [+ How bout a textarea?]

---

| (Submit)

---

A row with a | and < and { to make sure system ignores it

| I can have a lot of text | there's some more here we | if I want | to
| in a particular column   | could get into            | if I want | to
| if I wanted to           | but we won't              | if I want | to

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi porta tincidunt diam vel imperdiet. Pellentesque tincidunt urna ac lacus semper, dapibus rutrum sem faucibus. Donec efficitur in nunc vel congue. Fusce in bibendum lacus. Vivamus nec enim metus. Maecenas facilisis pellentesque dictum. Donec porta nisl sapien, ut lobortis risus fermentum id. Maecenas egestas posuere orci, non porttitor libero porta ut. Suspendisse velit ligula, accumsan non nisi vel, scelerisque mollis libero. Sed justo enim, pretium sit amet sollicitudin sed, vulputate ac lorem.
```

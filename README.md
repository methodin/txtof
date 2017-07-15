# txtof
Simple text-based form generator for prototyping

# Usage

txtof accepts standard input (e.g. via | or <) as well as an optional parameter for a template path

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

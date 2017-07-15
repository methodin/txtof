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


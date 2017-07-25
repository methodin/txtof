# Text is king
There are a lot of tools in existence that make prototyping easy but they all center around a GUI for creation and I wanted to make something based on text instead. Text allows you to run diffs, store anywhere and create a core set of symbols that can express many UI themes.

# Usage
This file is best used by either a tool like (CodePen)[http://codepen.io] or a (bookmarket)[http://www.fillmein.com]. 

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

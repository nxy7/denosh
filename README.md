# Deno Shell

Deno Shell is a experimental TypeScript shell that's very thin layer on top of Deno.
DenoSH scripts is superset of TypeScript so any typescript code should work just fine.

# Features

- [ ] first class executables
- [ ] piping function output into variables

# Areas to explore

- structured output
- concurrency (execute binaries with something like `await Promise.all([$(echo one), $(echo two)])`)
- queue command while another command is running

# Non goals

- performance (who cares - it's shell)

# Syntax rules

- ?

# Examples (not implemented)

### Saving output of command

```js
let x = $(echo test message);
console.log(x); // outputs `test message`
```

### Parallelize execution

```js
const results = await Promise.all([$(sleep 10), $(sleep 15), $(sleep 20)]);
// Will sleep for 20sec
```

### JSON support

```js
let z = $(curl http://some-json.com/json);
console.log(z.nested.field); // returned json was changed into json without user action
```

### String interpoloration

```js
let z = "Some string";
echo ${z}

let url = "http://some-json.com/json";
curl ${url} | echo
```

### Error codes?

I'd like something like this to work

```js
fallibleCommand;
// if command fails, error will be printed out
```

```js
try {
  fallibleCommand;
} catch (e) {
  // if command fails, user has access to error inside `e` variable
  // e: {code: 123, text: "error text"}
}
```

# Contributing

Contributions are welcome! Please open an issue or submit a pull request.
License

This project is licensed under the MIT License.

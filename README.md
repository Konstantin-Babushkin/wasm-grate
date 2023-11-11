# WASM-grate
WASM-grate (a portmanteau of WebAssembly and "integrate/migrate") is a tool designed to streamline the adoption of WebAssembly in modern front-end web projects.

It helps you to identify potential bottlenecks or performance hitches in JavaScript (JS) or TypeScript (TS) codebases. WASM-grate highlights areas that could significantly benefit from the speed and efficiency of WebAssembly (e.g. CPU-intensive tasks).

## Installation
```bash
npm i wasm-grate
```

## Usage
```bash
wasm-grate -path '<path to a file or directory>'
```
or
```bash
wasm-grate -p '<path>'
```


Examples:
```bash
wasm-grate --path src/pages/Search
```

```bash
wasm-grate -p src/components/Report/Feed/helpers.ts
```

## Output
```bash
# LOCATION
src/components/ProjectReport/ProjectFeed/Contribution/helpers/process-pr-data.ts:14:19

# COMPLEXITY SCORE
Complexity: 4/10

# DECLARATION OF THE FUNCTION
Declaration: const getScale = (totalChanges: number | null): number
```

## Deploy to NPM and crates
```bash
rust-to-npm-cli deploy -b
```

## Current state
WASM-grate is still in development and not production-ready yet.

For now it works only for: 
- function declarations
```Javascript
function foo() {
    console.log('function declaration');
} 
```
- expressions 
```Javascript
const bar = function() {
    console.log('function expression');
}
```
- arrow functions
```Javascript
const foo = () => {
    console.log('arrow function');
} 
```

**Please, wait for version 1.0.0**  

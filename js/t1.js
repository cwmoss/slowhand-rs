import { parse } from 'kdljs'
import fs from 'node:fs';

console.log("parser", parse);

let doc = parse(`
// Nodes can be separated into multiple lines
title \
  "Some title"
`)


console.log("result", doc)

let data = fs.readFileSync('schema.kdl', 'utf8');
doc = parse(data)
console.log("schema result", doc.output)
console.log("schema errors", doc.errors)
console.log(JSON.stringify(doc.output, null, 3))
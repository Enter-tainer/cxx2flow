import path from "node:path";
import { pathToFileURL } from "node:url";

const moduleUrl = pathToFileURL(path.resolve("pkg", "cxx2flow.js")).href;
const wasmPkg = await import(moduleUrl);
const init = wasmPkg.default;
const { generate_dot } = wasmPkg;

const code = `
int main() {
  int x = 0;
  if (x) {
    return 1;
  }
  return 0;
}
`;

await init();
const output = generate_dot(code, "main", false);

if (!output.includes("digraph") || !output.includes("begin") || !output.includes("end")) {
  throw new Error("wasm smoke test failed: unexpected output");
}

console.log("wasm smoke test passed");

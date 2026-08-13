import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const sourcePath = new URL("../src/ProceduralMemorySeed.tsx", import.meta.url);
const source = readFileSync(sourcePath, "utf8");
const svgStart = source.indexOf("<svg");
const svgEnd = source.indexOf("</svg>", svgStart);

if (svgStart < 0 || svgEnd < 0) {
  throw new Error("ProceduralMemorySeed SVG block was not found.");
}

let svg = source.slice(svgStart, svgEnd + "</svg>".length);
const replacements = new Map([
  ["bodyGradient", "body"],
  ["leafGradient", "leaf"],
  ["petalGradient", "petal"],
  ["eyeGradient", "eye"],
  ["coreGradient", "core"],
  ["bodyGlow", "body-glow"],
  ["coreGlow", "core-glow"],
  ["plateShadow", "plate-shadow"],
]);

svg = svg
  .replace(/\s+(?:aria-hidden|focusable|role|data-[\w-]+)=(?:"[^"]*"|\{[^}]*\})/g, "")
  .replaceAll("className=", "class=")
  .replace(/([\w-]+)=\{([0-9.]+)\}/g, '$1="$2"')
  .replace(/([\w-]+)=\{`([^`]*)`\}/g, '$1="$2"')
  .replaceAll("floodColor=", "flood-color=")
  .replaceAll("floodOpacity=", "flood-opacity=")
  .replaceAll("stopColor=", "stop-color=")
  .replaceAll("strokeLinecap=", "stroke-linecap=")
  .replaceAll("strokeWidth=", "stroke-width=")
  .replaceAll("strokeOpacity=", "stroke-opacity=");

for (const [expression, id] of replacements) {
  svg = svg
    .replaceAll(`id={${expression}}`, `id="${id}"`)
    .replaceAll(`{\`url(#\${${expression}})\`}`, `"url(#${id})"`)
    .replaceAll(`url(#\${${expression}})`, `url(#${id})`);
}

const unresolvedExpression = svg.match(/\{[^}]*\}|`/);
if (unresolvedExpression) {
  console.error(svg.slice(Math.max(0, unresolvedExpression.index - 80), unresolvedExpression.index + 160));
  throw new Error("Unresolved JSX expression remains in the extracted SVG.");
}

const html = `<!doctype html><meta charset="utf-8"><style>
html,body{margin:0;width:320px;height:320px;overflow:hidden;background:transparent}
body{display:grid;place-items:center}
svg{width:280px;height:292px;overflow:visible}
</style>${svg}`;

const outputPath = process.argv[2] || join(tmpdir(), "memoryling-procedural-seed.png");
const tempRoot = join(tmpdir(), "memoryling-offscreen-render");
const htmlPath = join(tempRoot, "seed.html");
const profilePath = join(tempRoot, "edge-profile");
mkdirSync(profilePath, { recursive: true });
writeFileSync(htmlPath, html, "utf8");

const edgeCandidates = [
  "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe",
  "C:\\Program Files\\Microsoft\\Edge\\Application\\msedge.exe",
];
const edgePath = edgeCandidates.find(existsSync);
if (!edgePath) throw new Error("Microsoft Edge was not found for headless rendering.");

execFileSync(
  edgePath,
  [
    "--headless=new",
    "--disable-gpu",
    "--hide-scrollbars",
    "--default-background-color=00000000",
    `--user-data-dir=${profilePath}`,
    "--window-size=320,320",
    `--screenshot=${outputPath}`,
    new URL(`file:///${htmlPath.replaceAll("\\", "/")}`).href,
  ],
  { stdio: "ignore", windowsHide: true },
);

console.log(outputPath);

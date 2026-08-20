#!/usr/bin/env bun

import { $, YAML } from "bun";

// ファイル名を指定して YAML を読み込んでスクリプトを実行する
// 例えば引数が tessoku/tessoku-book/src/a01.py だった場合は
// tessoku/tessoku-book/testcases/a01.yml を読み込む
// yaml は以下のようになっている
// ```yaml
// cases:
//   - name: sample1
//     in: |
//       2
//     out: |
//       4
// ```
// python3 tessoku/tessoku-book/src/a01.py に対して、標準入力に 2 を与えたときの標準出力が 4 であることを確認する
//

const scriptPath = process.argv[2];
if (!scriptPath) {
  console.error("Usage: bun run script.ts <script.py>");
  process.exit(1);
}

const testCasePath = scriptPath
  .replace(/src/, "testcases")
  .replace(/\.py$/, ".yml");
const data = YAML.parse(await Bun.file(testCasePath).text()) as any;

for (const testCase of data.cases) {
  const input = testCase.in;
  const expectedOutput = testCase.out;
  const proc = $`echo "${input}" | python3 ${scriptPath}`;
  const output = await proc.text();
  if (output === expectedOutput) {
    console.log(`Test case ${testCase.name} passed`);
  } else {
    console.error(`Test case ${testCase.name} failed`);
    console.error(`Input:\n${input}`);
    console.error(`Expected output:\n${expectedOutput}`);
    console.error(`Actual output:\n${output}`);
  }
}

function removeKatexTag() {
  document.querySelectorAll('.katex-mathml').forEach((e) => e.remove());
}
function cleanText(text) {
  return text?.replace(/\u200b/g, '')?.replace(/\=/g, '≠');
}

function scrapeData() {
  let langContainer = document.querySelector('.lang-en');

  if (!langContainer) {
    return 'Error: Language container not found!';
  }

  // KaTeXタグを削除
  removeKatexTag();

  let data = { problemStatement: '' };

  // 問題文の取得
  let problemStatementSection = Array.from(
    langContainer.querySelectorAll('.part > section > h3')
  ).find((h3) => h3.textContent.trim() === 'Problem Statement');

  let problemParagraph = problemStatementSection.nextElementSibling;
  while (['P', 'UL'].includes(problemParagraph?.tagName)) {
    data.problemStatement += cleanText(problemParagraph.textContent.trim());
    problemParagraph = problemParagraph.nextElementSibling;
  }

  // 条件の取得
  data.constraints = [];
  let constraintsSection = Array.from(
    langContainer.querySelectorAll('.part > section > h3')
  ).find((h3) => h3.textContent.trim() === 'Constraints');
  if (constraintsSection) {
    let constraintsList =
      constraintsSection.nextElementSibling.querySelectorAll('li');
    constraintsList.forEach((item) => {
      data.constraints.push(cleanText(item.textContent.trim()));
    });
  }

  // 入力の取得
  let inputSection = Array.from(
    langContainer.querySelectorAll('.io-style .part > section > h3')
  ).find((h3) => h3.textContent.trim() === 'Input');
  data.input = {
    description: inputSection
      ? cleanText(inputSection.nextElementSibling.textContent.trim())
      : 'Not found',
    detail: inputSection
      ? cleanText(
          inputSection.nextElementSibling.nextElementSibling.textContent.trim()
        )
      : 'Not found',
  };

  // 出力の取得
  let outputSection = Array.from(
    langContainer.querySelectorAll('.io-style .part > section > h3')
  ).find((h3) => h3.textContent.trim() === 'Output');
  data.output = {
    description: outputSection
      ? cleanText(outputSection.nextElementSibling.textContent.trim())
      : 'Not found',
    detail: outputSection
      ? cleanText(
          outputSection.nextElementSibling?.nextElementSibling?.textContent?.trim()
        )
      : 'Not found',
  };

  // サンプルの入力と出力を取得
  data.samples = [];
  let inputSampleSections = Array.from(
    langContainer.querySelectorAll('.part section h3')
  ).filter((h3) => h3.textContent.trim().startsWith('Sample Input'));
  let outputSampleSections = Array.from(
    langContainer.querySelectorAll('.part section h3')
  ).filter((h3) => h3.textContent.trim().startsWith('Sample Output'));

  inputSampleSections.forEach((inputSection, index) => {
    let inputPre = inputSection.parentElement.querySelector('pre');
    let outputPre =
      outputSampleSections[index].parentElement.querySelector('pre');

    let inputNextSibling = inputPre.nextElementSibling;
    let inputDetail = '';
    while (inputNextSibling) {
      inputDetail += cleanText(inputNextSibling.textContent.trim()) + '\n';
      inputNextSibling = inputNextSibling.nextElementSibling;
    }

    let outputNextSibling = outputPre.nextElementSibling;
    let outputDetail = '';
    while (outputNextSibling) {
      outputDetail += cleanText(outputNextSibling.textContent.trim()) + '\n';
      outputNextSibling = outputNextSibling.nextElementSibling;
    }

    let sampleData = {
      input: inputPre ? cleanText(inputPre.textContent.trim()) : 'Not found',
      inputDetail,
      output: outputPre ? cleanText(outputPre.textContent.trim()) : 'Not found',
      outputDetail,
    };
    data.samples.push(sampleData);
  });

  return data;
}

function toMarkdown(data) {
  let markdown = '';

  markdown += `# Problem Statement\n\n${data.problemStatement}\n\n`;

  markdown += `## Constraints\n\n`;
  data.constraints.forEach((constraint) => {
    markdown += `- ${constraint}\n`;
  });

  markdown += `\n## Input\n\n${data.input.description}\n\n`;
  markdown += '```\n' + data.input.detail + '\n```\n';

  markdown += `\n## Output\n\n${data.output.description}\n\n`;
  if (data.output.detail) {
    markdown += '```\n' + data.output.detail + '\n```\n';
  }

  data.samples.forEach((sample, i) => {
    markdown += `### Sample Input ${i + 1}\n\n`;
    markdown += '```\n' + sample.input + '\n```\n';
    markdown += sample.inputDetail + '\n';
    markdown += `### Sample Output ${i + 1}\n\n`;
    markdown += '```\n' + sample.output + '\n```\n';
    markdown += sample.outputDetail + '\n';
  });

  return markdown;
}

let data = scrapeData();
let markdownText = toMarkdown(data);
console.log(markdownText);
copy(markdownText);

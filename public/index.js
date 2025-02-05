import init, { is_allowed, validate } from "./robots_txt_wasm.js";
const $form = document.forms['form'];
const $badge = document.querySelector('[data-status]');
const $result = document.querySelector('#result');

const updateUI = (isAllowed, $report) => {
  if (isAllowed === undefined) {
    $badge.dataset.status === "";
    while ($result.firstChild) {
      $result.removeChild($result.lastChild);
    }
    return;
  }
  $badge.dataset.status = isAllowed ? "allowed" : "disallowed";
  $result.appendChild($report);
};

const getStatsPerLine = (body) => {
  const report = validate(body);
  const lines = body.split("\n");
  const valid_lines = Array.from(report.valid_lines);
  const invalid_lines = Array.from(report.invalid_lines);
  return lines.map((line, index) => {
    const lineWithoutComment = line.split("#")[0];
    const lineNumber = index + 1;
    const isValid = valid_lines.includes(lineNumber);
    const isInvalid = invalid_lines.includes(lineNumber) || !!lineWithoutComment;
    return {
      valid: isValid || !isInvalid,
      line,
    };
  });
};

const createReportHtml = (stats) => {
  const $ = document.createDocumentFragment();

  stats.forEach((item) => {
    const $div = document.createElement('div');
    $div.innerText = item.line || '';
    if (!item.valid) {
      $div.classList.add('invalid');
    }
    $.appendChild($div);
  });

  return $;
};

const main = async () => {
  await init();
  $form.onsubmit = (e) => {
    e.preventDefault();
    // reset
    updateUI();

    const data = new FormData($form);
    const { body, useragent, url } = Object.fromEntries(data.entries());

    // validate
    const stats = getStatsPerLine(body);
    const $report = createReportHtml(stats);

    const isAllowed = is_allowed(body, useragent, url);
    updateUI(isAllowed, $report);
  };
};

main();


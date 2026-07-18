import { access, readFile } from 'node:fs/promises';
import { dirname, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const requirementsDocument = await readFile(
  resolve(root, 'FUNCTIONAL_REQUIREMENTS.md'),
  'utf8',
);
const manifest = JSON.parse(
  await readFile(resolve(root, 'traceability.json'), 'utf8'),
);

const requirementIds = [
  ...requirementsDocument.matchAll(/^\|\s*(FR-[A-Z]+-\d{3})\s*\|/gm),
].map((match) => match[1]);
const errors = [];
const mappingsById = new Map();

for (const mapping of manifest.requirements) {
  if (mappingsById.has(mapping.id)) {
    errors.push(`duplicate mapping: ${mapping.id}`);
  }
  mappingsById.set(mapping.id, mapping);
}

for (const id of requirementIds) {
  const mapping = mappingsById.get(id);
  if (!mapping) {
    errors.push(`unmapped requirement: ${id}`);
    continue;
  }

  for (const field of ['source', 'test']) {
    try {
      await access(resolve(root, mapping[field]));
    } catch {
      errors.push(`${id} references missing ${field}: ${mapping[field]}`);
    }
  }

  try {
    const testContent = await readFile(resolve(root, mapping.test), 'utf8');
    if (!testContent.includes(mapping.case)) {
      errors.push(`${id} references missing test case: ${mapping.case}`);
    }
  } catch {
    // The missing-file error above is more actionable.
  }
}

for (const id of mappingsById.keys()) {
  if (!requirementIds.includes(id)) {
    errors.push(`stale mapping without active requirement: ${id}`);
  }
}

const mapped = requirementIds.filter((id) => mappingsById.has(id)).length;
const e2e = requirementIds.filter((id) => {
  const test = mappingsById.get(id)?.test;
  return test?.split(/[\\/]/).join(sep).startsWith(`tests${sep}e2e${sep}`);
}).length;
const requirementCoverage = (mapped / requirementIds.length) * 100;
const e2eCoverage = (e2e / requirementIds.length) * 100;
const minimum = manifest.minimumCoveragePercent;

if (requirementCoverage < minimum) {
  errors.push(
    `requirement-to-test coverage ${requirementCoverage.toFixed(1)}% is below ${minimum}%`,
  );
}
if (e2eCoverage < minimum) {
  errors.push(`E2E coverage ${e2eCoverage.toFixed(1)}% is below ${minimum}%`);
}

console.log(
  `Traceability: ${mapped}/${requirementIds.length} (${requirementCoverage.toFixed(1)}%)`,
);
console.log(
  `E2E requirements: ${e2e}/${requirementIds.length} (${e2eCoverage.toFixed(1)}%)`,
);

if (errors.length > 0) {
  for (const error of errors) console.error(`ERROR: ${error}`);
  process.exitCode = 1;
}

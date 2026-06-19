/**
 * E2E smoke test for @phenotype/config-ts.
 *
 * Exercises the consumer's full user journey against the built package:
 *   1. Import from the built artifact (dist/index.mjs) — proves the package
 *      is publishable and the public exports resolve correctly.
 *   2. Construct an EnvConfigSource — the primary runtime source.
 *   3. Read entries via load() and a value via get().
 *   4. Validate returned values with the exported Zod schema.
 *
 * NOTE: Conft is a library with no UI or server. Web-app E2E (Playwright) is
 * N/A here — see FLEET-AUDIT-30-PILLAR.md T3 (UI pillars) for the reason
 * (N/A = 3 for library / CLI repos). This "consumer journey" is the
 * library-equivalent: a downstream package would import from dist/, build
 * a source, read values, and validate. That import-and-validate flow is
 * the T3 surface for a library.
 */

import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { writeFile, unlink, mkdtemp } from 'fs/promises';
import { tmpdir } from 'os';
import { join } from 'path';
import {
  EnvConfigSource,
  FileConfigSource,
  ConfigValueSchema,
  type ConfigValue,
} from '../../dist/index.mjs';

describe('@phenotype/config-ts — E2E consumer journey', () => {
  let tmpDir: string;
  let tmpFile: string;

  beforeAll(async () => {
    tmpDir = await mkdtemp(join(tmpdir(), 'conft-e2e-'));
    tmpFile = join(tmpDir, 'config.json');
    await writeFile(
      tmpFile,
      JSON.stringify({ feature: 'on', retries: '3', nested: { a: '1' } }),
      'utf-8',
    );
  });

  afterAll(async () => {
    await unlink(tmpFile).catch(() => undefined);
  });

  it('exports the public API surface from the built artifact', () => {
    expect(EnvConfigSource).toBeDefined();
    expect(FileConfigSource).toBeDefined();
    expect(ConfigValueSchema).toBeDefined();
  });

  it('EnvConfigSource.load() returns prefixed entries with stripped, lowercased keys', async () => {
    process.env.APP_FOO = 'bar';
    process.env.APP_BAZ = 'qux';
    process.env.UNRELATED = 'skip';
    try {
      const source = new EnvConfigSource('APP_');
      const entries = await source.load();
      const keys = entries.map((e) => e.key).sort();
      expect(keys).toEqual(['baz', 'foo']);
      const foo = entries.find((e) => e.key === 'foo');
      expect(foo?.value).toBe('bar');
      expect(foo?.source).toBe('env');
    } finally {
      delete process.env.APP_FOO;
      delete process.env.APP_BAZ;
      delete process.env.UNRELATED;
    }
  });

  it('EnvConfigSource.get() returns the value at prefix+key (case-sensitive)', async () => {
    process.env.APP_FEATURE = 'on';
    try {
      const source = new EnvConfigSource('APP_');
      const value = await source.get('FEATURE');
      expect(value).toBe('on');
      const parsed: ConfigValue = ConfigValueSchema.parse(value);
      expect(parsed).toBe('on');
    } finally {
      delete process.env.APP_FEATURE;
    }
  });

  it('EnvConfigSource.get() parses numeric strings to numbers', async () => {
    process.env.APP_RETRIES = '5';
    try {
      const source = new EnvConfigSource('APP_');
      const value = await source.get('RETRIES');
      expect(value).toBe(5);
      const parsed: ConfigValue = ConfigValueSchema.parse(value);
      expect(parsed).toBe(5);
    } finally {
      delete process.env.APP_RETRIES;
    }
  });

  it('EnvConfigSource.get() parses boolean strings to booleans', async () => {
    process.env.APP_DEBUG = 'true';
    try {
      const source = new EnvConfigSource('APP_');
      const value = await source.get('DEBUG');
      expect(value).toBe(true);
      const parsed: ConfigValue = ConfigValueSchema.parse(value);
      expect(parsed).toBe(true);
    } finally {
      delete process.env.APP_DEBUG;
    }
  });

  it('FileConfigSource reads values from a JSON file end-to-end', async () => {
    const source = new FileConfigSource(tmpFile);
    const feature = await source.get('feature');
    const retries = await source.get('retries');
    expect(feature).toBe('on');
    expect(retries).toBe('3');
  });

  it('FileConfigSource.load() returns all entries with the file source tag', async () => {
    const source = new FileConfigSource(tmpFile);
    const entries = await source.load();
    const keys = entries.map((e) => e.key).sort();
    expect(keys).toEqual(['feature', 'nested', 'retries']);
    for (const entry of entries) {
      expect(entry.source).toBe('file');
      expect(typeof entry.timestamp).toBe('number');
    }
  });

  it('ConfigValueSchema accepts the canonical scalar and array shapes', () => {
    expect(() => ConfigValueSchema.parse('hello')).not.toThrow();
    expect(() => ConfigValueSchema.parse(42)).not.toThrow();
    expect(() => ConfigValueSchema.parse(true)).not.toThrow();
    expect(() => ConfigValueSchema.parse(['a', 'b'])).not.toThrow();
    expect(() => ConfigValueSchema.parse({ k1: 'v1', k2: 'v2' })).not.toThrow();
  });
});

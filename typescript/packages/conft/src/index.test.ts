import { describe, it, expect } from 'vitest';
import * as Config from './index';

describe('@phenotype/config-ts entry point', () => {
  it('re-exports the public API', () => {
    expect(Config).toBeDefined();
  });
});

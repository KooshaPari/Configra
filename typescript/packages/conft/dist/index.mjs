// src/domain/config.ts
import { z } from "zod";
var ConfigValueSchema = z.union([
  z.string(),
  z.number(),
  z.boolean(),
  z.array(z.string()),
  z.record(z.string(), z.string())
]);
var ConfigEntrySchema = z.object({
  key: z.string(),
  value: ConfigValueSchema,
  source: z.string().optional(),
  timestamp: z.number().optional()
});
var ConfigSnapshotSchema = z.object({
  entries: z.record(ConfigValueSchema),
  sources: z.array(z.string()),
  version: z.string(),
  timestamp: z.number()
});
var ConfigErrorSchema = z.object({
  message: z.string(),
  path: z.array(z.string()),
  value: z.unknown(),
  context: z.record(z.unknown()).optional()
});
var ConfigValidationError = class extends Error {
  constructor(message, path, value, context) {
    super(message);
    this.path = path;
    this.value = value;
    this.context = context;
    this.name = "ConfigValidationError";
  }
  toJSON() {
    return {
      message: this.message,
      path: this.path,
      value: this.value,
      context: this.context
    };
  }
};
var ConfigSourceNotFoundError = class extends Error {
  constructor(message, source, context) {
    super(message);
    this.source = source;
    this.context = context;
    this.name = "ConfigSourceNotFoundError";
  }
};
var ImmutableConfig = class {
  constructor(entries, sources, version) {
    this.version = version;
    this.entries = new Map(entries);
    this.sources = [...sources];
  }
  get(key) {
    return this.entries.get(key);
  }
  has(key) {
    return this.entries.has(key);
  }
  toSnapshot() {
    return {
      entries: Object.fromEntries(this.entries),
      sources: [...this.sources],
      version: this.version,
      timestamp: Date.now()
    };
  }
};

// src/adapters/env-adapter.ts
var EnvConfigSource = class {
  constructor(prefix = "APP_") {
    this.name = "env";
    this.prefix = prefix;
  }
  async load() {
    const entries = [];
    for (const [key, value] of Object.entries(process.env)) {
      if (key.startsWith(this.prefix) && value !== void 0) {
        entries.push({
          key: this.stripPrefix(key),
          value: this.parseValue(value),
          source: this.name,
          timestamp: Date.now()
        });
      }
    }
    return entries;
  }
  async get(key) {
    const fullKey = this.prefix + key;
    const value = process.env[fullKey];
    if (value === void 0) return void 0;
    return this.parseValue(value);
  }
  async set(key, value) {
    throw new Error("Environment variables are read-only");
  }
  isWritable() {
    return false;
  }
  stripPrefix(key) {
    return key.slice(this.prefix.length).toLowerCase();
  }
  parseValue(value) {
    try {
      const parsed = JSON.parse(value);
      const result = ConfigValueSchema.safeParse(parsed);
      if (result.success) {
        return result.data;
      }
    } catch {
    }
    if (value.toLowerCase() === "true") return true;
    if (value.toLowerCase() === "false") return false;
    const number = Number(value);
    if (value.trim() !== "" && Number.isFinite(number)) return number;
    return value;
  }
};

// src/adapters/file-adapter.ts
import { readFile } from "fs/promises";
var FileConfigSource = class {
  constructor(path) {
    this.name = "file";
    this.path = path;
  }
  async load() {
    const content = await readFile(this.path, "utf-8");
    const parsed = JSON.parse(content);
    const entries = [];
    for (const [key, value] of Object.entries(parsed)) {
      entries.push({
        key,
        value,
        source: this.name,
        timestamp: Date.now()
      });
    }
    return entries;
  }
  async get(key) {
    const entries = await this.load();
    const entry = entries.find((e) => e.key === key);
    return entry?.value;
  }
  async set(_key, _value) {
    throw new Error("File configuration sources are read-only");
  }
  isWritable() {
    return false;
  }
};
export {
  ConfigEntrySchema,
  ConfigErrorSchema,
  ConfigSnapshotSchema,
  ConfigSourceNotFoundError,
  ConfigValidationError,
  ConfigValueSchema,
  EnvConfigSource,
  FileConfigSource,
  ImmutableConfig
};

"use strict";
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// src/index.ts
var index_exports = {};
__export(index_exports, {
  ConfigEntrySchema: () => ConfigEntrySchema,
  ConfigErrorSchema: () => ConfigErrorSchema,
  ConfigSnapshotSchema: () => ConfigSnapshotSchema,
  ConfigSourceNotFoundError: () => ConfigSourceNotFoundError,
  ConfigValidationError: () => ConfigValidationError,
  ConfigValueSchema: () => ConfigValueSchema,
  EnvConfigSource: () => EnvConfigSource,
  FileConfigSource: () => FileConfigSource,
  ImmutableConfig: () => ImmutableConfig
});
module.exports = __toCommonJS(index_exports);

// src/domain/config.ts
var import_zod = require("zod");
var ConfigValueSchema = import_zod.z.union([
  import_zod.z.string(),
  import_zod.z.number(),
  import_zod.z.boolean(),
  import_zod.z.array(import_zod.z.string()),
  import_zod.z.record(import_zod.z.string(), import_zod.z.string())
]);
var ConfigEntrySchema = import_zod.z.object({
  key: import_zod.z.string(),
  value: ConfigValueSchema,
  source: import_zod.z.string().optional(),
  timestamp: import_zod.z.number().optional()
});
var ConfigSnapshotSchema = import_zod.z.object({
  entries: import_zod.z.record(ConfigValueSchema),
  sources: import_zod.z.array(import_zod.z.string()),
  version: import_zod.z.string(),
  timestamp: import_zod.z.number()
});
var ConfigErrorSchema = import_zod.z.object({
  message: import_zod.z.string(),
  path: import_zod.z.array(import_zod.z.string()),
  value: import_zod.z.unknown(),
  context: import_zod.z.record(import_zod.z.unknown()).optional()
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
var import_promises = require("fs/promises");
var FileConfigSource = class {
  constructor(path) {
    this.name = "file";
    this.path = path;
  }
  async load() {
    const content = await (0, import_promises.readFile)(this.path, "utf-8");
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
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  ConfigEntrySchema,
  ConfigErrorSchema,
  ConfigSnapshotSchema,
  ConfigSourceNotFoundError,
  ConfigValidationError,
  ConfigValueSchema,
  EnvConfigSource,
  FileConfigSource,
  ImmutableConfig
});

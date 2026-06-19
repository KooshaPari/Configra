# Conft — SPEC.md

## Document Information

| Field | Value |
|-------|-------|
| **Project** | Conft (phenotype-config-ts) |
| **Version** | 1.0.0 |
| **Status** | Draft |
| **Date** | 2026-04-04 |
| **Author** | Conft Core Team |

---

## Table of Contents

1. [Overview](#overview)
2. [Mission and Tenets](#mission-and-tenets)
3. [Architecture](#architecture)
4. [Components](#components)
5. [Data Models](#data-models)
6. [API Specification](#api-specification)
7. [Adapters](#adapters)
8. [Validation](#validation)
9. [Error Handling](#error-handling)
10. [Performance Requirements](#performance-requirements)
11. [Security Considerations](#security-considerations)
12. [Testing Strategy](#testing-strategy)
13. [Implementation Details](#implementation-details)
14. [Usage Examples](#usage-examples)
15. [Project Structure](#project-structure)
16. [Migration Guide](#migration-guide)
17. [Future Work](#future-work)
18. [References](#references)
19. [Appendix A: Glossary](#appendix-a-glossary)
20. [Appendix B: Changelog](#appendix-b-changelog)

---

## Overview

Conft (phenotype-config-ts) is a TypeScript configuration management library designed for production-grade applications. It provides type-safe configuration loading with Zod validation, hexagonal architecture with `ConfigSource` ports, and adapters for file and environment sources.

### Key Features

- **Type-safe configuration**: Full TypeScript type inference from Zod schemas
- **Hexagonal architecture**: Domain logic isolated from infrastructure concerns
- **Multiple sources**: File (JSON/YAML), environment variables, memory, and composite adapters
- **Layered configuration**: Deep merge semantics for environment-specific overrides
- **Async-first API**: Non-blocking configuration loading
- **Production ready**: Comprehensive error handling and validation

### Target Use Cases

1. Microservices requiring environment-specific configuration
2. Serverless functions with typed configuration
3. CLI tools with complex configuration needs
4. Testing scenarios with mockable configuration sources
5. Monorepos with shared configuration schemas

### Non-Goals

Conft explicitly does NOT:
- Replace infrastructure configuration tools (Terraform, CloudFormation)
- Provide a configuration UI or editor
- Manage feature flags (use specialized services)
- Replace secret management systems (HashiCorp Vault, AWS Secrets Manager)
- Support dynamic languages other than TypeScript/JavaScript

---

## Mission and Tenets

### Mission

Conft provides type-safe, validated configuration management for TypeScript applications with zero-configuration defaults, hexagonal architecture compliance, and production-grade reliability.

### Tenets

1. **Type Safety First**: All configuration must be fully typed. Runtime validation must produce TypeScript-compatible types. No `any` types in public APIs.

2. **Fail Fast, Fail Loud**: Invalid configuration must be caught at application startup, not at runtime. Error messages must identify the exact failing field, the expected type, and the actual value received.

3. **Configuration as Code**: Configuration schemas are code, not comments. They live in version control, are reviewed in pull requests, and are tested in CI.

4. **Layered Sources**: Configuration comes from multiple sources with predictable priority: defaults < files < environment < command line.

5. **Hexagonal by Design**: The domain knows nothing about files, environment variables, or network calls. Adapters handle external concerns.

6. **Zero Configuration Defaults**: Sensible defaults must work out of the box. A new user should be productive in minutes.

7. **Performance at Scale**: Configuration loading must not block the event loop. File watching must use efficient OS primitives.

8. **Observable Behavior**: All configuration loading, reloading, and validation must be observable via events and hooks.

---

## Architecture

### Layered Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              Application Layer                                   │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │                          ConfigService<T>                                  │  │
│  │  ┌──────────────────────────────────────────────────────────────────────┐ │  │
│  │  │  load(): Promise<ConfigResult<T>>                                   │ │  │
│  │  │  reload(): Promise<ConfigResult<T>>                                │ │  │
│  │  │  get(): T                                                           │ │  │
│  │  │  getSafe(): T | undefined                                           │ │  │
│  │  │  onChange(callback): void                                          │ │  │
│  │  │  onError(callback): void                                           │ │  │
│  │  └──────────────────────────────────────────────────────────────────────┘ │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                      │                                           │
│                                      │ uses                                       │
│                                      ▼                                           │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │                             Domain Layer                                   │  │
│  │  ┌──────────────────────────────────────────────────────────────────────┐ │  │
│  │  │                        Config<T>                                       │ │  │
│  │  │  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐          │ │  │
│  │  │  │   value: T     │  │   source:      │  │  version:      │          │ │  │
│  │  │  │                │  │   ConfigSource │  │  string        │          │ │  │
│  │  │  └────────────────┘  └────────────────┘  └────────────────┘          │ │  │
│  │  └──────────────────────────────────────────────────────────────────────┘ │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                      │                                           │
│                                      │ depends on                                 │
│                                      ▼                                           │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │                              Port Layer                                      │  │
│  │  ┌──────────────────────────────────────────────────────────────────────┐ │  │
│  │  │                      ConfigSource (Interface)                          │ │  │
│  │  │  ┌──────────────────────────────────────────────────────────────────┐ │ │  │
│  │  │  │  readonly name: string                                          │ │ │  │
│  │  │  │  load(): Promise<ConfigEntry[]>                                   │ │ │  │
│  │  │  │  get(key: string): Promise<ConfigValue | undefined>              │ │ │  │
│  │  │  │  set(key: string, value: ConfigValue): Promise<void>              │ │ │  │
│  │  │  │  isWritable(): boolean                                            │ │ │  │
│  │  │  └──────────────────────────────────────────────────────────────────┘ │ │  │
│  │  └──────────────────────────────────────────────────────────────────────┘ │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                      │                                           │
│                                      │ implements                                 │
│                                      ▼                                           │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │                            Adapter Layer                                     │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌────────────────┐   │  │
│  │  │   File       │  │     Env      │  │   Memory     │  │   Composite    │   │  │
│  │  │   Adapter    │  │   Adapter    │  │   Adapter    │  │   Adapter      │   │  │
│  │  │              │  │              │  │              │  │                │   │  │
│  │  │ • JSON       │  │ • process.env│  │ • In-memory  │  │ • Multi-source │   │  │
│  │  │ • YAML       │  │ • Prefix     │  │ • Testing    │  │ • Priority     │   │  │
│  │  │ • TOML       │  │ • Type coerce│  │ • Defaults   │  │ • Deep merge   │   │  │
│  │  └──────────────┘  └──────────────┘  └──────────────┘  └────────────────┘   │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Hexagonal Architecture Principles

1. **Dependency Inversion**: Domain depends on `ConfigSource` port, not specific adapters
2. **Interface Segregation**: `ConfigSource` is a focused, single-purpose interface
3. **Testability**: Domain logic can be tested with `MemoryAdapter` without filesystem mocking

### Configuration Loading Pipeline

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Sources   │───▶│   Parsing   │───▶│  Validation │───▶│   Domain    │
│             │    │             │    │             │    │             │
│ • Files     │    │ • JSON      │    │ • Zod       │    │ • Typed     │
│ • Env vars  │    │ • YAML      │    │   Schema    │    │   Config    │
│ • Memory    │    │ • TOML      │    │ • Type      │    │ • Immutable │
│ • Remote    │    │ • Coercion  │    │   Inference │    │ • Observable│
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
      │                   │                   │                   │
      │              ┌────┴────┐              │              ┌────┴────┐
      │              │  Error  │              │              │  Error  │
      └──────────────┤ Handling├──────────────┴──────────────┤ Handling│
                     └─────────┘                             └─────────┘
```

---

## Components

### Domain Components

#### ConfigEntry

Represents a single configuration value with metadata.

```typescript
interface ConfigEntry {
  key: string;           // Dot-notation path: "database.host"
  value: ConfigValue;      // The configuration value
  source?: string;         // Source identifier for debugging
  timestamp?: number;      // Unix timestamp of load
}
```

#### ConfigValue

Union type for valid configuration values.

```typescript
type ConfigValue = 
  | string 
  | number 
  | boolean 
  | string[] 
  | Record<string, string>;
```

#### ConfigSnapshot

Immutable representation of configuration state.

```typescript
interface ConfigSnapshot {
  entries: Record<string, ConfigValue>;  // Key-value pairs
  sources: string[];                      // Source identifiers
  version: string;                        // Config version hash
  timestamp: number;                      // Load timestamp
}
```

#### ConfigError

Structured error information.

```typescript
interface ConfigError {
  message: string;           // Human-readable error message
  path: string[];            // Path to failing field
  value: unknown;          // Actual value that failed
  code: string;            // Error code for programmatic handling
  source?: string;         // Source that produced the error
}
```

### Port Components

#### ConfigSource

Core port interface for all configuration sources.

```typescript
export interface ConfigSource {
  /**
   * Source identifier for debugging and error reporting.
   */
  readonly name: string;

  /**
   * Load all configuration entries from this source.
   * 
   * @returns Promise resolving to array of config entries
   * @throws ConfigSourceError if source is unavailable
   */
  load(): Promise<ConfigEntry[]>;

  /**
   * Get a specific configuration value by key.
   * 
   * @param key - Configuration key to retrieve
   * @returns Promise resolving to value or undefined
   */
  get(key: string): Promise<ConfigValue | undefined>;

  /**
   * Set a configuration value (if source is writable).
   * 
   * @param key - Configuration key to set
   * @param value - Value to set
   * @throws Error if source is read-only
   */
  set(key: string, value: ConfigValue): Promise<void>;

  /**
   * Check if this source supports write operations.
   */
  isWritable(): boolean;

  /**
   * Optional: Watch for configuration changes.
   * 
   * @param callback - Function called when config changes
   */
  watch?(callback: (entries: ConfigEntry[]) => void): void;

  /**
   * Optional: Stop watching for changes.
   */
  unwatch?(): void;
}
```

#### ConfigValidator

Port for configuration validation.

```typescript
export interface ConfigValidator<T> {
  /**
   * Validate a value against the schema.
   * 
   * @param value - Value to validate
   * @returns Promise resolving to validated type
   * @throws ConfigValidationError if validation fails
   */
  validate(value: unknown): Promise<T>;

  /**
   * Parse without throwing; returns result object.
   * 
   * @param value - Value to parse
   * @returns SafeParseResult with success flag
   */
  safeParse(value: unknown): SafeParseResult<T>;
}

type SafeParseResult<T> =
  | { success: true; data: T }
  | { success: false; errors: ConfigError[] };
```

### Application Components

#### ConfigService

Primary interface for application configuration access.

```typescript
export interface ConfigService<T> {
  /**
   * Load and validate configuration from all sources.
   * 
   * @returns Promise resolving to load result
   */
  load(): Promise<ConfigResult<T>>;

  /**
   * Get current configuration.
   * 
   * @returns Current configuration object
   * @throws ConfigNotLoadedError if load() was not called
   */
  get(): T;

  /**
   * Safe getter that returns undefined if not loaded.
   * 
   * @returns Configuration or undefined
   */
  getSafe(): T | undefined;

  /**
   * Reload configuration from all sources.
   * 
   * @returns Promise resolving to reload result
   */
  reload(): Promise<ConfigResult<T>>;

  /**
   * Subscribe to configuration changes.
   * 
   * @param callback - Function called when config changes
   */
  onChange(callback: (config: T) => void): void;

  /**
   * Subscribe to configuration errors.
   * 
   * @param callback - Function called on errors
   */
  onError(callback: (error: ConfigError[]) => void): void;

  /**
   * Unsubscribe from events.
   * 
   * @param callback - Callback to remove
   */
  off(callback: Function): void;

  /**
   * Check if configuration has been loaded.
   */
  isLoaded(): boolean;

  /**
   * Get the last load timestamp.
   */
  getLastLoadTime(): number | undefined;

  /**
   * Get the current configuration snapshot.
   */
  getSnapshot(): ConfigSnapshot | undefined;
}

type ConfigResult<T> =
  | { success: true; data: T; warnings?: string[]; snapshot: ConfigSnapshot }
  | { success: false; errors: ConfigError[] };
```

---

## Data Models

### Configuration Schema Definition

```typescript
// src/schemas/DatabaseConfig.ts
import { z } from 'zod';

/**
 * Database configuration schema.
 * 
 * Validates and provides defaults for database connections.
 */
export const DatabaseConfigSchema = z.object({
  host: z.string()
    .min(1, 'Database host cannot be empty')
    .default('localhost'),
  
  port: z.number()
    .int()
    .min(1, 'Port must be at least 1')
    .max(65535, 'Port must be at most 65535')
    .default(5432),
  
  username: z.string()
    .min(1, 'Username is required'),
  
  password: z.string()
    .min(8, 'Password must be at least 8 characters'),
  
  database: z.string()
    .min(1, 'Database name is required'),
  
  ssl: z.boolean()
    .default(false),
  
  poolSize: z.number()
    .int()
    .positive()
    .default(10),
  
  connectionTimeout: z.number()
    .int()
    .positive()
    .default(30000),
  
  idleTimeout: z.number()
    .int()
    .positive()
    .default(600000),
  
  sslConfig: z.object({
    ca: z.string().optional(),
    cert: z.string().optional(),
    key: z.string().optional(),
    rejectUnauthorized: z.boolean().default(true),
  }).optional(),
});

export type DatabaseConfig = z.infer<typeof DatabaseConfigSchema>;
```

### Application Configuration Schema

```typescript
// src/schemas/AppConfig.ts
import { z } from 'zod';
import { DatabaseConfigSchema } from './DatabaseConfig';
import { CacheConfigSchema } from './CacheConfig';
import { LoggingConfigSchema } from './LoggingConfig';

/**
 * Application configuration schema.
 * 
 * Top-level configuration combining all domain schemas.
 */
export const AppConfigSchema = z.object({
  // Environment identification
  env: z.enum(['development', 'staging', 'production', 'test'])
    .default('development'),
  
  // Server configuration
  server: z.object({
    host: z.string().default('0.0.0.0'),
    port: z.number().int().min(1).max(65535).default(3000),
    keepAliveTimeout: z.number().int().positive().default(5000),
    headersTimeout: z.number().int().positive().default(60000),
  }),
  
  // Security configuration
  security: z.object({
    cors: z.object({
      enabled: z.boolean().default(true),
      origins: z.array(z.string()).default(['*']),
      methods: z.array(z.string()).default(['GET', 'POST', 'PUT', 'DELETE']),
      credentials: z.boolean().default(false),
    }),
    rateLimit: z.object({
      enabled: z.boolean().default(true),
      windowMs: z.number().int().positive().default(900000), // 15 minutes
      maxRequests: z.number().int().positive().default(100),
    }),
    helmet: z.boolean().default(true),
  }),
  
  // Domain configurations
  database: DatabaseConfigSchema,
  cache: CacheConfigSchema,
  logging: LoggingConfigSchema,
  
  // Feature flags
  features: z.object({
    enableMetrics: z.boolean().default(false),
    enableTracing: z.boolean().default(false),
    enableCache: z.boolean().default(true),
    enableWebSockets: z.boolean().default(false),
    maintenanceMode: z.boolean().default(false),
  }),
  
  // External services
  externalServices: z.object({
    apiKey: z.string().optional(),
    webhookSecret: z.string().optional(),
    timeout: z.number().int().positive().default(30000),
  }).optional(),
});

export type AppConfig = z.infer<typeof AppConfigSchema>;
```

### Cache Configuration Schema

```typescript
// src/schemas/CacheConfig.ts
import { z } from 'zod';

export const CacheConfigSchema = z.object({
  type: z.enum(['memory', 'redis', 'memcached'])
    .default('memory'),
  
  ttl: z.number()
    .int()
    .positive()
    .default(3600000), // 1 hour in ms
  
  maxSize: z.number()
    .int()
    .positive()
    .default(1000),
  
  checkPeriod: z.number()
    .int()
    .positive()
    .default(600000), // 10 minutes
  
  // Redis-specific
  redis: z.object({
    host: z.string().default('localhost'),
    port: z.number().int().min(1).max(65535).default(6379),
    password: z.string().optional(),
    db: z.number().int().min(0).max(15).default(0),
    keyPrefix: z.string().default('app:'),
  }).optional(),
  
  // Memcached-specific
  memcached: z.object({
    servers: z.array(z.string()).default(['localhost:11211']),
    retries: z.number().int().nonnegative().default(2),
    retryDelay: z.number().int().positive().default(100),
  }).optional(),
});

export type CacheConfig = z.infer<typeof CacheConfigSchema>;
```

### Logging Configuration Schema

```typescript
// src/schemas/LoggingConfig.ts
import { z } from 'zod';

export const LoggingConfigSchema = z.object({
  level: z.enum(['debug', 'info', 'warn', 'error', 'fatal'])
    .default('info'),
  
  format: z.enum(['json', 'pretty', 'compact'])
    .default('json'),
  
  destination: z.enum(['stdout', 'stderr', 'file', 'remote'])
    .default('stdout'),
  
  // File logging
  file: z.object({
    path: z.string(),
    maxSize: z.number().int().positive().default(10485760), // 10MB
    maxFiles: z.number().int().positive().default(5),
    compress: z.boolean().default(true),
  }).optional(),
  
  // Remote logging
  remote: z.object({
    url: z.string().url(),
    batchSize: z.number().int().positive().default(100),
    flushInterval: z.number().int().positive().default(5000),
    apiKey: z.string(),
  }).optional(),
  
  // Redaction
  redact: z.array(z.string())
    .default(['password', 'secret', 'token', 'authorization']),
  
  // Sampling
  sampleRate: z.number()
    .min(0)
    .max(1)
    .default(1),
});

export type LoggingConfig = z.infer<typeof LoggingConfigSchema>;
```

---

## API Specification

### ConfigService Options

```typescript
interface ConfigServiceOptions<T> {
  /**
   * Zod schema for validation and type inference.
   */
  schema: z.ZodSchema<T>;

  /**
   * Configuration source(s).
   * Can be a single source or array of sources (converted to CompositeAdapter).
   */
  source: ConfigSource | ConfigSource[];

  /**
   * Optional: Default values merged before other sources.
   */
  defaults?: Partial<T>;

  /**
   * Optional: Enable hot reloading.
   */
  hotReload?: boolean;

  /**
   * Optional: Callback when config changes.
   */
  onChange?: (config: T) => void;

  /**
   * Optional: Callback when errors occur.
   */
  onError?: (errors: ConfigError[]) => void;
}
```

### ConfigService Implementation

```typescript
// src/services/ConfigServiceImpl.ts
import { z } from 'zod';
import { EventEmitter } from 'events';
import { ConfigSource, ConfigEntry, ConfigValue } from '../ports/config-source';
import { ConfigService, ConfigResult, ConfigSnapshot, ConfigError } from '../domain/ConfigService';
import { ConfigValidationError } from '../domain/config';
import { CompositeAdapter } from '../adapters/CompositeAdapter';

export class ConfigServiceImpl<T> extends EventEmitter implements ConfigService<T> {
  private config: T | undefined;
  private snapshot: ConfigSnapshot | undefined;
  private source: ConfigSource;
  private schema: z.ZodSchema<T>;
  private loaded = false;
  private lastLoadTime: number | undefined;

  constructor(options: ConfigServiceOptions<T>) {
    super();
    
    this.schema = options.schema;
    
    // Normalize source to single ConfigSource
    if (Array.isArray(options.source)) {
      this.source = new CompositeAdapter({
        sources: options.source.map((s, i) => ({ 
          adapter: s, 
          priority: (i + 1) * 10 
        })),
      });
    } else {
      this.source = options.source;
    }

    // Set up event handlers
    if (options.onChange) {
      this.on('change', options.onChange);
    }
    if (options.onError) {
      this.on('error', options.onError);
    }

    // Enable hot reloading if requested
    if (options.hotReload && this.source.watch) {
      this.source.watch(async (entries) => {
        await this.reload();
      });
    }
  }

  async load(): Promise<ConfigResult<T>> {
    try {
      const entries = await this.source.load();
      const record = this.entriesToRecord(entries);
      
      const parseResult = this.schema.safeParse(record);
      
      if (!parseResult.success) {
        const errors = this.zodErrorsToConfigErrors(parseResult.error);
        return { success: false, errors };
      }

      this.config = parseResult.data;
      this.loaded = true;
      this.lastLoadTime = Date.now();
      this.snapshot = this.createSnapshot(entries);

      this.emit('change', this.config);

      return {
        success: true,
        data: this.config,
        snapshot: this.snapshot,
      };
    } catch (error) {
      const configError: ConfigError = {
        message: error instanceof Error ? error.message : 'Unknown error',
        path: [],
        value: undefined,
        code: 'LOAD_ERROR',
        source: this.source.name,
      };
      
      this.emit('error', [configError]);
      
      return { success: false, errors: [configError] };
    }
  }

  get(): T {
    if (!this.loaded || this.config === undefined) {
      throw new ConfigNotLoadedError(
        'Configuration has not been loaded. Call load() first.'
      );
    }
    return this.config;
  }

  getSafe(): T | undefined {
    return this.config;
  }

  async reload(): Promise<ConfigResult<T>> {
    return this.load();
  }

  onChange(callback: (config: T) => void): void {
    this.on('change', callback);
  }

  onError(callback: (errors: ConfigError[]) => void): void {
    this.on('error', callback);
  }

  off(callback: Function): void {
    this.removeListener('change', callback as any);
    this.removeListener('error', callback as any);
  }

  isLoaded(): boolean {
    return this.loaded;
  }

  getLastLoadTime(): number | undefined {
    return this.lastLoadTime;
  }

  getSnapshot(): ConfigSnapshot | undefined {
    return this.snapshot;
  }

  private entriesToRecord(entries: ConfigEntry[]): Record<string, ConfigValue> {
    const record: Record<string, ConfigValue> = {};
    for (const entry of entries) {
      this.setPath(record, entry.key, entry.value);
    }
    return record;
  }

  private setPath(
    obj: Record<string, unknown>,
    path: string,
    value: ConfigValue
  ): void {
    const keys = path.split('.');
    let current: Record<string, unknown> = obj;
    
    for (let i = 0; i < keys.length - 1; i++) {
      const key = keys[i];
      if (!(key in current) || typeof current[key] !== 'object') {
        current[key] = {};
      }
      current = current[key] as Record<string, unknown>;
    }
    
    current[keys[keys.length - 1]] = value;
  }

  private zodErrorsToConfigErrors(zodError: z.ZodError): ConfigError[] {
    return zodError.errors.map((err) => ({
      message: err.message,
      path: err.path.map(String),
      value: undefined,
      code: err.code,
    }));
  }

  private createSnapshot(entries: ConfigEntry[]): ConfigSnapshot {
    return {
      entries: this.entriesToRecord(entries),
      sources: [...new Set(entries.map(e => e.source || 'unknown'))],
      version: this.hashEntries(entries),
      timestamp: Date.now(),
    };
  }

  private hashEntries(entries: ConfigEntry[]): string {
    // Simple hash for versioning - replace with crypto in production
    const content = JSON.stringify(entries.sort((a, b) => a.key.localeCompare(b.key)));
    let hash = 0;
    for (let i = 0; i < content.length; i++) {
      const char = content.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash;
    }
    return hash.toString(16);
  }
}

class ConfigNotLoadedError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'ConfigNotLoadedError';
  }
}
```

---

## Adapters

### FileAdapter

Loads configuration from JSON, YAML, or TOML files.

```typescript
// src/adapters/FileAdapter.ts
import { readFile, watch } from 'fs/promises';
import { ConfigSource, ConfigEntry, ConfigValue } from '../ports/config-source';

export interface FileAdapterOptions {
  /** Path to configuration file */
  path: string;
  
  /** File format (auto-detected from extension if not specified) */
  format?: 'json' | 'yaml' | 'toml';
  
  /** Enable file watching for hot reloading */
  watch?: boolean;
  
  /** Allow missing file (return empty config) */
  required?: boolean;
  
  /** Default values if file is missing */
  defaults?: Record<string, ConfigValue>;
}

export class FileAdapter implements ConfigSource {
  readonly name: string;
  private path: string;
  private format: 'json' | 'yaml' | 'toml';
  private required: boolean;
  private defaults: Record<string, ConfigValue>;
  private watcher?: ReturnType<typeof watch>;
  private watchCallbacks: Array<(entries: ConfigEntry[]) => void> = [];

  constructor(options: FileAdapterOptions) {
    this.path = options.path;
    this.name = `file:${this.path}`;
    this.format = options.format || this.detectFormat(this.path);
    this.required = options.required !== false;
    this.defaults = options.defaults || {};

    if (options.watch) {
      this.startWatching();
    }
  }

  async load(): Promise<ConfigEntry[]> {
    try {
      const content = await readFile(this.path, 'utf-8');
      const parsed = this.parse(content);
      return this.flatten(parsed);
    } catch (error) {
      if (!this.required && this.isNotFoundError(error)) {
        return this.flatten(this.defaults);
      }
      throw new FileLoadError(
        `Failed to load config from ${this.path}: ${error instanceof Error ? error.message : 'Unknown error'}`,
        this.path
      );
    }
  }

  async get(key: string): Promise<ConfigValue | undefined> {
    const entries = await this.load();
    const entry = entries.find(e => e.key === key);
    return entry?.value;
  }

  async set(key: string, value: ConfigValue): Promise<void> {
    // Implementation would read, modify, write
    throw new Error('FileAdapter set() not implemented - use ConfigManager');
  }

  isWritable(): boolean {
    return true;
  }

  watch(callback: (entries: ConfigEntry[]) => void): void {
    this.watchCallbacks.push(callback);
  }

  unwatch(): void {
    if (this.watcher) {
      this.watcher.return?.();
      this.watcher = undefined;
    }
    this.watchCallbacks = [];
  }

  private detectFormat(path: string): 'json' | 'yaml' | 'toml' {
    const ext = path.split('.').pop()?.toLowerCase();
    switch (ext) {
      case 'json': return 'json';
      case 'yaml':
      case 'yml': return 'yaml';
      case 'toml': return 'toml';
      default: return 'json';
    }
  }

  private parse(content: string): Record<string, ConfigValue> {
    switch (this.format) {
      case 'json':
        return JSON.parse(content);
      case 'yaml':
        // Would use yaml library
        throw new Error('YAML parsing not implemented in this example');
      case 'toml':
        // Would use toml library
        throw new Error('TOML parsing not implemented in this example');
      default:
        throw new Error(`Unknown format: ${this.format}`);
    }
  }

  private flatten(
    obj: Record<string, unknown>,
    prefix = ''
  ): ConfigEntry[] {
    const entries: ConfigEntry[] = [];
    
    for (const [key, value] of Object.entries(obj)) {
      const fullKey = prefix ? `${prefix}.${key}` : key;
      
      if (this.isConfigValue(value)) {
        entries.push({
          key: fullKey,
          value: value as ConfigValue,
          source: this.name,
          timestamp: Date.now(),
        });
      } else if (typeof value === 'object' && value !== null) {
        entries.push(...this.flatten(value as Record<string, unknown>, fullKey));
      }
    }
    
    return entries;
  }

  private isConfigValue(value: unknown): value is ConfigValue {
    return (
      typeof value === 'string' ||
      typeof value === 'number' ||
      typeof value === 'boolean' ||
      (Array.isArray(value) && value.every(v => typeof v === 'string')) ||
      (typeof value === 'object' && value !== null && 
       Object.values(value).every(v => typeof v === 'string'))
    );
  }

  private isNotFoundError(error: unknown): boolean {
    return error instanceof Error && 
           'code' in error && 
           error.code === 'ENOENT';
  }

  private async startWatching(): Promise<void> {
    // Simplified watching implementation
    // Production would use proper file watching
  }
}

class FileLoadError extends Error {
  constructor(message: string, public readonly path: string) {
    super(message);
    this.name = 'FileLoadError';
  }
}
```

### EnvAdapter

Loads configuration from environment variables.

```typescript
// src/adapters/EnvAdapter.ts
import { ConfigSource, ConfigEntry, ConfigValue } from '../ports/config-source';

export interface EnvAdapterOptions {
  /** Prefix for environment variables (e.g., 'APP_') */
  prefix?: string;
  
  /** Separator for nested keys (e.g., 'DATABASE__HOST') */
  separator?: string;
  
  /** Remove prefix from resulting keys */
  removePrefix?: boolean;
  
  /** Transform function for specific keys */
  transform?: Record<string, (value: string) => ConfigValue>;
  
  /** Required environment variables */
  required?: string[];
}

export class EnvAdapter implements ConfigSource {
  readonly name = 'env';
  private prefix: string;
  private separator: string;
  private removePrefix: boolean;
  private transform: Record<string, (value: string) => ConfigValue>;
  private required: string[];

  constructor(options: EnvAdapterOptions = {}) {
    this.prefix = options.prefix || '';
    this.separator = options.separator || '__';
    this.removePrefix = options.removePrefix !== false;
    this.transform = options.transform || {};
    this.required = options.required || [];
  }

  async load(): Promise<ConfigEntry[]> {
    const entries: ConfigEntry[] = [];
    const missing: string[] = [];

    for (const [key, value] of Object.entries(process.env)) {
      if (key.startsWith(this.prefix) && value !== undefined) {
        const configKey = this.normalizeKey(key);
        const configValue = this.coerceValue(configKey, value);
        
        entries.push({
          key: configKey,
          value: configValue,
          source: this.name,
          timestamp: Date.now(),
        });
      }
    }

    // Check required variables
    for (const req of this.required) {
      const fullKey = this.prefix + req;
      if (!(fullKey in process.env) || process.env[fullKey] === undefined) {
        missing.push(req);
      }
    }

    if (missing.length > 0) {
      throw new EnvConfigError(
        `Required environment variables missing: ${missing.join(', ')}`,
        missing
      );
    }

    return entries;
  }

  async get(key: string): Promise<ConfigValue | undefined> {
    const fullKey = this.prefix + key.toUpperCase();
    const value = process.env[fullKey];
    
    if (value === undefined) return undefined;
    
    return this.coerceValue(key, value);
  }

  async set(key: string, value: ConfigValue): Promise<void> {
    throw new EnvReadOnlyError('Environment variables are read-only in most contexts');
  }

  isWritable(): boolean {
    return false;
  }

  private normalizeKey(key: string): string {
    let result = key;
    
    // Remove prefix
    if (this.removePrefix && result.startsWith(this.prefix)) {
      result = result.slice(this.prefix.length);
    }
    
    // Replace separator with dots for nesting
    result = result.replace(new RegExp(this.separator, 'g'), '.');
    
    // Convert to lowercase for consistency
    return result.toLowerCase();
  }

  private coerceValue(key: string, value: string): ConfigValue {
    // Use custom transform if provided
    if (this.transform[key]) {
      return this.transform[key](value);
    }

    // Try JSON parsing for complex types
    try {
      const parsed = JSON.parse(value);
      if (typeof parsed === 'object' || Array.isArray(parsed)) {
        return parsed;
      }
    } catch {
      // Not JSON, continue with type coercion
    }

    // Boolean coercion
    if (value.toLowerCase() === 'true') return true;
    if (value.toLowerCase() === 'false') return false;

    // Number coercion
    if (value !== '' && !isNaN(Number(value)) && !isNaN(parseFloat(value))) {
      const num = Number(value);
      if (Number.isInteger(num)) return parseInt(value, 10);
      return num;
    }

    // Default: string
    return value;
  }
}

class EnvConfigError extends Error {
  constructor(
    message: string,
    public readonly missingVars: string[]
  ) {
    super(message);
    this.name = 'EnvConfigError';
  }
}

class EnvReadOnlyError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'EnvReadOnlyError';
  }
}
```

### MemoryAdapter

In-memory configuration for testing and defaults.

```typescript
// src/adapters/MemoryAdapter.ts
import { ConfigSource, ConfigEntry, ConfigValue } from '../ports/config-source';

export interface MemoryAdapterOptions {
  /** Initial configuration values */
  initial?: Record<string, ConfigValue>;
  
  /** Allow write operations */
  writable?: boolean;
}

export class MemoryAdapter implements ConfigSource {
  readonly name: string;
  private data: Map<string, ConfigValue>;
  private writable: boolean;

  constructor(options: MemoryAdapterOptions = {}) {
    this.name = 'memory';
    this.data = new Map(Object.entries(options.initial || {}));
    this.writable = options.writable !== false;
  }

  async load(): Promise<ConfigEntry[]> {
    const entries: ConfigEntry[] = [];
    
    for (const [key, value] of this.data) {
      entries.push({
        key,
        value,
        source: this.name,
        timestamp: Date.now(),
      });
    }
    
    return entries;
  }

  async get(key: string): Promise<ConfigValue | undefined> {
    return this.data.get(key);
  }

  async set(key: string, value: ConfigValue): Promise<void> {
    if (!this.writable) {
      throw new Error('MemoryAdapter is read-only');
    }
    this.data.set(key, value);
  }

  isWritable(): boolean {
    return this.writable;
  }

  /** Set multiple values at once (convenience method) */
  setMultiple(values: Record<string, ConfigValue>): void {
    if (!this.writable) {
      throw new Error('MemoryAdapter is read-only');
    }
    for (const [key, value] of Object.entries(values)) {
      this.data.set(key, value);
    }
  }

  /** Clear all values (testing utility) */
  clear(): void {
    if (!this.writable) {
      throw new Error('MemoryAdapter is read-only');
    }
    this.data.clear();
  }

  /** Get all keys (testing utility) */
  keys(): string[] {
    return Array.from(this.data.keys());
  }
}
```

### CompositeAdapter

Combines multiple sources with priority ordering.

```typescript
// src/adapters/CompositeAdapter.ts
import { ConfigSource, ConfigEntry, ConfigValue } from '../ports/config-source';

export interface CompositeSource {
  adapter: ConfigSource;
  priority: number;
}

export interface CompositeAdapterOptions {
  sources: CompositeSource[];
  strategy?: 'merge' | 'override';
}

export class CompositeAdapter implements ConfigSource {
  readonly name = 'composite';
  private sources: CompositeSource[];
  private strategy: 'merge' | 'override';

  constructor(options: CompositeAdapterOptions) {
    this.sources = options.sources.sort((a, b) => a.priority - b.priority);
    this.strategy = options.strategy || 'merge';
  }

  async load(): Promise<ConfigEntry[]> {
    const errors: Array<{ source: string; error: Error }> = [];
    let merged: Record<string, ConfigValue> = {};

    // Load all sources in parallel
    const results = await Promise.allSettled(
      this.sources.map(({ adapter }) => adapter.load())
    );

    // Process in priority order
    for (let i = 0; i < this.sources.length; i++) {
      const { adapter, priority } = this.sources[i];
      const result = results[i];

      if (result.status === 'rejected') {
        errors.push({ source: adapter.name, error: result.reason });
        continue;
      }

      // Convert entries to record
      const record: Record<string, ConfigValue> = {};
      for (const entry of result.value) {
        this.setNestedValue(record, entry.key, entry.value);
      }

      // Merge based on strategy
      if (this.strategy === 'merge') {
        merged = this.deepMerge(merged, record);
      } else {
        merged = { ...merged, ...record };
      }
    }

    // If all sources failed, throw error
    if (errors.length === this.sources.length) {
      throw new CompositeLoadError(
        'All configuration sources failed to load',
        errors
      );
    }

    // Convert back to entries
    return this.flattenRecord(merged);
  }

  async get(key: string): Promise<ConfigValue | undefined> {
    // Try sources in reverse priority order (highest first)
    const reversed = [...this.sources].reverse();
    
    for (const { adapter } of reversed) {
      const value = await adapter.get(key);
      if (value !== undefined) {
        return value;
      }
    }
    
    return undefined;
  }

  async set(key: string, value: ConfigValue): Promise<void> {
    // Find first writable source
    for (const { adapter } of this.sources) {
      if (adapter.isWritable()) {
        await adapter.set(key, value);
        return;
      }
    }
    
    throw new Error('No writable source available in composite');
  }

  isWritable(): boolean {
    return this.sources.some(({ adapter }) => adapter.isWritable());
  }

  private setNestedValue(
    obj: Record<string, unknown>,
    path: string,
    value: ConfigValue
  ): void {
    const keys = path.split('.');
    let current: Record<string, unknown> = obj;
    
    for (let i = 0; i < keys.length - 1; i++) {
      const key = keys[i];
      if (!(key in current) || typeof current[key] !== 'object') {
        current[key] = {};
      }
      current = current[key] as Record<string, unknown>;
    }
    
    current[keys[keys.length - 1]] = value;
  }

  private deepMerge(
    target: Record<string, unknown>,
    source: Record<string, unknown>
  ): Record<string, unknown> {
    const result: Record<string, unknown> = { ...target };
    
    for (const [key, value] of Object.entries(source)) {
      // Security: skip prototype pollution keys
      if (key === '__proto__' || key === 'constructor') {
        continue;
      }
      
      if (
        typeof value === 'object' &&
        value !== null &&
        !Array.isArray(value) &&
        typeof result[key] === 'object' &&
        result[key] !== null &&
        !Array.isArray(result[key])
      ) {
        result[key] = this.deepMerge(
          result[key] as Record<string, unknown>,
          value as Record<string, unknown>
        );
      } else {
        result[key] = value;
      }
    }
    
    return result;
  }

  private flattenRecord(record: Record<string, ConfigValue>): ConfigEntry[] {
    const entries: ConfigEntry[] = [];
    
    const flatten = (obj: Record<string, unknown>, prefix = ''): void => {
      for (const [key, value] of Object.entries(obj)) {
        const fullKey = prefix ? `${prefix}.${key}` : key;
        
        if (this.isPrimitiveValue(value)) {
          entries.push({
            key: fullKey,
            value: value as ConfigValue,
            source: this.name,
            timestamp: Date.now(),
          });
        } else if (typeof value === 'object' && value !== null) {
          flatten(value as Record<string, unknown>, fullKey);
        }
      }
    };
    
    flatten(record);
    return entries;
  }

  private isPrimitiveValue(value: unknown): value is ConfigValue {
    return (
      typeof value === 'string' ||
      typeof value === 'number' ||
      typeof value === 'boolean' ||
      (Array.isArray(value) && value.every(v => typeof v === 'string')) ||
      (typeof value === 'object' && value !== null && 
       Object.values(value).every(v => typeof v === 'string'))
    );
  }
}

class CompositeLoadError extends Error {
  constructor(
    message: string,
    public readonly errors: Array<{ source: string; error: Error }>
  ) {
    super(message);
    this.name = 'CompositeLoadError';
  }
}
```

---

## Validation

### Zod Schema Integration

Conft uses Zod for runtime validation with TypeScript type inference.

```typescript
// Validation flow
const result = schema.safeParse(rawConfig);

if (result.success) {
  // result.data is fully typed as T
  return result.data;
} else {
  // result.error contains structured validation errors
  throw new ConfigValidationError(result.error);
}
```

### Custom Validators

```typescript
// Custom validation with refinements
export const PortSchema = z.number()
  .int()
  .min(1)
  .max(65535)
  .refine(
    (port) => port !== 0,
    { message: 'Port cannot be 0' }
  );

// Email validation
export const EmailSchema = z.string()
  .email()
  .refine(
    (email) => !email.endsWith('@example.com'),
    { message: 'Example domain not allowed in production' }
  );

// URL validation with protocol restriction
export const WebhookUrlSchema = z.string()
  .url()
  .refine(
    (url) => url.startsWith('https://'),
    { message: 'Webhook URL must use HTTPS' }
  );
```

### Transformations

```typescript
// Preprocess for environment variable coercion
export const PortFromEnvSchema = z.preprocess(
  (val) => {
    if (typeof val === 'string') return parseInt(val, 10);
    if (typeof val === 'number') return val;
    return undefined;
  },
  z.number().int().min(1).max(65535)
);

// Transform for normalization
export const HostSchema = z.string()
  .transform((host) => host.toLowerCase().trim())
  .pipe(z.string().min(1));

// Default values
export const TimeoutSchema = z.number()
  .int()
  .positive()
  .default(30000); // 30 seconds
```

---

## Error Handling

### Error Hierarchy

```
ConfigError (base)
├── ConfigValidationError
│   └── Field-level validation failures
├── ConfigSourceError
│   ├── FileLoadError
│   ├── EnvConfigError
│   └── CompositeLoadError
├── ConfigNotLoadedError
└── ConfigAccessError
```

### Error Codes

| Code | Description | Recovery |
|------|-------------|----------|
| `LOAD_ERROR` | General source loading failure | Check source availability |
| `PARSE_ERROR` | Invalid JSON/YAML/TOML syntax | Fix configuration file syntax |
| `VALIDATION_ERROR` | Schema validation failed | Correct configuration values |
| `REQUIRED_MISSING` | Required field not provided | Add missing configuration |
| `TYPE_MISMATCH` | Value type doesn't match schema | Convert to correct type |
| `RANGE_ERROR` | Value outside allowed range | Adjust to valid range |
| `SOURCE_UNAVAILABLE` | Configuration source inaccessible | Check file permissions, network |

### Error Messages

Error messages follow the format:
```
[{source}] {path}: {message} (code: {code})
```

Example:
```
[file:./config.json] database.port: Expected number, received string (code: invalid_type)
[env] api_key: Required environment variable missing (code: REQUIRED_MISSING)
```

---

## Performance Requirements

### Loading Performance

| Metric | Target | Maximum |
|--------|--------|---------|
| JSON file load (1KB) | < 5ms | < 10ms |
| JSON file load (100KB) | < 20ms | < 50ms |
| Environment load | < 1ms | < 5ms |
| Validation (simple schema) | < 5ms | < 10ms |
| Validation (complex schema) | < 20ms | < 50ms |
| Composite (3 sources) | < 30ms | < 100ms |

### Memory Requirements

| Metric | Target |
|--------|--------|
| Base memory overhead | < 100KB |
| Per config entry | ~200 bytes |
| Maximum config entries | 10,000 |

### File Watching

| Platform | Mechanism | Max watched files |
|----------|-----------|-----------------|
| Linux | inotify | 8,192 (system limit) |
| macOS | FSEvents | Unlimited |
| Windows | ReadDirectoryChangesW | Unlimited |

---

## Security Considerations

### Prototype Pollution Prevention

All merge operations explicitly reject `__proto__` and `constructor` keys:

```typescript
if (key === '__proto__' || key === 'constructor') {
  continue; // Skip potential prototype pollution keys
}
```

### Secret Handling

1. Secrets are never logged
2. Secrets are marked in schema with `z.string().secret()`
3. Error messages redact secret values
4. `getSnapshot()` excludes secret keys

### File Permissions

FileAdapter warns on overly permissive configuration files:

```typescript
if (mode & 0o044) {
  console.warn(`Config file ${path} is world-readable`);
}
```

### Environment Variable Security

EnvAdapter supports `.env` file loading with security measures:
- `.env` files should be in `.gitignore`
- `.env.example` can be committed as template
- Variable expansion is disabled by default (prevents injection)

---

## Testing Strategy

### Unit Testing

Test each adapter in isolation:

```typescript
import { describe, it, expect } from 'vitest';
import { MemoryAdapter } from '../adapters/MemoryAdapter';

describe('MemoryAdapter', () => {
  it('should load initial values', async () => {
    const adapter = new MemoryAdapter({
      initial: { port: 3000, host: 'localhost' }
    });
    
    const entries = await adapter.load();
    
    expect(entries).toHaveLength(2);
    expect(entries.find(e => e.key === 'port')?.value).toBe(3000);
  });

  it('should support writes when writable', async () => {
    const adapter = new MemoryAdapter({ writable: true });
    
    await adapter.set('key', 'value');
    const value = await adapter.get('key');
    
    expect(value).toBe('value');
  });
});
```

### Integration Testing

Test ConfigService with real adapters:

```typescript
describe('ConfigService Integration', () => {
  it('should load from file and validate', async () => {
    const service = new ConfigServiceImpl({
      schema: AppConfigSchema,
      source: new FileAdapter({
        path: './tests/fixtures/valid-config.json'
      })
    });
    
    const result = await service.load();
    
    expect(result.success).toBe(true);
    expect(result.data).toBeDefined();
  });
});
```

### Contract Testing

Ensure all adapters implement ConfigSource correctly:

```typescript
export function testConfigSourceContract(
  name: string,
  createAdapter: () => ConfigSource
) {
  describe(`${name} ConfigSource Contract`, () => {
    let adapter: ConfigSource;
    
    beforeEach(() => {
      adapter = createAdapter();
    });
    
    it('should have a name', () => {
      expect(adapter.name).toBeDefined();
      expect(typeof adapter.name).toBe('string');
    });
    
    it('should implement load()', async () => {
      const result = await adapter.load();
      expect(Array.isArray(result)).toBe(true);
    });
    
    it('should report writability', () => {
      expect(typeof adapter.isWritable()).toBe('boolean');
    });
  });
}
```

---

## Implementation Details

### Type Inference

Conft leverages Zod's type inference for zero-duplication type definitions:

```typescript
// Schema definition (single source of truth)
const ConfigSchema = z.object({
  port: z.number().default(3000),
  host: z.string().default('localhost'),
});

// Type automatically inferred
// No separate interface needed!
type Config = z.infer<typeof ConfigSchema>;
// Equivalent to: { port: number; host: string }
```

### Async Initialization Pattern

Applications must handle async configuration loading at startup:

```typescript
// main.ts - Entry point pattern
async function bootstrap() {
  // 1. Load configuration
  const configService = new ConfigServiceImpl({
    schema: AppConfigSchema,
    source: new CompositeAdapter({
      sources: [
        { adapter: new FileAdapter('./config/default.json'), priority: 10 },
        { adapter: new EnvAdapter({ prefix: 'APP_' }), priority: 20 },
      ]
    })
  });
  
  const result = await configService.load();
  
  if (!result.success) {
    console.error('Configuration failed to load:');
    result.errors.forEach(e => console.error(`  - ${e.path}: ${e.message}`));
    process.exit(1);
  }
  
  // 2. Start application with validated config
  const app = createApp(result.data);
  await app.start();
}

bootstrap().catch(console.error);
```

### Hot Reloading

Enable hot reloading for development:

```typescript
const configService = new ConfigServiceImpl({
  schema: AppConfigSchema,
  source: new FileAdapter({
    path: './config/app.json',
    watch: true, // Enable file watching
  }),
  onChange: (config) => {
    console.log('Configuration reloaded');
    // Restart services or update in-place
  },
  onError: (errors) => {
    console.error('Configuration error:', errors);
  }
});
```

---

## Usage Examples

### Basic Web Server

```typescript
// server.ts
import { z } from 'zod';
import { ConfigServiceImpl } from '@phenotype/config-ts';
import { FileAdapter, EnvAdapter, CompositeAdapter } from '@phenotype/config-ts';
import express from 'express';

const ServerConfigSchema = z.object({
  port: z.number().default(3000),
  host: z.string().default('0.0.0.0'),
  logLevel: z.enum(['debug', 'info', 'warn', 'error']).default('info'),
});

async function main() {
  const configService = new ConfigServiceImpl({
    schema: ServerConfigSchema,
    source: new CompositeAdapter({
      sources: [
        { adapter: new FileAdapter('./config/server.json'), priority: 10 },
        { adapter: new EnvAdapter({ prefix: 'SERVER_' }), priority: 20 },
      ]
    })
  });
  
  const result = await configService.load();
  if (!result.success) {
    console.error('Failed to load configuration:', result.errors);
    process.exit(1);
  }
  
  const config = result.data;
  const app = express();
  
  app.listen(config.port, config.host, () => {
    console.log(`Server running on http://${config.host}:${config.port}`);
  });
}

main();
```

### Database Configuration

```typescript
// database.ts
import { z } from 'zod';
import { ConfigServiceImpl, EnvAdapter } from '@phenotype/config-ts';
import { Pool } from 'pg';

const DatabaseConfigSchema = z.object({
  host: z.string().default('localhost'),
  port: z.number().default(5432),
  database: z.string(),
  user: z.string(),
  password: z.string(),
  ssl: z.boolean().default(false),
  poolSize: z.number().default(10),
});

async function createDatabasePool() {
  const configService = new ConfigServiceImpl({
    schema: DatabaseConfigSchema,
    source: new EnvAdapter({
      prefix: 'DB_',
      transform: {
        port: (v) => parseInt(v, 10),
        ssl: (v) => v === 'true',
        poolSize: (v) => parseInt(v, 10),
      },
      required: ['DB_DATABASE', 'DB_USER', 'DB_PASSWORD']
    })
  });
  
  const result = await configService.load();
  if (!result.success) {
    throw new Error(`Database config error: ${result.errors[0].message}`);
  }
  
  const config = result.data;
  return new Pool({
    host: config.host,
    port: config.port,
    database: config.database,
    user: config.user,
    password: config.password,
    ssl: config.ssl,
    max: config.poolSize,
  });
}
```

### Testing with Mocked Config

```typescript
// app.test.ts
import { describe, it, expect, beforeEach } from 'vitest';
import { ConfigServiceImpl, MemoryAdapter } from '@phenotype/config-ts';
import { AppConfigSchema } from './schemas';
import { createApp } from './app';

describe('Application', () => {
  let configService: ConfigServiceImpl<AppConfig>;
  
  beforeEach(async () => {
    configService = new ConfigServiceImpl({
      schema: AppConfigSchema,
      source: new MemoryAdapter({
        initial: {
          env: 'test',
          port: 9999,
          database: {
            host: 'localhost',
            port: 5432,
            username: 'test',
            password: 'test',
            database: 'test_db'
          }
        }
      })
    });
    
    await configService.load();
  });
  
  it('should use test configuration', () => {
    const app = createApp(configService.get());
    expect(app.getPort()).toBe(9999);
  });
});
```

### Multi-Environment Setup

```typescript
// config/index.ts
import { z } from 'zod';
import { ConfigServiceImpl, FileAdapter, EnvAdapter, CompositeAdapter } from '@phenotype/config-ts';

const AppConfigSchema = z.object({
  env: z.enum(['development', 'staging', 'production']),
  server: z.object({
    port: z.number().default(3000),
    host: z.string().default('0.0.0.0'),
  }),
  database: z.object({
    url: z.string(),
    poolSize: z.number().default(10),
  }),
  features: z.object({
    enableMetrics: z.boolean().default(false),
    enableCache: z.boolean().default(true),
  }),
});

export async function loadConfig() {
  const env = process.env.NODE_ENV || 'development';
  
  const configService = new ConfigServiceImpl({
    schema: AppConfigSchema,
    source: new CompositeAdapter({
      sources: [
        // Layer 1: Default configuration
        {
          adapter: new FileAdapter({
            path: './config/default.json',
            required: false,
          }),
          priority: 10
        },
        // Layer 2: Environment-specific file
        {
          adapter: new FileAdapter({
            path: `./config/${env}.json`,
            required: false,
          }),
          priority: 20
        },
        // Layer 3: Local overrides (gitignored)
        {
          adapter: new FileAdapter({
            path: './config/local.json',
            required: false,
          }),
          priority: 30
        },
        // Layer 4: Environment variables (highest priority)
        {
          adapter: new EnvAdapter({ prefix: 'APP_' }),
          priority: 40
        },
      ]
    })
  });
  
  const result = await configService.load();
  
  if (!result.success) {
    console.error('Configuration errors:');
    result.errors.forEach(err => {
      console.error(`  [${err.source}] ${err.path.join('.')}: ${err.message}`);
    });
    process.exit(1);
  }
  
  return result.data;
}
```

---

## Project Structure

```
Conft/
├── src/
│   ├── index.ts                    # Public API exports
│   ├── domain/
│   │   ├── config.ts              # Domain models and errors
│   │   └── ConfigService.ts        # Service interface
│   ├── ports/
│   │   └── config-source.ts       # ConfigSource port
│   ├── adapters/
│   │   ├── FileAdapter.ts         # File-based config
│   │   ├── EnvAdapter.ts          # Environment variables
│   │   ├── MemoryAdapter.ts       # In-memory/testing
│   │   └── CompositeAdapter.ts    # Multi-source composition
│   ├── services/
│   │   └── ConfigServiceImpl.ts   # Service implementation
│   └── schemas/
│       └── (example schemas)
├── tests/
│   ├── unit/
│   │   ├── adapters/
│   │   └── domain/
│   ├── integration/
│   │   └── config-service.test.ts
│   └── fixtures/
│       ├── default.json
│       └── production.json
├── docs/
│   ├── .vitepress/
│   │   └── config.ts
│   ├── guide/
│   │   ├── getting-started.md
│   │   ├── schemas.md
│   │   └── adapters.md
│   └── api/
│       └── reference.md
├── adr/
│   ├── ADR-001-zod-validation.md
│   ├── ADR-002-async-first.md
│   ├── ADR-003-deep-merge.md
│   └── ADR-004-composite-adapter.md
├── package.json
├── tsconfig.json
├── tsup.config.ts
└── vitest.config.ts
```

---

## Migration Guide

### Migrating from node-config

**Before (node-config):**
```javascript
const config = require('config');
const dbHost = config.get('database.host');
```

**After (Conft):**
```typescript
import { ConfigServiceImpl, FileAdapter } from '@phenotype/config-ts';
import { z } from 'zod';

const ConfigSchema = z.object({
  database: z.object({
    host: z.string().default('localhost'),
  }),
});

const service = new ConfigServiceImpl({
  schema: ConfigSchema,
  source: new FileAdapter('./config/default.json'),
});

const result = await service.load();
const dbHost = result.data.database.host; // Fully typed!
```

### Migrating from dotenv

**Before (dotenv):**
```javascript
require('dotenv').config();
const port = parseInt(process.env.PORT, 10);
```

**After (Conft):**
```typescript
import { EnvAdapter } from '@phenotype/config-ts';

const service = new ConfigServiceImpl({
  schema: z.object({ port: z.number().default(3000) }),
  source: new EnvAdapter({
    prefix: 'APP_',
    transform: { port: (v) => parseInt(v, 10) }
  }),
});
```

---

## Future Work

### Planned Features

1. **Remote Config Sources**
   - HTTP/HTTPS configuration loading
   - AWS Systems Manager Parameter Store adapter
   - HashiCorp Vault adapter
   - etcd/consul adapter

2. **Advanced Validation**
   - Conditional schema validation
   - Cross-field validation
   - Custom error message templates

3. **Secret Management**
   - Automatic secret detection
   - Secret provider integration
   - Rotation support

4. **Hot Reloading**
   - File watching with debouncing
   - Graceful service restart
   - Partial reload (changed keys only)

5. **CLI Tool**
   - `conft validate` - Validate config files
   - `conft generate` - Generate TypeScript from schema
   - `conft migrate` - Migration helpers

6. **IDE Integration**
   - VS Code extension for schema validation
   - Autocomplete for config keys
   - Go-to-definition for config references

### Version Roadmap

| Version | Features |
|---------|----------|
| 1.0 | Core adapters (File, Env, Memory, Composite), Zod validation, basic hot reload |
| 1.1 | Remote HTTP adapter, secret detection |
| 1.2 | CLI tool, VS Code extension |
| 2.0 | Plugin system, custom adapter API stability |

---

## References

### Dependencies

| Package | Purpose | Version |
|---------|---------|---------|
| zod | Schema validation | ^3.22.4 |

### Dev Dependencies

| Package | Purpose | Version |
|---------|---------|---------|
| typescript | Language | ^5.3.2 |
| vitest | Testing | ^1.0.4 |
| tsup | Building | ^8.0.1 |

### Related Projects

- [Zod](https://zod.dev/) - TypeScript-first schema validation
- [phenotype-config](https://github.com/phenotype/config) - Rust implementation
- [node-config](https://github.com/node-config/node-config) - Node.js config (inspiration)
- [Viper](https://github.com/spf13/viper) - Go configuration (inspiration)

---

## Appendix A: Glossary

| Term | Definition |
|------|------------|
| **Adapter** | Implementation of a port for a specific external system |
| **ConfigEntry** | Single configuration value with metadata |
| **ConfigSource** | Port interface for configuration sources |
| **ConfigValue** | Valid configuration value types |
| **Deep Merge** | Recursive merging of nested objects |
| **Hexagonal Architecture** | Ports and adapters pattern |
| **Port** | Interface defining a boundary between layers |
| **Schema** | Definition of valid configuration structure |
| **Zod** | TypeScript-first schema validation library |

---

## Appendix B: Changelog

### 1.0.0 (2026-04-04)

- Initial release
- FileAdapter with JSON support
- EnvAdapter with type coercion
- MemoryAdapter for testing
- CompositeAdapter for multi-source
- ConfigService with async loading
- Zod schema validation
- Hot reloading support

---

*Document Version: 1.0.0*  
*Last Updated: 2026-04-04*  
*Maintainer: Conft Core Team*

---

## Appendix C: Configuration Patterns Catalog

This appendix documents common configuration patterns and their implementation in Conft.

### C.1 Environment-Specific Configuration

Pattern: Different configuration values for development, staging, and production environments.

```typescript
// config/index.ts
import { z } from 'zod';
import { CompositeAdapter, FileAdapter, EnvAdapter } from '@phenotype/config-ts';

const EnvSchema = z.enum(['development', 'staging', 'production', 'test']);
type Env = z.infer<typeof EnvSchema>;

export async function loadEnvironmentConfig(env: Env) {
  return new ConfigServiceImpl({
    schema: AppConfigSchema,
    source: new CompositeAdapter({
      sources: [
        { adapter: new FileAdapter('./config/default.json'), priority: 10 },
        { adapter: new FileAdapter(`./config/${env}.json`), priority: 20 },
        { adapter: new EnvAdapter({ prefix: 'APP_' }), priority: 30 },
      ]
    })
  });
}
```

### C.2 Secret Injection Pattern

Pattern: Reference secrets by name; resolve at runtime.

```typescript
// Schema with secret references
const ConfigSchema = z.object({
  database: z.object({
    url: z.string(),
    // Reference to secret, not the secret itself
    passwordRef: z.string().default('vault:database/password'),
  }),
});

// Secret resolution adapter
class SecretResolvingAdapter implements ConfigSource {
  constructor(
    private inner: ConfigSource,
    private secretProvider: SecretProvider
  ) {}

  async load(): Promise<ConfigEntry[]> {
    const entries = await this.inner.load();
    return Promise.all(
      entries.map(async (e) => ({
        ...e,
        value: await this.resolveSecrets(e.value),
      }))
    );
  }

  private async resolveSecrets(value: ConfigValue): Promise<ConfigValue> {
    if (typeof value === 'string' && value.startsWith('vault:')) {
      return this.secretProvider.get(value.slice(6));
    }
    return value;
  }
}
```

### C.3 Feature Flag Configuration

Pattern: Enable/disable features via configuration.

```typescript
const FeaturesSchema = z.object({
  newDashboard: z.boolean().default(false),
  betaApi: z.boolean().default(false),
  maintenanceMode: z.boolean().default(false),
});

// Usage with kill switch
const config = await configService.load();
if (config.data.features.maintenanceMode) {
  return res.status(503).json({ error: 'Maintenance in progress' });
}
```

### C.4 Tiered Timeout Configuration

Pattern: Different timeouts for different dependency tiers.

```typescript
const TimeoutsSchema = z.object({
  critical: z.number().default(5000),   // Database - fail fast
  standard: z.number().default(30000),  // APIs - reasonable
  background: z.number().default(300000), // Analytics - generous
});

// Usage
const dbTimeout = config.timeouts.critical;
const analyticsTimeout = config.timeouts.background;
```

### C.5 Conditional Configuration

Pattern: Configuration that varies based on other configuration values.

```typescript
const DynamicConfigSchema = z.discriminatedUnion('storageType', [
  z.object({
    storageType: z.literal('local'),
    localPath: z.string(),
  }),
  z.object({
    storageType: z.literal('s3'),
    bucket: z.string(),
    region: z.string(),
    accessKeyId: z.string(),
    secretAccessKey: z.string(),
  }),
]);
```

---

## Appendix D: Troubleshooting Guide

### D.1 Common Errors

#### Error: "Configuration has not been loaded. Call load() first."

**Cause**: Accessing `configService.get()` before `await configService.load()` completed.

**Solution**:
```typescript
// Wrong
const service = new ConfigServiceImpl({ schema, source });
const config = service.get(); // Throws!

// Right
const service = new ConfigServiceImpl({ schema, source });
const result = await service.load();
const config = result.data;
```

#### Error: "Expected number, received string"

**Cause**: Environment variable value not coerced to correct type.

**Solution**:
```typescript
new EnvAdapter({
  prefix: 'APP_',
  transform: {
    port: (v) => parseInt(v, 10),
  },
})
```

#### Error: "Required environment variables missing: API_KEY"

**Cause**: Required environment variable not set.

**Solution**: Set the variable or make it optional in schema with `.optional()`.

### D.2 Debugging Configuration Loading

Enable debug mode:

```typescript
const service = new ConfigServiceImpl({
  schema,
  source: new CompositeAdapter({
    sources: [...],
    debug: true, // Logs merge decisions
  }),
});
```

### D.3 Performance Profiling

```typescript
console.time('config-load');
const result = await service.load();
console.timeEnd('config-load');
```

---

## Appendix E: Best Practices

### E.1 Schema Organization

Organize schemas by domain, not by source:

```
src/
├── schemas/
│   ├── database.ts
│   ├── cache.ts
│   ├── logging.ts
│   └── features.ts
├── config.ts          # Composite schema & types
```

### E.2 Configuration Reviews

Treat configuration schemas as code:
- Review in pull requests
- Require tests for new config keys
- Document breaking changes

### E.3 Validation at CI

```yaml
# .github/workflows/config-check.yml
- name: Validate Configuration
  run: |
    npx ts-node scripts/validate-config.ts \
      --schema ./src/schemas/AppConfig.ts \
      --config ./config/production.json
```

### E.4 Secret Rotation

```typescript
// Periodic secret refresh
setInterval(async () => {
  await configService.reload();
}, 3600000); // Every hour
```

---

*Document Version: 1.0.0*  
*Last Updated: 2026-04-04*  
*Maintainer: Conft Core Team*


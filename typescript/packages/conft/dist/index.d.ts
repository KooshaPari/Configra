import { z } from 'zod';

/**
 * Domain models for configuration management.
 *
 * Following hexagonal architecture:
 * - Pure domain logic with no external dependencies
 * - Zod schemas for runtime validation
 * - Immutable config objects
 *
 * xDD Principles:
 * - KISS: Simple data classes
 * - DRY: Shared validation schemas
 * - PoLA: Descriptive error messages
 */

/**
 * Base config value types.
 */
declare const ConfigValueSchema: z.ZodUnion<[z.ZodString, z.ZodNumber, z.ZodBoolean, z.ZodArray<z.ZodString, "many">, z.ZodRecord<z.ZodString, z.ZodString>]>;
type ConfigValue = z.infer<typeof ConfigValueSchema>;
/**
 * Config entry with metadata.
 */
declare const ConfigEntrySchema: z.ZodObject<{
    key: z.ZodString;
    value: z.ZodUnion<[z.ZodString, z.ZodNumber, z.ZodBoolean, z.ZodArray<z.ZodString, "many">, z.ZodRecord<z.ZodString, z.ZodString>]>;
    source: z.ZodOptional<z.ZodString>;
    timestamp: z.ZodOptional<z.ZodNumber>;
}, "strip", z.ZodTypeAny, {
    value: string | number | boolean | string[] | Record<string, string>;
    key: string;
    source?: string | undefined;
    timestamp?: number | undefined;
}, {
    value: string | number | boolean | string[] | Record<string, string>;
    key: string;
    source?: string | undefined;
    timestamp?: number | undefined;
}>;
type ConfigEntry = z.infer<typeof ConfigEntrySchema>;
/**
 * Config snapshot - immutable config state.
 */
declare const ConfigSnapshotSchema: z.ZodObject<{
    entries: z.ZodRecord<z.ZodString, z.ZodUnion<[z.ZodString, z.ZodNumber, z.ZodBoolean, z.ZodArray<z.ZodString, "many">, z.ZodRecord<z.ZodString, z.ZodString>]>>;
    sources: z.ZodArray<z.ZodString, "many">;
    version: z.ZodString;
    timestamp: z.ZodNumber;
}, "strip", z.ZodTypeAny, {
    entries: Record<string, string | number | boolean | string[] | Record<string, string>>;
    timestamp: number;
    sources: string[];
    version: string;
}, {
    entries: Record<string, string | number | boolean | string[] | Record<string, string>>;
    timestamp: number;
    sources: string[];
    version: string;
}>;
type ConfigSnapshot = z.infer<typeof ConfigSnapshotSchema>;
/**
 * Validation error with context.
 */
declare const ConfigErrorSchema: z.ZodObject<{
    message: z.ZodString;
    path: z.ZodArray<z.ZodString, "many">;
    value: z.ZodUnknown;
    context: z.ZodOptional<z.ZodRecord<z.ZodString, z.ZodUnknown>>;
}, "strip", z.ZodTypeAny, {
    path: string[];
    message: string;
    value?: unknown;
    context?: Record<string, unknown> | undefined;
}, {
    path: string[];
    message: string;
    value?: unknown;
    context?: Record<string, unknown> | undefined;
}>;
type ConfigError = z.infer<typeof ConfigErrorSchema>;
/**
 * Configuration validation error.
 *
 * Following PoLA (Principle of Least Astonishment):
 * - Descriptive error messages
 * - Path to the invalid field
 * - Original value for debugging
 */
declare class ConfigValidationError extends Error {
    readonly path: string[];
    readonly value: unknown;
    readonly context?: Record<string, unknown> | undefined;
    constructor(message: string, path: string[], value: unknown, context?: Record<string, unknown> | undefined);
    toJSON(): ConfigError;
}
/**
 * Config source not found error.
 */
declare class ConfigSourceNotFoundError extends Error {
    readonly source: string;
    readonly context?: Record<string, unknown> | undefined;
    constructor(message: string, source: string, context?: Record<string, unknown> | undefined);
}
/**
 * Immutable configuration snapshot.
 */
declare class ImmutableConfig {
    readonly version: string;
    readonly entries: ReadonlyMap<string, ConfigValue>;
    readonly sources: ReadonlyArray<string>;
    constructor(entries: ReadonlyMap<string, ConfigValue>, sources: ReadonlyArray<string>, version: string);
    get(key: string): ConfigValue | undefined;
    has(key: string): boolean;
    toSnapshot(): ConfigSnapshot;
}

/**
 * Ports (interfaces) for configuration sources.
 *
 * Following hexagonal architecture:
 * - Ports define the interface contract
 * - Adapters implement the ports
 * - Domain depends only on ports
 *
 * xDD Principles:
 * - Interface segregation: focused port interfaces
 * - Dependency inversion: domain depends on abstraction
 */

/**
 * Port interface for configuration sources.
 *
 * Implementations: EnvConfigSource, FileConfigSource, RemoteConfigSource
 */
interface ConfigSource {
    /**
     * Source name for debugging/logging.
     */
    readonly name: string;
    /**
     * Load all config entries from this source.
     */
    load(): Promise<ConfigEntry[]>;
    /**
     * Get a specific config value.
     */
    get(key: string): Promise<ConfigValue | undefined>;
    /**
     * Set a config value (if source is writable).
     */
    set(key: string, value: ConfigValue): Promise<void>;
    /**
     * Check if source is writable.
     */
    isWritable(): boolean;
}
/**
 * Port interface for config validation.
 */
interface ConfigValidator<T = unknown> {
    /**
     * Validate a config value against schema.
     */
    validate(value: unknown): Promise<T>;
    /**
     * Get validation errors.
     */
    getErrors(): ConfigValidationError[];
}

/**
 * Environment variable configuration source adapter.
 *
 * Implements ConfigSource port for environment variables.
 */

/**
 * Environment variable config source.
 *
 * Reads configuration from process.env.
 */
declare class EnvConfigSource implements ConfigSource {
    readonly name = "env";
    private readonly prefix;
    constructor(prefix?: string);
    load(): Promise<ConfigEntry[]>;
    get(key: string): Promise<ConfigValue | undefined>;
    set(key: string, value: ConfigValue): Promise<void>;
    isWritable(): boolean;
    private stripPrefix;
    private parseValue;
}

/**
 * File-based configuration source adapter.
 *
 * Implements ConfigSource port for JSON/YAML config files.
 */

/**
 * File-based config source.
 *
 * Reads configuration from JSON or YAML files.
 */
declare class FileConfigSource implements ConfigSource {
    readonly name = "file";
    private readonly path;
    constructor(path: string);
    load(): Promise<ConfigEntry[]>;
    get(key: string): Promise<ConfigValue | undefined>;
    set(_key: string, _value: ConfigValue): Promise<void>;
    isWritable(): boolean;
}

export { type ConfigEntry, ConfigEntrySchema, type ConfigError, ConfigErrorSchema, type ConfigSnapshot, ConfigSnapshotSchema, type ConfigSource, ConfigSourceNotFoundError, ConfigValidationError, type ConfigValidator, type ConfigValue, ConfigValueSchema, EnvConfigSource, FileConfigSource, ImmutableConfig };

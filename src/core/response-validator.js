/**
 * Response validation helper
 * Validates tool responses against Zod schemas and wraps in MCP TextContent format
 */
import { createLogger } from './logger.js';

const logger = createLogger('response-validator');

/**
 * Validate response data against a Zod schema and wrap in MCP TextContent format
 *
 * @param {import('zod').ZodSchema} schema - Zod schema to validate against
 * @param {Object} data - Response data to validate
 * @param {Object} options - Validation options
 * @param {boolean} options.strict - If true, throws on validation errors (default: true)
 * @param {string} options.context - Context string for error logging
 * @returns {Object} MCP TextContent response object
 * @throws {Error} If validation fails and strict mode is enabled
 *
 * @example
 * ```js
 * import { validateResponse } from '../../core/response-validator.js';
 * import { AgentResponseSchema } from '../schemas/response-schemas.js';
 *
 * return validateResponse(AgentResponseSchema, {
 *   success: true,
 *   operation: 'create',
 *   agent_id: 'agent-123',
 *   message: 'Agent created successfully'
 * });
 * ```
 */
export function validateResponse(schema, data, options = {}) {
    const { strict = true, context = 'response' } = options;

    try {
        // Validate data against schema
        const validatedData = schema.parse(data);

        // Wrap in MCP TextContent format
        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify(validatedData, null, 0),
                },
            ],
        };
    } catch (error) {
        // Log validation error with context
        logger.error(`Response validation failed for ${context}`, {
            error: error.message,
            issues: error.errors || [],
            data,
        });

        if (strict) {
            // In strict mode, throw the validation error
            throw new Error(
                `Response validation failed for ${context}: ${error.message}\n` +
                    `Issues: ${JSON.stringify(error.errors || [], null, 2)}`,
            );
        }

        // In non-strict mode, return the unvalidated data wrapped in MCP format
        logger.warn(`Continuing with unvalidated data for ${context}`);
        return {
            content: [
                {
                    type: 'text',
                    text: JSON.stringify(data, null, 0),
                },
            ],
        };
    }
}

/**
 * Validate response data against a Zod schema without wrapping
 * Useful for intermediate validation steps
 *
 * @param {import('zod').ZodSchema} schema - Zod schema to validate against
 * @param {Object} data - Response data to validate
 * @param {string} context - Context string for error logging
 * @returns {Object} Validated data
 * @throws {Error} If validation fails
 */
export function validateData(schema, data, context = 'data') {
    try {
        return schema.parse(data);
    } catch (error) {
        logger.error(`Data validation failed for ${context}`, {
            error: error.message,
            issues: error.errors || [],
            data,
        });

        throw new Error(
            `Data validation failed for ${context}: ${error.message}\n` +
                `Issues: ${JSON.stringify(error.errors || [], null, 2)}`,
        );
    }
}

/**
 * Create a validated error response
 *
 * @param {string} operation - Operation that failed
 * @param {Error} error - Error object
 * @param {Object} additionalData - Additional data to include in error response
 * @returns {Object} MCP TextContent error response
 */
export function createErrorResponse(operation, error, additionalData = {}) {
    const errorData = {
        success: false,
        operation,
        message: error.message || 'Operation failed',
        error: {
            name: error.name,
            message: error.message,
            stack: process.env.NODE_ENV === 'development' ? error.stack : undefined,
        },
        ...additionalData,
    };

    return {
        content: [
            {
                type: 'text',
                text: JSON.stringify(errorData, null, 0),
            },
        ],
    };
}

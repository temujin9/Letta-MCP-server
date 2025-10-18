/**
 * Zod schemas for validating tool responses
 * These provide type safety and runtime validation without exposing outputSchema to MCP
 */
import { z } from 'zod';

/**
 * Base response schema - common fields for all responses
 */
export const BaseResponseSchema = z.object({
    success: z.boolean().describe('Whether the operation succeeded'),
    operation: z.string().describe('Operation that was performed'),
    message: z.string().optional().describe('Status or error message'),
});

/**
 * MCP Server Operations Responses
 */
export const McpServerResponseSchema = BaseResponseSchema.extend({
    server_name: z.string().optional().describe('Name of the MCP server'),
    server_config: z.any().optional().describe('Server configuration'),
    servers: z.array(z.object({
        name: z.string(),
        type: z.string(),
        status: z.string(),
    })).optional().describe('List of servers'),
    test_result: z.object({
        connected: z.boolean(),
        latency_ms: z.number().optional(),
        error: z.string().optional(),
    }).optional().describe('Test connection result'),
    execution_result: z.any().optional().describe('Result from tool execution'),
    oauth_url: z.string().url().optional().describe('OAuth authorization URL'),
    tool_id: z.string().optional().describe('Registered tool ID'),
    tool: z.any().optional().describe('Tool details'),
});

/**
 * Agent Operations Responses
 */
export const AgentResponseSchema = BaseResponseSchema.extend({
    agent_id: z.string().optional().describe('Agent identifier'),
    agent: z.any().optional().describe('Agent details'),
    agents: z.array(z.any()).optional().describe('List of agents'),
    tools: z.array(z.any()).optional().describe('Agent tools'),
    messages: z.array(z.any()).optional().describe('Agent messages'),
    message_id: z.string().optional().describe('Message identifier'),
    context: z.any().optional().describe('Agent context'),
    summary: z.string().optional().describe('Conversation summary'),
    count: z.number().optional().describe('Count of items'),
    file_path: z.string().optional().describe('Exported file path'),
});

/**
 * Memory Operations Responses
 */
export const MemoryResponseSchema = BaseResponseSchema.extend({
    block_id: z.string().optional().describe('Memory block identifier'),
    block: z.any().optional().describe('Memory block details'),
    blocks: z.array(z.any()).optional().describe('List of memory blocks'),
    core_memory: z.any().optional().describe('Core memory state'),
    passage_id: z.string().optional().describe('Passage identifier'),
    passage: z.any().optional().describe('Passage details'),
    passages: z.array(z.any()).optional().describe('List of passages'),
    results: z.array(z.any()).optional().describe('Search results'),
    agents_using_block: z.array(z.string()).optional().describe('Agents using this block'),
});

/**
 * Tool Manager Operations Responses
 */
export const ToolManagerResponseSchema = BaseResponseSchema.extend({
    tool_id: z.string().optional().describe('Tool identifier'),
    tool: z.any().optional().describe('Tool details'),
    tools: z.array(z.any()).optional().describe('List of tools'),
    count: z.number().optional().describe('Count of tools'),
    schema: z.any().optional().describe('Generated tool schema'),
    result: z.any().optional().describe('Tool execution result'),
    attached_to: z.array(z.string()).optional().describe('Agents tool is attached to'),
    generated_code: z.string().optional().describe('Generated tool code'),
});

/**
 * Source Manager Operations Responses
 */
export const SourceManagerResponseSchema = BaseResponseSchema.extend({
    source_id: z.string().optional().describe('Source identifier'),
    source: z.any().optional().describe('Source details'),
    sources: z.array(z.any()).optional().describe('List of sources'),
    count: z.number().optional().describe('Count of sources'),
    file_id: z.string().optional().describe('File identifier'),
    file: z.any().optional().describe('File details'),
    files: z.array(z.any()).optional().describe('List of files'),
    passages: z.array(z.any()).optional().describe('Source passages'),
    metadata: z.any().optional().describe('Source metadata'),
});

/**
 * Job Monitor Operations Responses
 */
export const JobMonitorResponseSchema = BaseResponseSchema.extend({
    job_id: z.string().optional().describe('Job identifier'),
    job: z.any().optional().describe('Job details'),
    jobs: z.array(z.any()).optional().describe('List of jobs'),
    status: z.string().optional().describe('Job status'),
    progress: z.number().optional().describe('Job progress percentage'),
    cancelled: z.boolean().optional().describe('Whether job was cancelled'),
});

/**
 * File/Folder Operations Responses
 */
export const FileFolderResponseSchema = BaseResponseSchema.extend({
    file_id: z.string().optional().describe('File identifier'),
    file: z.any().optional().describe('File details'),
    files: z.array(z.any()).optional().describe('List of files'),
    folder_id: z.string().optional().describe('Folder identifier'),
    folder: z.any().optional().describe('Folder details'),
    folders: z.array(z.any()).optional().describe('List of folders'),
    agents: z.array(z.string()).optional().describe('Agents in folder'),
    session_id: z.string().optional().describe('File session ID'),
});

/**
 * Model/Prompts Responses
 */
export const ModelResponseSchema = z.object({
    models: z.array(z.any()).describe('List of available models'),
});

export const PromptResponseSchema = z.object({
    prompts: z.array(z.any()).describe('List of available prompts'),
});

export const PromptExecutionResponseSchema = BaseResponseSchema.extend({
    result: z.any().optional().describe('Prompt execution result'),
    prompt_name: z.string().optional().describe('Executed prompt name'),
});

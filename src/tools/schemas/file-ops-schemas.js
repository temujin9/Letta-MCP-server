/**
 * Schemas for letta_file_folder_ops tool
 * Provides discriminator-based schemas for file and folder management
 */

/**
 * Input schema for letta_file_folder_ops tool
 */
export const fileOpsInputSchema = {
    type: 'object',
    properties: {
        operation: {
            type: 'string',
            enum: [
                'list_files',
                'open_file',
                'close_file',
                'close_all_files',
                'list_folders',
                'attach_folder',
                'detach_folder',
                'list_agents_in_folder',
            ],
            description: 'File/folder operation to perform',
        },
        agent_id: {
            type: 'string',
            description: 'Agent ID (required for agent-specific operations)',
        },
        file_id: {
            type: 'string',
            description: 'File ID (required for open_file, close_file)',
        },
        folder_id: {
            type: 'string',
            description: 'Folder ID (required for attach/detach/list_agents_in_folder)',
        },
        request_heartbeat: {
            type: 'boolean',
            description: 'Ignored parameter (for MCP client compatibility)',
        },
    },
    required: ['operation'],
    additionalProperties: true,
};

/**
 * File metadata schema
 */
export const FileMetadataSchema = {
    type: 'object',
    properties: {
        id: { type: 'string' },
        filename: { type: 'string' },
        size: { type: 'integer' },
        mime_type: { type: 'string' },
        is_open: { type: 'boolean' },
        opened_at: { type: 'string' },
    },
    required: ['id', 'filename'],
    additionalProperties: false,
};

/**
 * Folder metadata schema
 */
export const FolderMetadataSchema = {
    type: 'object',
    properties: {
        id: { type: 'string' },
        name: { type: 'string' },
        path: { type: 'string' },
        file_count: { type: 'integer' },
        agent_count: { type: 'integer' },
    },
    required: ['id', 'name'],
    additionalProperties: false,
};

/**
 * Agent reference schema
 */
export const AgentReferenceSchema = {
    type: 'object',
    properties: {
        id: { type: 'string' },
        name: { type: 'string' },
    },
    required: ['id', 'name'],
    additionalProperties: false,
};


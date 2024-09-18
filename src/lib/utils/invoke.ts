import { invoke } from '@tauri-apps/api/tauri'

async function invokeSshCommand<T>(
    command: SshCommand,
): Promise<SshCommandResult<T>> {
    return await invoke<SshCommandResult<T>>('handle_ssh_command', { command })
}

async function invokeDbOperation<T>(
    operation: DbOperation,
): Promise<DbOperationResult<T>> {
    return await invoke<DbOperationResult<T>>('handle_db_operation', {
        operation,
    })
}

export { invokeSshCommand, invokeDbOperation }

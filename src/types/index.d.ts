interface Connection {
    id: number
    name: string
    host: string
    port: number
    username?: string
    password?: string
    connected?: boolean
    sessionId?: number
}

interface Response<T> {
    code: number
    message: string
    data: T
}

// SSH 命令类型
type SshCommandResult<T> = Response<T>

type OpenConnectionCommand = {
    OpenConnection: string
}

type CloseConnectionCommand = {
    CloseConnection: number
}

type ExecuteQueryCommand = {
    ExecuteQuery: {
        connection_id: number
        query: string
    }
}

type SshCommand =
    | OpenConnectionCommand
    | CloseConnectionCommand
    | ExecuteQueryCommand

// 添加一个辅助函数类型
type InvokeSshCommand = (command: SshCommand) => Promise<SshCommandResult>

// 数据库操作类型
type DbOperationResult<T> = Response<T>

type InsertOperation = { Insert: Connection }
type UpdateOperation = { Update: Connection }
type DeleteOperation = { Delete: number }
type SelectAllOperation = 'SelectAll'

type DbOperation =
    | InsertOperation
    | UpdateOperation
    | DeleteOperation
    | SelectAllOperation

// 添加一个辅助函数类型
type InvokeDbOperation = (operation: DbOperation) => Promise<DbOperationResult>

interface TauriError extends Error {
    __tauriModule: 'Error'
}

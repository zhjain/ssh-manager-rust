interface Connection {
    id: number
    name: string
    host: string
    port: number
    username?: string
    password?: string
    connected?: boolean
}

interface ServerStatus {
    status: string
    uptime: string
    cpu_usage: number
    memory_usage: number
    memory_usages?: number[]
    upload_speed: number
    download_speed: number
    uploadSpeedHistory?: number[]
    downloadSpeedHistory?: number[]
    network_usage: {
        rx_speed: number
        tx_speed: number
    }
    disk_usage: {
        used: string
        available: string
        total: string
        use_percentage: string
    }
}

interface Response<T> {
    code: number
    message: string
    data: T
}

// SSH 命令类型
type SshCommandResult<T> = Response<T>

type OpenConnectionCommand = {
    OpenConnection: {
        id: number
        url: string
    }
}

type CloseConnectionCommand = {
    CloseConnection: number
}

type ExecuteQueryCommand = {
    ExecuteQuery: {
        id: number
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

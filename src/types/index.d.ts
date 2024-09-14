interface Connection {
    id: number
    name: string
    host: string
    port: number
    username?: string
    password?: string
    connected?: boolean
}

interface Response<T> {
    code: number
    message: string
    data: T
}

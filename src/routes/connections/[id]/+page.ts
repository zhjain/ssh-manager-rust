export const prerender = 'auto'
import { invokeSshCommand } from '$lib/utils'
import type { PageLoad } from './$types'
import { get } from 'svelte/store'

import connectionStore from '$lib/store/connectionStore'

export const load: PageLoad = async () => {
    try {
        const cachedConnections = get(connectionStore)
        if (
            !cachedConnections.current ||
            !cachedConnections.current.sessionId
        ) {
            throw new Error('无效的连接或会话ID')
        }
        const command: SshCommand = {
            ExecuteQuery: {
                connection_id: cachedConnections.current.sessionId,
                // query: "uptime && free -m && top -bn1 | grep 'Cpu(s)'",
                query: 'ls',
            },
        }

        const response = await invokeSshCommand<{
            status: string
            uptime: string
            cpu_usage: number
            memory_usage: number
        }>(command)

        return {
            connectionStatus: response.data,
        }
    } catch {
        return {
            connectionStatus: null,
            error: '无法获取服务器状态',
        }
    }
}

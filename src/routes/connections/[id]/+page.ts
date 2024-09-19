export const prerender = 'auto'
import { invokeSshCommand } from '$lib/utils'
import type { PageLoad } from './$types'
import { get } from 'svelte/store'

import connectionStore from '$lib/store/connectionStore'

export const load: PageLoad = async () => {
    try {
        const cachedConnections = get(connectionStore)
        if (cachedConnections.serverStatus[cachedConnections.current.id])
            return {
                connectionStatus:
                    cachedConnections.serverStatus[
                        cachedConnections.current.id
                    ],
                error: null,
            }
        if (!cachedConnections.current || !cachedConnections.current.id) {
            throw new Error('无效的连接或会话ID')
        }
        const command: SshCommand = {
            ExecuteQuery: {
                id: cachedConnections.current.id,
                // query: "uptime && free -m && top -bn1 | grep 'Cpu(s)'",
                query: 'baseinfo',
            },
        }

        const response = await invokeSshCommand<ServerStatus>(command)
        cachedConnections.serverStatus[cachedConnections.current.id] =
            response.data
        return {
            connectionStatus: response.data,
            error: null,
        }
    } catch {
        return {
            connectionStatus: null,
            error: '无法获取服务器状态',
        }
    }
}

import { get } from 'svelte/store'
import { invoke } from '@tauri-apps/api/tauri'

import connectionStore from '$lib/store/connectionStore'

export async function load() {
    const cachedConnections = get(connectionStore)

    if (cachedConnections.all.length > 0) {
        // 如果有缓存的数据，直接返回
        return { list: cachedConnections.all }
    }
    try {
        const res = await invoke<Response<Connection[]>>(
            'handle_db_operation',
            {
                operation: 'SelectAll',
            },
        )
        connectionStore.update(c => ({ ...c, all: res.data }))
        return { list: res.data }
    } catch {
        return { list: [] }
    }
}

import { get } from 'svelte/store'

import connectionStore from '$lib/store/connectionStore'
import { invokeDbOperation } from '$lib/utils'

export async function load() {
    const cachedConnections = get(connectionStore)

    if (cachedConnections.all.length > 0) {
        // 如果有缓存的数据，直接返回
        return { list: cachedConnections.all }
    }
    try {
        const res = await invokeDbOperation<Connection[]>('SelectAll')
        connectionStore.update(c => ({ ...c, all: res.data }))
        return { list: res.data }
    } catch {
        return { list: [] }
    }
}

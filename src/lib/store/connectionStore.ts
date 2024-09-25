import { writable } from 'svelte/store'

interface ConnectionStore {
    all: Connection[]
    connected: Connection[]
    selecting: Connection
    current: Connection | { id: string }
    serverStatus: Record<number, ServerStatus>
}

const connectionStore = writable<ConnectionStore>({
    all: [],
    connected: [],
    selecting: {} as Connection,
    current: {} as Connection,
    serverStatus: {},
})

export default connectionStore

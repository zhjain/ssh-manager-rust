import { writable } from 'svelte/store'

interface ConnectionStore {
    all: Connection[]
    connected: Connection[]
    selecting: Connection
    current: Connection
}

const connectionStore = writable<ConnectionStore>({
    all: [],
    connected: [],
    selecting: {} as Connection,
    current: {} as Connection,
})

export default connectionStore

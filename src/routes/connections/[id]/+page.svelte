<script lang="ts">
    import { listen } from "@tauri-apps/api/event"
    import { onMount } from "svelte"
    import { page } from "$app/stores"
    import connectionStore from "$lib/store/connectionStore"
    import { NetworkSpeedChart } from "$lib/components"

    // 获取路由参数
    let id = Number($page.params.id)

    let connectionStatus: {
        status: string
        uptime: string
        cpu_usage: number
        memory_usage: number
        upload_speed: number
        download_speed: number
    } | null = null

    let error: string | null = null

    onMount(() => {
        connectionStatus = $connectionStore.serverStatus[id]
        listenToConnection(id)
    })

    function listenToConnection(connectionId: number) {
        const listeners = [
            listen<{ uptime: string }>(
                `server-uptime-update-${connectionId}`,
                event => {
                    $connectionStore.serverStatus[connectionId] = {
                        ...$connectionStore.serverStatus[connectionId],
                        uptime: event.payload.uptime,
                    }
                    if ($connectionStore.current.id === connectionId) {
                        connectionStatus =
                            $connectionStore.serverStatus[connectionId]
                    }
                },
            ),
            listen<{
                memory_usage: { total: number; used: number; free: number }
            }>(`server-memory-update-${connectionId}`, event => {
                $connectionStore.serverStatus[connectionId] = {
                    ...$connectionStore.serverStatus[connectionId],
                    memory_usage:
                        (event.payload.memory_usage.used /
                            event.payload.memory_usage.total) *
                        100,
                }
                if ($connectionStore.current.id === connectionId) {
                    connectionStatus =
                        $connectionStore.serverStatus[connectionId]
                }
            }),
            listen<{
                cpu_usage: { user: number; system: number; total: number }
            }>(`server-cpu-update-${connectionId}`, event => {
                $connectionStore.serverStatus[connectionId] = {
                    ...$connectionStore.serverStatus[connectionId],
                    cpu_usage: event.payload.cpu_usage.total,
                }
                if ($connectionStore.current.id === connectionId) {
                    connectionStatus =
                        $connectionStore.serverStatus[connectionId]
                }
            }),
            listen<{ network_usage: { rx_speed: number; tx_speed: number } }>(
                `server-network-update-${connectionId}`,
                event => {
                    $connectionStore.serverStatus[connectionId] = {
                        ...$connectionStore.serverStatus[connectionId],
                        upload_speed: event.payload.network_usage.tx_speed,
                        download_speed: event.payload.network_usage.rx_speed,
                        uploadSpeedHistory: [
                            ...($connectionStore.serverStatus[connectionId]
                                .uploadSpeedHistory || []),
                            event.payload.network_usage.tx_speed,
                        ],
                        downloadSpeedHistory: [
                            ...($connectionStore.serverStatus[connectionId]
                                .downloadSpeedHistory || []),
                            event.payload.network_usage.rx_speed,
                        ],
                    }
                    if ($connectionStore.current.id === connectionId) {
                        connectionStatus =
                            $connectionStore.serverStatus[connectionId]
                    }
                },
            ),
        ]

        return () => {
            listeners.forEach(async unlistenPromise => {
                const unlisten = await unlistenPromise
                unlisten()
            })
        }
    }
</script>

<div>
    <div class="border-b">
        {connectionStatus?.status || ""}
    </div>
    <div class="border-b">
        {connectionStatus?.uptime || ""}
    </div>
    <div class="border-b">
        CPU使用率: {connectionStatus?.cpu_usage || ""}%
    </div>
    <div class="border-b">
        内存使用率: {connectionStatus?.memory_usage || ""}%
    </div>
    <div class="border-b">
        上传速率: {connectionStatus?.upload_speed || ""} MB/s
    </div>
    <div class="border-b">
        下载速率: {connectionStatus?.download_speed || ""} MB/s
    </div>
    <div class="w-80 max-w-2xl mx-auto mt-8 h-48">
        <NetworkSpeedChart connectionId="{id}" />
    </div>

    {error}
</div>

<style>
</style>

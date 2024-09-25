<script lang="ts">
    import { listen } from "@tauri-apps/api/event"
    import { onMount } from "svelte"
    import { page } from "$app/stores"
    import connectionStore from "$lib/store/connectionStore"
    import { formatBytes } from "$lib/utils"
    // import { NetworkSpeedChart } from "$lib/components"

    // 获取路由参数
    let id = Number($page.params.id)

    let connectionStatus: ServerStatus | null = null

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
            listen<{
                disk_usage: {
                    used: string
                    available: string
                    total: string
                    use_percentage: string
                }
            }>(`server-disk-update-${connectionId}`, event => {
                $connectionStore.serverStatus[connectionId] = {
                    ...$connectionStore.serverStatus[connectionId],
                    disk_usage: event.payload.disk_usage,
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

<div
    class="grid grid-cols-2 gap-4 p-4 z-10 bg-white rounded-lg shadow-md w-full">
    <div class="bg-gray-100 p-4 rounded-md shadow">
        <span class="text-sm text-gray-600 mb-2 block">磁盘使用情况：</span>
        <span class="text-lg font-bold text-gray-800">
            {#if connectionStatus?.disk_usage}
                {connectionStatus.disk_usage.used} / {connectionStatus
                    .disk_usage.total} ({connectionStatus.disk_usage
                    .use_percentage})
            {:else}
                0G / 0G (0%)
            {/if}
        </span>
    </div>
    <div class="bg-gray-50 p-4 rounded-md shadow">
        <span class="text-sm text-gray-600 mb-2 block">运行时间：</span>
        <span class="text-lg font-bold text-gray-800"
            >{connectionStatus?.uptime || "00:00:00 up 0 days"}</span>
    </div>
    <div class="bg-gray-50 p-4 rounded-md shadow">
        <span class="text-sm text-gray-600 mb-2 block">CPU使用率：</span>
        <span class="text-lg font-bold text-gray-800"
            >{connectionStatus?.cpu_usage?.toFixed(2) || "0.00"}%</span>
    </div>
    <div class="bg-gray-50 p-4 rounded-md shadow">
        <span class="text-sm text-gray-600 mb-2 block">内存使用率：</span>
        <span class="text-lg font-bold text-gray-800"
            >{connectionStatus?.memory_usage?.toFixed(2) || "0.00"}%</span>
    </div>
    <div class="bg-gray-50 p-4 rounded-md shadow">
        <span class="text-sm text-gray-600 mb-2 block">上传速率：</span>
        <span class="text-lg font-bold text-gray-800"
            >{formatBytes(connectionStatus?.upload_speed || 0)}</span>
    </div>
    <div class="bg-gray-50 p-4 rounded-md shadow">
        <span class="text-sm text-gray-600 mb-2 block">下载速率：</span>
        <span class="text-lg font-bold text-gray-800"
            >{formatBytes(connectionStatus?.download_speed || 0)}</span>
    </div>
</div>

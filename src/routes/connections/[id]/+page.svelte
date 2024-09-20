<script lang="ts">
    import { listen } from "@tauri-apps/api/event"

    import { onMount } from "svelte"
    import { page } from "$app/stores"

    import connectionStore from "$lib/store/connectionStore"
    // 获取路由参数
    let id = Number($page.params.id)

    let connectionStatus: {
        status: string
        uptime: string
        cpu_usage: number
        memory_usage: number
    } | null = null

    let error: string | null = null

    onMount(() => {
        connectionStatus = $connectionStore.serverStatus[id]
        // error = data.error
        // 监听某个 SSH 连接的事件
        listenToConnection(id)
    })

    function listenToConnection(connectionId: number) {
        listen<Response<ServerStatus>>(
            `server-status-update-${connectionId}`,
            event => {
                // 保存接收到的服务器状态
                $connectionStore.serverStatus[connectionId] = event.payload.data

                // 如果是当前显示的连接，更新UI
                if ($connectionStore.current.id === connectionId) {
                    connectionStatus = event.payload.data
                }
            },
        )
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
        {JSON.stringify(connectionStatus?.cpu_usage) || ""}
    </div>
    <div class="border-b">
        {JSON.stringify(connectionStatus?.memory_usage) || ""}
    </div>

    {error}
</div>

<style>
</style>

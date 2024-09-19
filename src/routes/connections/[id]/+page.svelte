<script lang="ts">
    // import { page } from "$app/stores"
    // import connectionStore from "$lib/store/connectionStore"
    // 获取路由参数
    // let { id } = $page.params

    import { onMount } from "svelte"
    import type { PageData } from "./$types"

    export let data: PageData

    let connectionStatus: {
        status: string
        uptime: string
        cpu_usage: number
        memory_usage: number
    } | null = null

    let error: string | null = null

    onMount(() => {
        connectionStatus = data.connectionStatus
        error = data.error
    })
</script>

<div>
    <div class="border-b">
        {connectionStatus?.status}
    </div>
    <div class="border-b">
        {connectionStatus?.uptime}
    </div>
    <div class="border-b">
        {connectionStatus?.cpu_usage}
    </div>
    <div class="border-b">
        {connectionStatus?.memory_usage}
    </div>

    {error}
</div>

<style>
</style>

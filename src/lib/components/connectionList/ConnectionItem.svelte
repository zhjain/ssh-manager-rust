<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import { goto } from "$app/navigation"

    import connectionStore from "$lib/store/connectionStore"
    import { cn } from "$lib/utils"

    export let connection: Connection

    // 局部变量, 用于连接状态的缓存, 避免在建立连接时, 连接列表的样式重新渲染
    let connected = connection.connected

    $: isCurrent = $connectionStore.selecting.id === connection.id

    async function goConnect() {
        if (connection.connected) {
            await goto(`/connections/${connection.id}`)
            $connectionStore.current = connection
        } else {
            console.log(connection.host+connection.port);
            await invoke("connection", { url: `${connection.host}:${connection.port}` })
            await goto(`/connections/${connection.id}`)
            $connectionStore.current = connection
            connection.connected = true
        }
    }
    function disConnect() {
        connection.connected = false
        connected = false
    }
    function setCurrent() {
        $connectionStore.selecting = connection
    }
    function deleteConnection() {
        $connectionStore.all = $connectionStore.all.filter(
            c => c.id !== connection.id,
        )
        if ($connectionStore.selecting.id === connection.id) {
            $connectionStore.selecting = $connectionStore.all[0]
        }
    }
</script>

<button
    on:click="{setCurrent}"
    on:dblclick="{goConnect}"
    class="{cn('flex flex-row w-full justify-between items-center px-4', {
        'bg-red-200 transition-all': isCurrent,
    })}">
    <div class="flex gap-2 items-center">
        <div
            class="{cn(
                'w-8 h-8 my-2 rounded-full',
                connected ? 'bg-red-400' : 'bg-gray-200',
            )}">
        </div>
        <div class="text-lg font-bold">{connection.name}</div>
    </div>
    {#if isCurrent}
        <div class="flex gap-2 items-center">
            {#if connection.connected}
                <button on:click="{disConnect}" class="text-gray-500 text-sm"
                    >断开</button>
            {:else}
                <button on:click="{goConnect}" class="text-gray-500 text-sm"
                    >连接</button>
            {/if}

            <button class="text-gray-500 text-sm">编辑</button>
            <button on:click="{deleteConnection}" class="text-gray-500 text-sm"
                >删除</button>
        </div>
    {/if}
</button>

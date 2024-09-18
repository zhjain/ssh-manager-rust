<!-- svelte-ignore a11y-click-events-have-key-events -->
<script lang="ts">
    import { goto } from "$app/navigation"

    import connectionStore from "$lib/store/connectionStore"
    import { cn, invokeDbOperation, invokeSshCommand } from "$lib/utils"

    export let connection: Connection

    // 使用 $: 响应式声明来保持连接状态的同步
    $: connected = connection.connected

    $: isCurrent = $connectionStore.selecting.id === connection.id

    async function goConnect() {
        if (connection.connected) {
            await goto(`/connections/${connection.id}`)
            $connectionStore.current = connection
        } else {
            const command: SshCommand = {
                OpenConnection: `${connection.username}:${connection.password}@${connection.host}:${connection.port}`,
            }
            const sshRes = await invokeSshCommand<{
                id: number
                message: string
            }>(command)
            await goto(`/connections/${connection.id}`)
            $connectionStore.current = connection
            connection.sessionId = sshRes.data.id
            connection.connected = true
        }
    }
    async function disConnect() {
        const command: SshCommand = {
            CloseConnection: connection.sessionId!,
        }
        await invokeSshCommand<{
            id: number
            message: string
        }>(command)
        connection.connected = false
        connection.sessionId = undefined
        // connected = false
    }
    function setCurrent() {
        $connectionStore.selecting = connection
    }
    async function deleteConnection() {
        if (connected) {
            const command: SshCommand = {
                CloseConnection: connection.sessionId!,
            }
            await invokeSshCommand<{
                id: number
                message: string
            }>(command)
            connection.connected = false
            connection.sessionId = undefined
            // connected = false
        }
        const dbCommand: DbOperation = {
            Delete: connection.id,
        }
        await invokeDbOperation<Connection>(dbCommand)
        $connectionStore.all = $connectionStore.all.filter(
            c => c.id !== connection.id,
        )
        if ($connectionStore.selecting.id === connection.id) {
            $connectionStore.selecting = $connectionStore.all[0]
        }
    }
</script>

<div
    on:click="{setCurrent}"
    on:dblclick="{goConnect}"
    class="{cn(
        'flex flex-row w-full justify-between cursor-pointer items-center px-4',
        {
            'bg-red-200 transition-all': isCurrent,
        },
    )}">
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
                <div on:click="{disConnect}" class="text-gray-500 text-sm">
                    断开
                </div>
            {:else}
                <div on:click="{goConnect}" class="text-gray-500 text-sm">
                    连接
                </div>
            {/if}

            <div class="text-gray-500 text-sm">编辑</div>
            <div on:click="{deleteConnection}" class="text-gray-500 text-sm">
                删除
            </div>
        </div>
    {/if}
</div>

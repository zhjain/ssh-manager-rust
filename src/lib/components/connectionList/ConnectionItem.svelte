<!-- svelte-ignore a11y-click-events-have-key-events -->
<script lang="ts">
    import { goto } from "$app/navigation"
    import { toast } from "@zerodevx/svelte-toast"

    import connectionStore from "$lib/store/connectionStore"
    import { cn, invokeDbOperation, invokeSshCommand } from "$lib/utils"

    export let connection: Connection

    // 使用 $: 响应式声明来保持连接状态的同步
    $: connected = connection.connected

    $: isCurrent = $connectionStore.selecting.id === connection.id

    let showEditDialog = false
    let editFormData: Connection

    async function goConnect() {
        if (connection.connected) {
            await goto(`/connections/${connection.id}`)
            $connectionStore.current = connection
        } else {
            try {
                const command: SshCommand = {
                    OpenConnection: `${connection.username}:${connection.password}@${connection.host}:${connection.port}`,
                }
                const sshRes = await invokeSshCommand<{
                    id: number
                    message: string
                }>(command)
                $connectionStore.current = connection
                connection.sessionId = sshRes.data.id
                await goto(`/connections/${connection.id}`)
                connection.connected = true
            } catch (error) {
                toast.push(error as string)
            }
        }
    }

    async function disConnect() {
        try {
            const command: SshCommand = {
                CloseConnection: connection.sessionId!,
            }
            await invokeSshCommand<{
                id: number
                message: string
            }>(command)
            connection.connected = false
            connection.sessionId = undefined
        } catch (error) {
            toast.push(`断开连接失败: ${(error as Error).message}`)
        }
    }

    function setCurrent() {
        $connectionStore.selecting = connection
    }

    async function deleteConnection() {
        if (connected) {
            try {
                const command: SshCommand = {
                    CloseConnection: connection.sessionId!,
                }
                await invokeSshCommand<{
                    id: number
                    message: string
                }>(command)
                connection.connected = false
                connection.sessionId = undefined
            } catch (error) {
                toast.push(`断开连接失败: ${(error as Error).message}`)
                return
            }
        }
        try {
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
        } catch (error) {
            toast.push(`删除连接失败: ${(error as Error).message}`)
        }
    }

    function openEditDialog() {
        editFormData = { ...connection }
        showEditDialog = true
    }

    async function handleEditSubmit() {
        if (validateEditForm()) {
            const dbCommand: DbOperation = {
                Update: editFormData,
            }
            try {
                await invokeDbOperation<Connection>(dbCommand)
                Object.assign(connection, editFormData)
                $connectionStore.all = $connectionStore.all.map(c =>
                    c.id === connection.id ? connection : c,
                )
                showEditDialog = false
                toast.push("连接信息更新成功")
            } catch (error) {
                toast.push("更新失败: " + (error as Error).message)
            }
        }
    }

    function validateEditForm() {
        if (!editFormData.name) {
            toast.push("连接名称不能为空")
            return false
        }
        if (!editFormData.host) {
            toast.push("主机地址不能为空")
            return false
        }
        if (
            !editFormData.port ||
            editFormData.port < 1 ||
            editFormData.port > 65535
        ) {
            toast.push("端口号必须是1到65535之间的整数")
            return false
        }
        if (!editFormData.username) {
            toast.push("用户名不能为空")
            return false
        }
        if (!editFormData.password) {
            toast.push("密码不能为空")
            return false
        }
        return true
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

            <div on:click="{openEditDialog}" class="text-gray-500 text-sm">
                编辑
            </div>
            <div on:click="{deleteConnection}" class="text-gray-500 text-sm">
                删除
            </div>
        </div>
    {/if}
</div>

{#if showEditDialog}
    <div
        class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
        <div class="bg-white rounded-lg shadow-xl p-6 w-96 max-w-full">
            <h2 class="text-2xl font-bold mb-6 text-gray-800">编辑SSH连接</h2>
            <div class="mb-4">
                <span class="block text-sm font-medium text-gray-700 mb-1"
                    >连接名称</span>
                <input
                    class="form-input w-full"
                    type="text"
                    bind:value="{editFormData.name}"
                    placeholder="请输入连接名称" />
            </div>
            <div class="mb-4">
                <span class="block text-sm font-medium text-gray-700 mb-1"
                    >主机</span>
                <input
                    class="form-input w-full"
                    type="text"
                    bind:value="{editFormData.host}"
                    placeholder="请输入主机地址" />
            </div>
            <div class="mb-4">
                <span class="block text-sm font-medium text-gray-700 mb-1"
                    >端口</span>
                <input
                    class="form-input w-full"
                    type="number"
                    bind:value="{editFormData.port}"
                    placeholder="请输入端口号" />
            </div>
            <div class="mb-4">
                <span class="block text-sm font-medium text-gray-700 mb-1"
                    >用户名</span>
                <input
                    class="form-input w-full"
                    type="text"
                    bind:value="{editFormData.username}"
                    placeholder="请输入用户名" />
            </div>
            <div class="mb-4">
                <span class="block text-sm font-medium text-gray-700 mb-1"
                    >密码</span>
                <input
                    class="form-input w-full"
                    type="password"
                    bind:value="{editFormData.password}"
                    placeholder="请输入密码" />
            </div>
            <div class="flex justify-end space-x-3 mt-6">
                <button
                    type="button"
                    class="px-4 py-2 text-gray-600 bg-gray-100 hover:bg-gray-200 rounded-md transition-colors"
                    on:click="{() => (showEditDialog = false)}">
                    取消
                </button>
                <button
                    on:click="{handleEditSubmit}"
                    class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors">
                    保存修改
                </button>
            </div>
        </div>
    </div>
{/if}

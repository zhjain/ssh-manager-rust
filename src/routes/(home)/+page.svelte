<script lang="ts">
    import { toast } from "@zerodevx/svelte-toast"
    import { Loader } from "lucide-svelte"

    // import { goto } from "$app/navigation"

    import Dialog from "$lib/components/common/Dialog.svelte"
    import connectionStore from "$lib/store/connectionStore"
    import { invokeDbOperation } from "$lib/utils"

    function initializeForm() {

        return {
            name: "",
            host: "",
            port: 22,
            username: "",
            password: "",
        }
    }
    let formData = initializeForm()

    let isValid = false
    let isLoading = false

    function validateForm() {
        if (!formData.name) {
            toast.push("创建失败, 连接名称不能为空")
            return false
        }
        // 从store中检查名称是否已存在
        const existingConnection = $connectionStore.all.find(
            conn => conn.name === formData.name,
        )
        if (existingConnection) {
            toast.push("创建失败, 连接名称已存在")
            return false
        }
        if (!formData.host) {
            toast.push("创建失败, 主机地址不能为空")
            return false
        }
        if (!formData.port || formData.port < 1 || formData.port > 65535) {
            toast.push("创建失败, 端口号必须是1到65535之间的整数")
            return false
        }
        if (!formData.username) {
            toast.push("创建失败, 用户名不能为空")
            return false
        }
        if (!formData.password) {
            toast.push("创建失败, 密码不能为空")
            return false
        }
        return true
    }

    async function handleSubmit() {
        isValid = validateForm()
        if (isValid) {
            isLoading = true
            try {
                const operation: DbOperation = {
                    Insert: {
                        ...formData,
                        id: 0,
                    },
                }
                const dbRes = await invokeDbOperation<Connection>(operation)
                const connection = dbRes.data
                toast.push("新连接创建成功")
                $connectionStore.all = [connection, ...$connectionStore.all]
                showDialog = false
                formData = initializeForm()
            } catch (error) {
                toast.push(error as string)
            } finally {
                isLoading = false
            }
        }
    }

    let showDialog = false

    $: if (showDialog) {
        formData = initializeForm()
    }
</script>

<div class="p-4">
    <p class="w-full mb-4 no-double-click-selection">主页内容</p>
    <button
        class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
        on:click="{() => (showDialog = true)}">
        创建新SSH连接
    </button>

    {#if showDialog}
        <Dialog>
            <div class="bg-white rounded-lg shadow-xl p-6 w-96 max-w-full">
                <h2 class="text-2xl font-bold mb-6 text-gray-800">
                    创建新SSH连接
                </h2>
                <div class="mb-4">
                    <span class="block text-sm font-medium text-gray-700 mb-1"
                        >连接名称</span>
                    <input
                        class="form-input"
                        type="text"
                        bind:value="{formData.name}"
                        placeholder="请输入连接名称" />
                </div>
                <div class="mb-4">
                    <strong class="block text-sm font-medium text-gray-700 mb-1"
                        >主机</strong>
                    <input
                        class="form-input"
                        type="text"
                        bind:value="{formData.host}"
                        placeholder="请输入主机地址" />
                </div>
                <div class="mb-4">
                    <em class="block text-sm font-medium text-gray-700 mb-1"
                        >端口</em>
                    <input
                        class="form-input"
                        type="number"
                        bind:value="{formData.port}"
                        placeholder="请输入端口号" />
                </div>
                <div class="mb-4">
                    <b class="block text-sm font-medium text-gray-700 mb-1"
                        >用户名</b>
                    <input
                        class="form-input"
                        type="text"
                        bind:value="{formData.username}"
                        placeholder="请输入用户名" />
                </div>
                <div class="mb-4">
                    <i class="block text-sm font-medium text-gray-700 mb-1"
                        >密码</i>
                    <input
                        class="form-input"
                        type="password"
                        bind:value="{formData.password}"
                        placeholder="请输入密码" />
                </div>
                <div class="flex justify-end space-x-3 mt-6">
                    <button
                        type="button"
                        class="px-4 py-2 text-gray-600 bg-gray-100 hover:bg-gray-200 rounded-md transition-colors"
                        on:click="{() => (showDialog = false)}">
                        取消
                    </button>
                    <button
                        on:click="{handleSubmit}"
                        class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors flex items-center justify-center"
                        class:cursor-not-allowed="{isLoading}"
                        disabled="{isLoading}">
                        {#if isLoading}
                            <Loader class="animate-spin mr-2" size="{18}" />
                        {/if}
                        创建连接
                    </button>
                </div>
            </div>
        </Dialog>
    {/if}
</div>

<style>
</style>

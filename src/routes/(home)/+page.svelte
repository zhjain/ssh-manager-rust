<script lang="ts">
    import Dialog from "$lib/components/common/Dialog.svelte"
    import { invoke } from "@tauri-apps/api/tauri"

    let form = {
        name: "",
        host: "",
        port: "",
    }

    let greetMsg = ""
    let showDialog = false

    async function addConnection() {
        try {
            const res: string = await invoke("handle_db_operation", {
                operation: { Insert: form },
            })
            greetMsg = res
        } catch (e) {
            greetMsg = e as string
        } finally {
            showDialog = false
        }
    }
</script>

<div class="p-4">
    <p class="w-full mb-4">主页内容</p>
    <button
        class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
        on:click="{() => (showDialog = true)}">
        创建新连接
    </button>

    {#if showDialog}
        <Dialog>
            <div class="bg-white rounded-lg shadow-xl p-6 w-96 max-w-full">
                <h2 class="text-2xl font-bold mb-6 text-gray-800">
                    创建新连接
                </h2>
                <input
                    class="form-input"
                    type="text"
                    placeholder="请输入连接名"
                    bind:value="{form.name}" />
                <div class="flex gap-2">
                    <input
                        class="form-input basis-2/3"
                        type="text"
                        placeholder="127.0.0.1"
                        bind:value="{form.host}" />
                    <span class="h-full my-auto pb-4 text-gray-500">:</span>
                    <input
                        class="form-input basis-1/3"
                        type="text"
                        placeholder="6379"
                        bind:value="{form.port}" />
                </div>
                <div class="flex justify-end space-x-3 mt-6">
                    <button
                        class="px-4 py-2 text-gray-600 bg-gray-100 hover:bg-gray-200 rounded-md transition-colors"
                        on:click="{() => (showDialog = false)}">
                        取消
                    </button>
                    <button
                        class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors"
                        on:click="{addConnection}">
                        创建连接
                    </button>
                </div>
            </div>
        </Dialog>
    {/if}

    {#if greetMsg}
        <p class="mt-4 text-green-600">{greetMsg}</p>
    {/if}
</div>

<style>
</style>

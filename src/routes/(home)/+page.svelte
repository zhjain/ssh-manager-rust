<script lang="ts">
    import Dialog from "$lib/components/common/Dialog.svelte"
    import { invoke } from "@tauri-apps/api/tauri"
    import { createForm } from 'felte'
    import { validator } from '@felte/validator-zod'
    import { z } from "zod"

    import connectionStore  from "$lib/store/connectionStore"

    const schema = z.object({
        name: z.string().min(1, { message: "连接名称不能为空" }),
        host: z.string().min(1, { message: "主机不能为空" }),
        port: z.number().int().min(1).max(65535).default(22),
        username: z.string().min(1, { message: "用户名不能为空" }),
        password: z.string().min(1, { message: "密码不能为空" }),
    })

    const { form, errors, isValid } = createForm({
        initialValues: {
            name: '',
            host: '',
            port: 0,
            username: '',
            password: '',
        },
        validate: validator({ schema }),
        onSubmit: async (values) => {
            try {
                const res = await invoke("handle_db_operation", {
                    operation: { Insert: values },
                })
                $connectionStore.update(c => ({ ...c, all: res.data }))
                greetMsg = res
                showDialog = false
            } catch (e) {
                greetMsg = e as string
            }
        },
    })

    let greetMsg = ""
    let showDialog = false
</script>



<div class="p-4">
    <p class="w-full mb-4">主页内容</p>
    <button
        class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
        on:click="{() => (showDialog = true)}">
        创建新SSH连接
    </button>

    {#if showDialog}
        <Dialog>
            <form
                use:form
                class="bg-white rounded-lg shadow-xl p-6 w-96 max-w-full">
                <h2 class="text-2xl font-bold mb-6 text-gray-800">
                    创建新SSH连接
                </h2>
                <div class="mb-4">
                    <input
                        class="form-input w-full {$errors.name
                            ? 'border-red-500'
                            : ''}"
                        type="text"
                        name="name"
                        placeholder="连接名称" />
                    {#if $errors.name}
                        <p class="text-red-500 text-sm mt-1">{$errors.name[0]}</p>
                    {/if}
                </div>
                <div class="mb-4">
                    <input
                        class="form-input w-full {$errors.host
                            ? 'border-red-500'
                            : ''}"
                        type="text"
                        name="host"
                        placeholder="主机" />
                    {#if $errors.host}
                        <p class="text-red-500 text-sm mt-1">{$errors.host[0]}</p>
                    {/if}
                </div>
                <div class="mb-4">
                    <input
                        class="form-input w-full {$errors.port
                            ? 'border-red-500'
                            : ''}"
                        type="number"
                        name="port"
                        placeholder="端口" />
                    {#if $errors.port}
                        <p class="text-red-500 text-sm mt-1">{$errors.port[0]}</p>
                    {/if}
                </div>
                <div class="mb-4">
                    <input
                        class="form-input w-full {$errors.username
                            ? 'border-red-500'
                            : ''}"
                        type="text"
                        name="username"
                        placeholder="用户名" />
                    {#if $errors.username}
                        <p class="text-red-500 text-sm mt-1">
                            {$errors.username[0]}
                        </p>
                    {/if}
                </div>
                <div class="mb-4">
                    <input
                        class="form-input w-full {$errors.password
                            ? 'border-red-500'
                            : ''}"
                        type="password"
                        name="password"
                        placeholder="密码" />
                    {#if $errors.password}
                        <p class="text-red-500 text-sm mt-1">
                            {$errors.password[0]}
                        </p>
                    {/if}
                </div>
                <div class="flex justify-end space-x-3 mt-6">
                    <button
                        type="button"
                        class="px-4 py-2 text-gray-600 bg-gray-100 hover:bg-gray-200 rounded-md transition-colors"
                        on:click="{() => (showDialog = false)}">
                        取消
                    </button>
                    <button
                        type="submit"
                        class="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors"
                        disabled="{!$isValid}">
                        创建连接
                    </button>
                </div>
            </form>
        </Dialog>
    {/if}

    {#if greetMsg}
        <p class="mt-4 text-green-600">{greetMsg}</p>
    {/if}
</div>

<style>
</style>

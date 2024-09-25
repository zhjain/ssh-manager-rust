<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import { goto } from "$app/navigation"
    import connectionStore from "$lib/store/connectionStore"
    import { Minus, Square, X } from "lucide-svelte"
    import { appWindow } from "@tauri-apps/api/window"
    import TabMenuItem from "./tabMenuItem.svelte"

    // 监听用户的输入方式（键盘或鼠标）
    function handleKeyboardEvent() {
        document.body.classList.add("user-is-tabbing")
    }

    function handleMouseEvent() {
        document.body.classList.remove("user-is-tabbing")
    }

    // 添加全局事件监听器
    window.addEventListener("keydown", e => {
        if (e.key === "Tab") {
            handleKeyboardEvent()
        }
    })

    window.addEventListener("mousedown", handleMouseEvent)

    window.addEventListener("keydown", function (e) {
        if (import.meta.env.DEV) {
            if (e.key === "F11") {
                e.preventDefault()
                invoke("handle_ssh_command", {
                    command: { CloseAllConnections: null },
                })
                $connectionStore.all.forEach(connection => {
                    connection.connected = false
                })
                $connectionStore.current = {} as Connection
                $connectionStore.serverStatus = {}
                $connectionStore.selecting = {} as Connection
                $connectionStore.connected = []
                goto("/")
            }
        } else {
            // 禁用 F5 刷新
            if (e.key === "F5") {
                e.preventDefault()
            }
            // 禁用 Ctrl+R 或 Command+R 刷新
            if ((e.ctrlKey || e.metaKey) && e.key === "r") {
                e.preventDefault()
            }
        }
        // 禁用 Ctrl+Shift+P
        if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === "P") {
            e.preventDefault()
        }
        // 禁用 Ctrl+P 打印
        if ((e.ctrlKey || e.metaKey) && e.key === "p") {
            e.preventDefault()
        }
        // 禁用 Ctrl+F 查找
        if ((e.ctrlKey || e.metaKey) && e.key === "f") {
            e.preventDefault()
        }
        // 禁用 F3 (通常用于查找下一项)
        if (e.key === "F3") {
            e.preventDefault()
        }
        // 禁用 F7 (通常用于启用/禁用光标浏览模式)
        if (e.key === "F7") {
            e.preventDefault()
        }
    })

    // 禁用右键菜单
    // window.addEventListener("contextmenu", function (e) {
    //     e.preventDefault()
    //     console.log("右键菜单已被禁用")
    // })

    // function handleDragTitleBar() {
    //     appWindow.startDragging()
    // }

    function handleMinimize() {
        appWindow.minimize()
    }

    function handleMaximize() {
        appWindow.toggleMaximize()
    }

    function handleClose() {
        appWindow.close()
    }

    import { page } from "$app/stores"
    import { cn } from "$lib/utils"

    $: isActive = (route: string) => $page.url.pathname.startsWith(route)

    function handleTabClick(item: Connection | { id: string }) {
        $connectionStore.current = item
        goto(`/connections/${item.id}`)
    }
</script>

<div
    data-tauri-drag-region
    class="flex fixed w-full h-10 bg-gray-100 border-b items-center pl-2">
    <div
        data-tauri-drag-region
        on:dblclick="{handleMaximize}"
        class="flex-1 flex cursor-default gap-2 items-center">
        <div data-tauri-drag-region class="bg-red-200 w-6 h-6"></div>
        <span data-tauri-drag-region class="font-semibold text-gray-700"
            >SSH连接管理器</span>
        <div
            class="{cn('flex h-full ml-20 overflow-x-auto', {
                hidden: !isActive('/connections'),
            })}">
            <TabMenuItem
                active="{isActive('/connections/all')}"
                on:click="{() => handleTabClick({ id: 'all' })}">
                全部
            </TabMenuItem>
            {#each $connectionStore.connected as item}
                <div
                    class="border-l border-gray-200 h-5 my-auto mx-0.5 opacity-50">
                </div>
                <TabMenuItem
                    active="{isActive(`/connections/${item.id}`)}"
                    on:click="{() => handleTabClick(item)}">
                    {item.name}
                    <X
                        class="hover:bg-gray-400 rounded transition-all"
                        color="gray"
                        size="{18}" />
                </TabMenuItem>
            {/each}
        </div>
    </div>

    <div class="flex items-center">
        <div on:click="{handleMinimize}" class="title-bar-button">
            <Minus size="{18}" />
        </div>
        <div on:click="{handleMaximize}" class="title-bar-button">
            <Square size="{14}" />
        </div>
        <div on:click="{handleClose}" class="title-bar-button hover:bg-red-400">
            <X size="{18}" />
        </div>
    </div>
</div>

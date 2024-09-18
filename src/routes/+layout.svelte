<script lang="ts">
    import { SvelteToast } from "@zerodevx/svelte-toast"
    import connectionStore from "$lib/store/connectionStore"
    import "../app.css"
    import SiderMenuItem from "$lib/components/leftNav/SiderNavItem.svelte"

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
        // 禁用 F5 刷新
        if (e.key === "F5") {
            e.preventDefault()
        }
        // 禁用 Ctrl+R 或 Command+R 刷新
        if ((e.ctrlKey || e.metaKey) && e.key === "r") {
            e.preventDefault()
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

    const options = {}
</script>

<div class="flex items-center justify-center w-full min-h-screen">
    <div class="flex flex-col shrink-0 w-12 h-screen border-r">
        <!-- <p>主页布局</p> -->
        {#if !!$connectionStore.current.id}
            <SiderMenuItem
                href="{'/connections/' + $connectionStore.current.id}"
                >主页</SiderMenuItem>
        {/if}
        <SiderMenuItem href="{'/'}">连接</SiderMenuItem>
        <SiderMenuItem href="{'/logger'}">日志</SiderMenuItem>
    </div>
    <div class="flex-1">
        <slot />
    </div>
</div>

<SvelteToast {options} />

<style>
</style>

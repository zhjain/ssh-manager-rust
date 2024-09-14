<script lang="ts">
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
</script>

<div class="flex items-center justify-center w-full min-h-screen">
    <div class="flex flex-col shrink-0 w-12 h-screen border-r">
        <!-- <p>主页布局</p> -->
        {#if !!$connectionStore.current.id}
            <SiderMenuItem href="{'/connections/' + $connectionStore.current.id}"
                >主页</SiderMenuItem>
        {/if}
        <SiderMenuItem href="{'/'}">连接</SiderMenuItem>
        <SiderMenuItem href="{'/logger'}">日志</SiderMenuItem>
    </div>
    <div class="flex-1">
        <slot />
    </div>
</div>

<style>
</style>

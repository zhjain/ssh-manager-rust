<script lang="ts">
    import { SvelteToast } from "@zerodevx/svelte-toast"
    import connectionStore from "$lib/store/connectionStore"
    import "../app.css"
    import SiderMenuItem from "$lib/components/leftNav/SiderNavItem.svelte"
    import TitleBar from "$lib/components/titleBar/index.svelte"

    const options = {}
</script>

<TitleBar />
<div class="flex flex-1 w-full h-screen pt-10">
    <div class="flex flex-col w-12 border-r">
        <!-- <p>主页布局</p> -->
        {#if $connectionStore.connected.length > 0}
            <SiderMenuItem
                href="{'/connections/' + $connectionStore.current.id}"
                >主页</SiderMenuItem>
        {/if}
        <SiderMenuItem href="{'/'}">连接</SiderMenuItem>
        <SiderMenuItem href="{'/logger'}">日志</SiderMenuItem>
    </div>
    <div class="flex-1 h-full">
        <slot />
    </div>
</div>
<SvelteToast {options} />

<style>
</style>

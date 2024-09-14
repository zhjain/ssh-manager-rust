<script lang="ts">
    import { page } from "$app/stores"
    import connectionStore from "$lib/store/connectionStore"
    // 获取路由参数
    let { id } = $page.params
</script>

<p class="flex w-full items-center justify-center">主页内容</p>
{id}
{#each $connectionStore.all as connection}
    {#if connection.connected}
        <p
            class="flex w-full items-center justify-center"
            class:text-red-500="{connection.id === parseInt(id)}">
            {connection.name}
        </p>
    {/if}
{/each}

<style>
</style>

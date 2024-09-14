<script lang="ts">
    import connectionStore from "$lib/store/connectionStore"

    import ConnectionItem from "./ConnectionItem.svelte"
    import ConnectionListBottom from "./ConnectionListBottom.svelte"
    import EmptyConnection from "./EmptyConnection.svelte"
</script>

<div class="flex flex-col w-64 bg-gray-100">
    <div class="flex-1">
        {#if $connectionStore.all.length === 0}
            <EmptyConnection />
        {:else}
            {#each $connectionStore.all as connection}
                <ConnectionItem {connection} />
            {/each}
        {/if}
    </div>
    <ConnectionListBottom />
</div>

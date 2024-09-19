<script lang="ts">
    import { Svrollbar } from "svrollbar"

    import connectionStore from "$lib/store/connectionStore"

    import ConnectionItem from "./ConnectionItem.svelte"
    import ConnectionListBottom from "./ConnectionListBottom.svelte"
    import EmptyConnection from "./EmptyConnection.svelte"

    let viewport: Element
    let contents: Element
</script>

<div class="wrapper flex flex-col relative w-64 bg-gray-100">
    <div bind:this="{viewport}" class="viewport flex-1">
        {#if $connectionStore.all.length === 0}
            <EmptyConnection />
        {:else}
            <div bind:this="{contents}" class="contents">
                {#each $connectionStore.all as connection}
                    <ConnectionItem {connection} />
                {/each}
            </div>
        {/if}
    </div>
    <Svrollbar {viewport} {contents} />

    <ConnectionListBottom />
</div>

<style>
    .wrapper {
        --svrollbar-track-width: 6px;
        --svrollbar-track-radius: 0;
        --svrollbar-track-background: rgba(211, 211, 211, 0.274);
        --svrollbar-track-opacity: 1;
        --svrollbar-track-shadow: initial;

        --svrollbar-thumb-width: 6px;
        --svrollbar-thumb-radius: 8px;
        --svrollbar-thumb-background: rgba(128, 128, 128, 0.425);
        --svrollbar-thumb-opacity: 0.5;
        --svrollbar-thumb-shadow: initial;
    }
    .viewport {
        position: relative;
        /* width: 10rem; */
        /* height: 10rem; */
        overflow: scroll;
        /* border: 1px solid gray; */
        box-sizing: border-box;

        /* hide scrollbar */
        -ms-overflow-style: none;
        scrollbar-width: none;
    }

    .viewport::-webkit-scrollbar {
        /* hide scrollbar */
        display: none;
    }
</style>

<script lang="ts">
    import { cn } from "$lib/utils"

    export let active = false
</script>

<div
    on:click
    class="{cn(
        'flex gap-1 shrink-0 px-2 py-3 border border-transparent rounded-md hover:bg-gray-300 text-sm font-medium cursor-pointer transition-colors duration-200 items-center justify-center',
        active
            ? 'text-blue-600 font-bold border-gray-200 -z-1 bg-white hover:bg-white'
            : 'text-gray-700',
    )}">
    <slot />
</div>

<script lang="ts">
    import { ConnectionList } from "$lib/components/index.js"
    import { cn } from "$lib/utils"
</script>

<div class="{cn('flex h-screen')}">
    <ConnectionList />
    <div>
        <slot />
    </div>
</div>

<script>
    import { cn } from "$lib/utils"
    import { page } from "$app/stores"

    export let href

    $: currentPage = $page.url.pathname
</script>

<a {href} class="{cn('group flex w-full h-12 rounded hover:bg-gray-200')}">
    <div
        class="{cn(
            'flex w-full my-auto border-x-4',
            'border-x-transparent group-hover:border-l-red-500',
            href === currentPage && 'border-l-red-500',
        )}">
        <div class="m-auto">
            <slot />
        </div>
    </div>
</a>

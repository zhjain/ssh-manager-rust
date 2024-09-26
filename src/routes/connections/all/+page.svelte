<script lang="ts">
    import connectionStore from "$lib/store/connectionStore"
    import { cn } from "$lib/utils"
</script>

<div
    class="{cn(
        'grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4',
        'z-10 bg-white',
        'h-full w-full p-4 overflow-auto',
    )}">
    {#each $connectionStore.connected as { id, host, name }}
        <div
            class="{cn(
                'bg-white rounded-lg shadow-md',
                'p-4 mb-4 w-full h-fit max-w-md',
            )}">
            <h2 class="{cn('text-xl text-blue-600 font-semibold mb-2')}">
                {name}
                <span class="{cn('text-sm text-gray-500')}">{host}</span>
            </h2>
            <div class="{cn('text-sm text-gray-600')}">
                <p>
                    <span class="{cn('font-medium')}">CPU 使用率：</span>
                    {$connectionStore.serverStatus[id]?.cpu_usage || 0}%
                </p>
                <p>
                    <span class="{cn('font-medium')}">内存使用率：</span>
                    {(
                        $connectionStore.serverStatus[id]?.memory_usage || 0
                    ).toFixed(2)}%
                </p>
                <p>
                    <span class="{cn('font-medium')}">磁盘使用率：</span>
                    {$connectionStore.serverStatus[id]?.disk_usage
                        ?.use_percentage || "0%"}
                </p>
                <p>
                    <span class="{cn('font-medium')}">下载速率：</span>
                    {(
                        $connectionStore.serverStatus[id]?.download_speed || 0
                    ).toFixed(2)} KB/s
                </p>
                <p>
                    <span class="{cn('font-medium')}">上传速率：</span>
                    {(
                        $connectionStore.serverStatus[id]?.upload_speed || 0
                    ).toFixed(2)} KB/s
                </p>
            </div>
        </div>
    {/each}
</div>

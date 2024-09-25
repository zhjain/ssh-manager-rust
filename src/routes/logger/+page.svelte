<script lang="ts">
    // import BarChartLogger from "$lib/components/barChartLogger/index.svelte"
    // import { onMount } from "svelte"

    // let maxDataPoints = 100
    // let uploadData: number[] = []
    // let downloadData: number[] = []

    // function generateSmoothData(
    //     prevValue: number,
    //     min: number,
    //     max: number,
    // ): number {
    //     const maxChange = 5 // 最大变化幅度
    //     const change = Math.random() * maxChange * 2 - maxChange
    //     let newValue = prevValue + change
    //     newValue = Math.max(min, Math.min(max, newValue)) // 确保值在范围内
    //     return Number(newValue.toFixed(2)) // 保留两位小数
    // }

    // function simulateDataUpdate() {
    //     const lastUploadValue =
    //         uploadData.length > 0 ? uploadData[uploadData.length - 1] : 20
    //     const lastDownloadValue =
    //         downloadData.length > 0 ? downloadData[downloadData.length - 1] : 50

    //     // 修改上行数据生成逻辑，使其通常小于下行数据
    //     let newUploadValue = generateSmoothData(lastUploadValue, 0, 40)
    //     let newDownloadValue = generateSmoothData(lastDownloadValue, 30, 100)

    //     // 确保上行数据始终小于下行数据
    //     if (newUploadValue >= newDownloadValue) {
    //         newUploadValue = newDownloadValue * 0.8
    //     }

    //     uploadData = [...uploadData, newUploadValue]
    //     downloadData = [...downloadData, newDownloadValue]

    //     if (uploadData.length > maxDataPoints) {
    //         uploadData = uploadData.slice(-maxDataPoints)
    //     }
    //     if (downloadData.length > maxDataPoints) {
    //         downloadData = downloadData.slice(-maxDataPoints)
    //     }

    //     uploadData = [...uploadData]
    //     downloadData = [...downloadData]
    // }

    // onMount(() => {
    //     const interval = setInterval(simulateDataUpdate, 1000) // 每秒更新一次数据

    //     return () => {
    //         clearInterval(interval)
    //     }
    // })
</script>

<!-- <div class="w-80 max-w-2xl mx-auto mt-8 h-48">
    <BarChartLogger {maxDataPoints} {uploadData} {downloadData} />
</div> -->
<div
    class="flex relative items-center z-10 bg-white justify-center h-full w-full">
    <div class="text-2xl font-bold text-gray-600">
        <span class="animate-pulse">正在开发中</span>
        <span class="animate-bounce inline-block ml-2">...</span>
    </div>
</div>

<style>
    /* 可以添加额外的样式，如果需要的话 */
</style>

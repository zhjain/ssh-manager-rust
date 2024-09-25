<script lang="ts">
    import { onMount, afterUpdate } from "svelte"
    import Chart from "chart.js/auto"
    import connectionStore from "$lib/store/connectionStore"

    export let connectionId: number

    let maxDataPoints = 30
    let chartContainer: HTMLCanvasElement
    let chart: Chart
    let recentMaxValues: number[] = []
    let currentUnit = "kb/s"

    const uploadColor = "#FFD54F" // 更亮的黄色
    const downloadColor = "#29B6F6" // 浅蓝色

    $: serverStatus = $connectionStore.serverStatus[connectionId] || {}
    $: uploadData = serverStatus.uploadSpeedHistory || []
    $: downloadData = serverStatus.downloadSpeedHistory || []

    $: chartData = {
        labels: Array.from({ length: maxDataPoints }, (_, i) =>
            (maxDataPoints - 1 - i).toString(),
        ),
        datasets: [
            {
                label: "上传速率",
                data:
                    uploadData.length > 0
                        ? uploadData.slice(-maxDataPoints).reverse()
                        : new Array(maxDataPoints).fill(0),
                backgroundColor: `${uploadColor}dd`, // 添加透明度
                borderColor: uploadColor,
                borderWidth: 1,
                fill: true,
                tension: 0.4,
                pointRadius: 0, // 去掉圆圈
            },
            {
                label: "下载速率",
                data:
                    downloadData.length > 0
                        ? downloadData.slice(-maxDataPoints).reverse()
                        : new Array(maxDataPoints).fill(0),
                backgroundColor: `${downloadColor}dd`, // 添加透明度
                borderColor: downloadColor,
                borderWidth: 1,
                fill: true,
                tension: 0.4,
                pointRadius: 0, // 去掉圆圈
            },
        ],
    }

    $: {
        const currentMaxValue = Math.max(
            ...uploadData.slice(-1),
            ...downloadData.slice(-1),
        )
        recentMaxValues = [
            ...recentMaxValues.slice(-14),
            currentMaxValue,
        ].slice(-15)
    }
    $: maxValue = Math.max(...recentMaxValues)
    $: {
        if (maxValue > 1024) {
            currentUnit = "mb/s"
            chartData.datasets.forEach(dataset => {
                dataset.data = dataset.data.map(value => value / 1024)
            })
            maxValue /= 1024
        } else {
            currentUnit = "kb/s"
        }
    }
    $: suggestedMax = maxValue * 1.3 // 设置建议的最大值为当前最大值的1.1倍

    $: chartOptions = {
        responsive: true,
        maintainAspectRatio: false,
        animation: {
            duration: 0,
        },
        scales: {
            x: {
                type: "linear" as const,
                position: "bottom" as const,
                min: 0,
                max: maxDataPoints - 1,
                grid: {
                    display: false,
                },
                ticks: {
                    display: false,
                },
            },
            y: {
                type: "linear" as const,
                position: "left" as const,
                min: 0,
                max: suggestedMax,
                grid: {
                    display: true,
                    color: (context: { tick: { value: number } }) => {
                        if (context.tick.value % (suggestedMax / 4) === 0) {
                            return "rgba(0, 0, 0, 0.1)"
                        }
                        return "rgba(0, 0, 0, 0)"
                    },
                    lineWidth: 1,
                    drawTicks: false,
                    tickBorderDash: [2, 5],
                    z: 1,
                },
                ticks: {
                    display: true,
                    callback: function (value: number) {
                        return value.toFixed(2) + " " + currentUnit
                    },
                    color: "rgba(0, 0, 0, 0.6)", // 更柔和的刻度颜色
                    stepSize: suggestedMax / 4,
                },
            },
        },
        plugins: {
            legend: {
                display: true, // 显示图例
                position: "top" as const,
            },
            tooltip: {
                enabled: false, // 禁用tooltip
            },
        },
        hover: {
            mode: "nearest" as const, // 修改为有效的hover模式
        },
        layout: {
            padding: {
                left: 0,
                right: 10,
            },
        },
    }

    onMount(() => {
        chart = new Chart(chartContainer, {
            type: "line",
            data: chartData,
            options: chartOptions,
        })
        return () => {
            chart.destroy()
        }
    })

    afterUpdate(() => {
        if (chart) {
            chart.data = chartData
            chart.options = chartOptions
            chart.update()
        }
    })
</script>

<canvas bind:this="{chartContainer}"></canvas>

<style>
    canvas {
        background-color: rgba(255, 255, 255, 0.9); /* 略微透明的白色背景 */
    }
</style>

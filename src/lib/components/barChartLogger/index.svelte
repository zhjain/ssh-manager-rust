<script lang="ts">
    import { onMount, afterUpdate } from "svelte"
    import Chart from "chart.js/auto"
    import type { ChartConfiguration } from "chart.js"

    export let maxDataPoints = 100
    export let uploadColor = "#FFA726" // 橙色
    export let downloadColor = "#29B6F6" // 浅蓝色
    export let uploadData: number[] = []
    export let downloadData: number[] = []

    let chartContainer: HTMLCanvasElement
    let chart: Chart

    $: chartData = {
        labels: Array.from({ length: maxDataPoints }, (_, i) =>
            (maxDataPoints - 1 - i).toString(),
        ),
        datasets: [
            {
                label: "下载速率",
                data:
                    downloadData.length > 0
                        ? downloadData.slice(-maxDataPoints).reverse()
                        : new Array(maxDataPoints).fill(0),
                backgroundColor: downloadColor,
                borderWidth: 0,
                stack: "combined",
                type: "bar" as const,
                barPercentage: 1.0,
                categoryPercentage: 1.0,
            },
            {
                label: "上传速率",
                data:
                    uploadData.length > 0
                        ? uploadData.slice(-maxDataPoints).reverse()
                        : new Array(maxDataPoints).fill(0),
                backgroundColor: uploadColor,
                borderWidth: 0,
                stack: "combined",
                type: "bar" as const,
                barPercentage: 1.0,
                categoryPercentage: 1.0,
            },
        ],
    }
    const chartOptions: ChartConfiguration<"bar">["options"] = {
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
                offset: false,
                stacked: true,
            },
            y: {
                type: "linear" as const,
                position: "left" as const,
                min: 0,
                suggestedMax: 100,
                grid: {
                    display: true,
                    color: (context: { tick: { value: number } }) => {
                        if (context.tick.value % 25 === 0) {
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
                    callback: function (value) {
                        return value + " MB/s"
                    },
                    color: "rgba(0, 0, 0, 0.6)", // 更柔和的刻度颜色
                },
                stacked: true,
            },
        },
        plugins: {
            legend: {
                display: true,
                position: "top" as const,
                labels: {
                    color: "rgba(0, 0, 0, 0.7)", // 更柔和的图例文字颜色
                    font: {
                        size: 12,
                    },
                },
            },
            tooltip: {
                enabled: true,
                backgroundColor: "rgba(255, 255, 255, 0.8)", // 半透明白色背景
                titleColor: "rgba(0, 0, 0, 0.8)", // 深色标题
                bodyColor: "rgba(0, 0, 0, 0.7)", // 深色内容
                borderColor: "rgba(0, 0, 0, 0.1)", // 浅色边框
                borderWidth: 1,
                callbacks: {
                    label: function (context) {
                        let label = context.dataset.label || ""
                        if (label) {
                            label += ": "
                        }
                        if (context.parsed.y !== null) {
                            label += context.parsed.y.toFixed(2) + " MB/s"
                        }
                        return label
                    },
                },
            },
        },
        elements: {
            bar: {
                borderSkipped: false,
            },
        },
        hover: {
            mode: "index" as const,
            intersect: false,
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
            type: "bar",
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

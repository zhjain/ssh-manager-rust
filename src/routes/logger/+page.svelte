<script lang="ts">
    import { onMount } from "svelte"
    import Chart from "chart.js/auto"
    import type { ChartConfiguration, InteractionModeMap } from "chart.js"

    let chartContainer: HTMLCanvasElement
    let chart: Chart

    const maxDataPoints = 100
    let lastCpuUsage = 50 // 初始值设为50

    const chartData = {
        labels: Array.from({ length: maxDataPoints }, (_, i) => i.toString()),
        datasets: [
            {
                data: Array(maxDataPoints).fill(null),
                backgroundColor: "rgb(0, 123, 255)", // 使用不透明的颜色
                borderWidth: 0,
                barPercentage: 1,
                categoryPercentage: 1,
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
                    display: false, // 隐藏X轴刻度
                },
            },
            y: {
                type: "linear" as const,
                position: "left" as const,
                min: 0,
                max: 100,
                grid: {
                    display: true,
                    color: (context: { tick: { value: number } }) => {
                        if (context.tick.value % 25 === 0) {
                            return "rgba(0, 0, 0, 0.1)" // 点状线颜色
                        }
                        return "rgba(0, 0, 0, 0)"
                    },
                    lineWidth: 1,
                    drawTicks: false,
                    tickBorderDash: [2, 5], // 设置点状线样式
                    z: 1,
                },
                ticks: {
                    display: true, // 显示Y轴刻度
                    callback: (value: number | string) => {
                        const numValue = Number(value)
                        if (
                            numValue === 0 ||
                            numValue === 25 ||
                            numValue === 50 ||
                            numValue === 75 ||
                            numValue === 100
                        ) {
                            return numValue.toString()
                        }
                        return ""
                    },
                },
            },
        },
        plugins: {
            legend: {
                display: false,
            },
            tooltip: {
                enabled: false, // 禁用工具提示以去除悬停效果
            },
        },
        elements: {
            bar: {
                borderSkipped: false,
            },
        },
        hover: {
            mode: undefined, // 禁用悬停模式
        },
    }

    // 生成新的 CPU 使用率
    const getNextCpuUsage = (): number => {
        const change = Math.floor(Math.random() * 21) - 10 // 生成-10到10之间的随机数
        let newUsage = lastCpuUsage + change
        newUsage = Math.max(0, Math.min(100, newUsage)) // 确保值在0到100之间
        lastCpuUsage = newUsage
        return newUsage
    }

    // 添加新的数据点
    const addNewDataPoint = (): void => {
        const newData = getNextCpuUsage()
        chartData.datasets[0].data.push(newData)

        if (chartData.datasets[0].data.length > maxDataPoints) {
            chartData.datasets[0].data.shift()
        }

        chart.update()
    }

    onMount(() => {
        chart = new Chart(chartContainer, {
            type: "bar",
            data: chartData,
            options: chartOptions,
        })

        // 每0.3秒添加数据点
        const interval = setInterval(addNewDataPoint, 300)

        // 在组件卸载时清除定时器
        return () => {
            clearInterval(interval)
            chart.destroy()
        }
    })
</script>

<!-- HTML 模板 -->
<div class="w-full max-w-2xl mx-auto mt-8 h-64">
    <canvas bind:this="{chartContainer}"></canvas>
</div>

<style>
    /* 可以添加额外的样式，如果需要的话 */
</style>

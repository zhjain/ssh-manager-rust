<script lang="ts">
    import { onMount } from "svelte"
    import ApexCharts from "apexcharts" // 使用原生 ApexCharts

    let chart: ApexCharts | null = null // 用于存储图表实例

    // 图表初始配置
    let chartOptions = {
        chart: {
            type: "line",
            height: 350,
            animations: {
                enabled: true,
                easing: "linear",
                dynamicAnimation: {
                    speed: 1000,
                    // enabled: false,
                },
                // animateGradually: {
                //     enabled: true,
                //     delay: 150,
                // },
            },
            toolbar: {
                show: false,
            },
            zoom: {
                enabled: false,
            },
        },
        series: [
            {
                name: "CPU Usage",
                data: [] as { x: number; y: number }[],
            },
        ],
        xaxis: {
            type: "numeric",
            range: 10, // 显示 10 个数据点
            tickAmount: 10,
        },
        yaxis: {
            max: 100,
            min: 0,
        },
    }

    let lastTime = 0

    // 随机生成 CPU 使用率
    const getRandomCpuUsage = (): number => {
        return Math.floor(Math.random() * 100)
    }

    // 添加新的数据点
    const addNewDataPoint = (): void => {
        lastTime += 1

        const newData = {
            x: lastTime,
            y: getRandomCpuUsage(),
        }

        if (chart) {
            // 更新图表数据
            chart.appendData([
                {
                    // name: "CPU Usage",
                    data: [newData],
                },
            ])
            // console.log(chartOptions.series[0].data.length)
        }
    }

    onMount(() => {
        // 初始化图表
        const chartElement = document.querySelector("#cpu-chart") as HTMLElement
        chart = new ApexCharts(chartElement, chartOptions)
        chart.render()

        // 每秒添加数据点
        const interval = setInterval(addNewDataPoint, 1000)

        // 在组件卸载时清除定时器
        return () => {
            clearInterval(interval)
            if (chart) {
                chart.destroy() // 销毁图表实例
            }
        }
    })
</script>

<!-- HTML 模板 -->
<div id="cpu-chart"></div>

<style>
    /* 样式可以根据需求调整 */
    #cpu-chart {
        max-width: 100%;
        margin: 0 auto;
    }
</style>

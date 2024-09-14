<script>
    import connectionStore from "$lib/store/connectionStore"
    export let data
</script>

<p class="bg-red-300">日志内容</p>

<p>{data.post.title}</p>
<p>{data.post.content}</p>
<div>{$connectionStore.selecting}</div>

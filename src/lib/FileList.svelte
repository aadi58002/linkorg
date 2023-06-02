<script lang="ts">
 export let list: string[];
 export let FileListClick: any;
</script>

<h1 class="text-white text-center m-0 pt-10">Org Files List</h1>
<div class="px-20 text-center text-white">
    {#each list as filename}
    <hr class="my-4"/>
    <p on:click={FileListClick(filename)} on:keypress={FileListClick(filename)} class="m-0">{filename}</p>
    {/each}
    <hr class="my-4"/>
</div>

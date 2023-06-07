<script lang="ts">
 export let list: string[];
 export let FileListClick: any;
</script>

<h1 class="text-center m-0 pt-10">Org Files List</h1>
<div class="text-center px-20">
    {#each list as filename}
    <hr class="my-4"/>
    <p on:click={FileListClick(filename)} on:keypress={FileListClick(filename)} class="m-0">{filename.replace(/^.*[\\\/]/, '')}</p>
    {/each}
    <hr class="my-4"/>
</div>

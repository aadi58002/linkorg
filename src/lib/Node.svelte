<script lang="ts">
    import type { FileData, LinkData } from "../types/data.d";
    import Links from '../lib/Links.svelte';
    export let data: FileData;
    export let indent = 0;

    let open = true;

    function toggleOpen() {
        open = !open;
    }
    function checkForHeading(val: FileData): boolean {
        console.log(val,!Array.isArray(val))
        return !Array.isArray(val);
    }
</script>

{#if checkForHeading(data)}
    <h3 class="text-white" style="padding-left: {indent}px" on:click={toggleOpen} on:keypress={toggleOpen}>
        {data.heading}
        {open ? "(open)" : "(closed)"}
    </h3>
    {#if open}
        <svelte:self data={data.HeadingOrLinks} indent={indent + 24} />
    {/if}
{:else}
    <Links links={data}/>
{/if}


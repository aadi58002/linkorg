<script lang="ts">
    import type { HeadingData } from "../types/data.d";
    import Links from "../lib/Links.svelte";

    export let data: HeadingData[];
    export let indent = 0;

    indent += 24;

    let open = Array(data.length).fill(true);
    function toggleOpen(i: number) {
        open[i] = !open[i];
    }
</script>

<div>
    {#each data as element, i}
        <h3
            class="text-white"
            style="padding-left: {indent}px"
            on:click={() => {
                toggleOpen(i);
            }}
            on:keypress={() => {
                toggleOpen(i);
            }}
        >
            {element.title} + {element.level}
            {open[i] ? "(open)" : "(closed)"}
        </h3>
        {#if open[i]}
            <svelte:self data={element.heading} indent={indent} />
            <Links data={element.links} indent={indent}/>
        {/if}
    {/each}
</div>

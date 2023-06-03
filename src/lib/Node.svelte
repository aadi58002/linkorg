<script lang="ts">
    import type { HeadingData } from "../types/data.d";
    export let data: HeadingData[];
    console.log(data)
    export let indent = 0;
    let open = Array(data.length).fill(true);

    function toggleOpen(i: number) {
        open[i] = !open[i];
    }
</script>

    {#each data as element,i}
        {#if element.heading !== undefined}
            <h3
                class="text-white"
                style="padding-left: {indent}px"
                on:click={() => {toggleOpen(i)}}
                on:keypress={() => {toggleOpen(i)}}
            >
                {element.heading}
                {open[i] ? "(open)" : "(closed)"}
            </h3>
            {#if open[i]}
              <svelte:self
                data={element.heading_or_links}
                indent={indent + 24}
              />
            {/if}
        {:else}
            <div class="flex grow-1 justify-between text-white" style="padding-left: {indent}px">
                <p>{element.name}</p>
                <p>{element.link}</p>
                <p>{element.read_till}</p>
            </div>
        {/if}
    {/each}

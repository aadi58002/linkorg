<script lang="ts">
    import type { LinkData } from "../types/data.d";
    export let data: LinkData[];
    console.log(data);
    export let indent = 0;
    indent += 12;
</script>

{#each data as element}
    <div
        class="flex grow-1 justify-between text-white"
        style="padding-left: {indent}px"
    >
        <p>{element.name}</p>
        <p>{element.link}</p>
        <p>{element.read_till}</p>
    </div>
{/each}

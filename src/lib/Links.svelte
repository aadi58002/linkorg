<script lang="ts">
    import type { LinkData } from "../types/data.d";
    export let data: LinkData[];
    export let indent = 0;
    indent += 12;
</script>

<div class="flex grow-1 justify-between" style="padding-left: {indent}px">
    {#if data.length != 0}
        <table class="w-full border-collapse">
            <thead>
                <tr>
                    <th class="w-[25%] p-2 border-1 border-solid border-black"
                        >Name</th
                    >
                    <th class="w-[25%] p-2 border-1 border-solid border-black"
                        >Description</th
                    >
                    <th class="w-[25%] p-2 border-1 border-solid border-black"
                        >Likeability</th
                    >
                    <th class="w-[25%] p-2 border-1 border-solid border-black"
                        >Read After</th
                    >
                </tr>
            </thead>
            {#each data as element}
                <tbody>
                    <tr>
                        <td
                            class="w-[25%] p-2 border-1 border-solid border-black"
                            ><a href={element.link}>{element.name}</a></td
                        >
                        <td
                            class="w-[25%] p-2 border-1 border-solid border-black"
                            >{element.description ?? ""}</td
                        >
                        <td
                            class="w-[25%] p-2 border-1 border-solid border-black"
                            >{element.likeability ?? ""}</td
                        >
                        <td
                            class="w-[25%] p-2 border-1 border-solid border-black"
                            >{element.read_till}</td
                        >
                    </tr>
                </tbody>
            {/each}
        </table>
    {/if}
</div>

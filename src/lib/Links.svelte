<script lang="ts">
    import type { LinkData } from '../types/data.d'
    export let links: LinkData[];
</script>

<ul class="list-none flex text-white justify-center">
    {#each links as link}
        <li class="flex grow-1 justify-between">
            <p>{link.name}</p>
            <p>{link.link}</p>
            <p>{link.read_till}</p>
        </li>
    {/each}
</ul>

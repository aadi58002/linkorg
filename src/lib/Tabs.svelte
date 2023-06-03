<!-- <script lang="ts">
     import Node from "../lib/Node.svelte";
     import type { FileData } from "../types/data.d";

     export let items: FileData[];
     export let FileListClick: any;
     export let activeTabValue: string;
     const handleClick = (tabValue: string) => () => (activeTabValue = tabValue);
     </script>

     <ul>
     {#each items as item}
     <li class={activeTabValue === item.tab_title ? "active" : ""}>
     <span
     on:click={handleClick(item.tab_title)}
     on:keypress={handleClick(item.tab_title)}>{item.tab_title}</span
     >
     </li>
     {/each}
     </ul>
     <div class="justify-center flex">
     <button
     class="my-5 py-3 px-6 text-white border-0 bg-[#42414d] rounded"
     on:click={FileListClick("")}
     on:keypress={FileListClick("")}>Home</button
     >
     </div>
     <div class="px-10">
     {#each items as item}
     {#if activeTabValue == item.tab_title}
     <div class="box h-full">
     {#each item.heading_or_links as node}
     <Node data={node} />
     {/each}
     </div>
     {/if}
     {/each}
     </div>

     <style>
     .box {
     padding: 40px;
     height: 100%;
     border-radius: 0 0 0.5rem 0.5rem;
     }
     ul {
     display: flex;
     background: #1c1b22;
     justify-content: center;
     padding: 20px;
     margin: 0;
     color: white;
     flex-wrap: wrap;
     padding-left: 0;
     margin-bottom: 0;
     list-style: none;
     }
     li {
     margin: 5px;
     }

     span {
     border: 1px solid transparent;
     background: #1c1b22;
     border-radius: 0.25rem;
     display: block;
     padding: 0.5rem 1rem;
     cursor: pointer;
     color: white;
     }

     li.active > span {
     background-color: #42414d;
     border-color: #dee2e6 #dee2e6 #fff;
     }
     </style> -->

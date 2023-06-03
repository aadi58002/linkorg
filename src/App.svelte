<script lang="ts">
    import Node from "./lib/Node.svelte";
    import  from "./lib/Node.svelte";
    import FileList from "./lib/FileList.svelte";
 // import { invoke } from "@tauri-apps/api/tauri";
    import type { FileData } from "./types/data";

 // invoke("greet", { name: "testing" }).then((message) =>
 //     console.log(message)
 // );
    let fileOpen = "";
    let fileList = ["Content"];
    let item: FileData = {
        file_title: "Content",
        heading: [
            {
                title: "Level1",
                heading: [
                    {
                        heading: "Level2",
                        heading_or_links: [
                            {
                                name: "testing",
                                link: "hello",
                                likeability: "Must Read",
                                read_till: 5,
                                line_number: 1,
                            },
                        ],
                    },
                ],
                links: [
                    {
                        name: "testing 2",
                        link: "hello2",
                        likeability: "Must Read",
                        read_till: 2,
                        line_number: 2,
                    },
                    {
                        name: "testing 2",
                        link: "hello2",
                        likeability: "Must Read",
                        read_till: 2,
                        line_number: 2,
                    },
                ],
            },
        ],
        links: [],
    };

    const FileListClick = (file: string) => {
        fileOpen = file;
    };
</script>

<main class="h-full w-full bg-[#131516]">
    {#if fileOpen == ""}
        <FileList list={fileList} {FileListClick} />
    {:else}
        <div class="justify-center flex">
            <button
                class="my-5 py-3 px-6 text-white border-0 bg-[#42414d] rounded"
                on:click={() => {
                    FileListClick("");
                }}
                on:keypress={() => {
                    FileListClick("");
                }}>Home</button
            >
        </div>
        <div class="px-10">
            <div class="box h-full">
                <Node data={item.heading} />
                <Links data={item.links} />
            </div>
        </div>
    {/if}
</main>

<style>
    :global(html, body, main) {
        margin: 0px;
        padding: 0px;
        min-height: 100vh;
    }
</style>

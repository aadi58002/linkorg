<script lang="ts">
    import Tabs from "./lib/Tabs.svelte";
    import FileList from "./lib/FileList.svelte";
    import { invoke } from "@tauri-apps/api/tauri";

    invoke("greet", { name: "testing" }).then((message) =>
        console.log(message)
    );
    let tabOpen = "";
    let fileList = ["Content"];
    let items = [
        {
            tab_title: "Content",
            heading_or_links: {
                heading: "Level1",
                heading_or_links: {
                    heading: "Level2",
                    heading_or_links: [
                        {
                            name: "testing",
                            link: "hello",
                            read_till: 5,
                            line_number: 1,
                        },
                    ],
                },
            },
        },
    ];

    const FileListClick = (file: string) => {
        tabOpen = file;
    };
</script>

<main class="h-full w-full bg-[#131516]">
    {#if tabOpen == ""}
        <FileList list={fileList} {FileListClick} />
    {:else}
        <Tabs {items} activeTabValue={tabOpen} {FileListClick} />
    {/if}
</main>

<style>
    :global(html, body, main) {
        margin: 0px;
        padding: 0px;
        min-height: 100vh;
    }
</style>

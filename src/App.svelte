<script lang="ts">
    import Node from "./lib/Node.svelte";
    import Links from "./lib/Links.svelte";
    import FileList from "./lib/FileList.svelte";
    import MetaData from "./lib/MetaData.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import type { FileData } from "./types/data";

    invoke("greet", { name: "testing" }).then((message) =>
        console.log(message)
    );
    let fileList = (async () => {
        let config: any = await invoke("get_config");
        let list: string[] = await invoke("get_files_list", {
            notesDir: config.notes_dir,
        });
        return list;
    })();
    let fileOpen = "";

    let get_file_data = async (file: string) => {
        let file_data: FileData = await invoke("get_file_data", { file: file });
        console.log(file_data);
        return file_data;
    };

    const FileListClick = (file: string) => {
        fileOpen = file;
    };
</script>

<main class="h-full w-full">
    {#if fileOpen == ""}
        {#await fileList}
            <p>...Loading files</p>
        {:then fileList}
            <FileList list={fileList} {FileListClick} />
        {:catch e}
            <p>Something went wrong {e}</p>
        {/await}
    {:else}
        <div class="justify-center flex">
            <button
                class="my-5 py-3 px-6 font-bold border-0 rounded"
                on:click={() => {
                    FileListClick("");
                }}
                on:keypress={() => {
                    FileListClick("");
                }}>Home</button
            >
        </div>
        {#await get_file_data(fileOpen)}
            <p>...Loading files</p>
        {:then item}
            <div class="px-10">
                <div class="h-full">
                    <MetaData data={item.file_meta_data}/>
                    {#if item.heading.length != 0}
                        <Node data={item.heading} />
                    {/if}
                    <Links data={item.links} />
                </div>
            </div>
        {:catch e}
            <p>Something went wrong {e}</p>
        {/await}
    {/if}
</main>

<style>
    :global(html, body, main) {
        margin: 0px;
        padding: 0px;
        min-height: 100vh;
    }
</style>

<script lang="ts">
 import type { FileMetaData } from '../types/data';

 export let data: FileMetaData;
</script>

<hr/>
<div class="flex justify-center flex-col">
<p class="text-center">{data.file_title ?? "No Title"}</p>
<p class="text-center">{data.file_description ?? "No Description"}</p>
<p class="text-center">{data.file_creation_date ?? "No Creation Data"}</p>
<p class="text-center">{data.file_tags ?? "No Tags"}</p>
</div>
<hr/>                 
                      

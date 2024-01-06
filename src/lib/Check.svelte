<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";

    let is_in_tauri = true;

    onMount(async () => {
        try {
            is_in_tauri = await invoke("check"); // ask the backend if we're in tauri
        } catch (e) {
            // if we're not in tauri, we'll get an error
            is_in_tauri = false;
        }
    });
</script>

{#if is_in_tauri}
    <div>
        <p>You're in Tauri!</p>
    </div>
{:else}
    <div>
        <p>You're not in Tauri!</p>
    </div>
{/if}

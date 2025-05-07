<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
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

{#if !is_in_tauri}
    <div class="alert alert-danger" role="alert">
        <h1>
            This app is not running in Tauri, certain features will not
            function. Restart the app with <code>npm run tauri dev</code> or
            <code>npm run tauri build</code>
        </h1>
    </div>
{/if}

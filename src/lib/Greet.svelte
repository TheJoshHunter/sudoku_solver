<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let name = "";
    let msg = "";

    async function greet() {
        const response: string = await invoke("greet", { name });
        msg = response;
    }
</script>

<div>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}>Greet</button>
    <p>{msg}</p>
</div>

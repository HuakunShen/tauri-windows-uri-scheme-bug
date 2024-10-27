<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";

  const iframeSrc = convertFileSrc("", "ext");
  let showIframe = true;
  function reloadIframe() {
    showIframe = false;
    setTimeout(() => {
      showIframe = true;
    }, 100);
  }
</script>

<span>{iframeSrc}</span>
{#if showIframe}
  <iframe src={iframeSrc} title="iframe" width="100%"></iframe>
{/if}
<button on:click={reloadIframe}>Reload</button>

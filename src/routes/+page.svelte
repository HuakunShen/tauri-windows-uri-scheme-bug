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

  function fetchJs() {
    fetch("http://ext.localhost/main.js")
      .then((res) => res.text())
      .then((text) => {
        console.log(text);
      });
  }
</script>

<span>{iframeSrc}</span>
{#if showIframe}
  <iframe src={iframeSrc} title="iframe" width="100%"></iframe>
{/if}
<button on:click={reloadIframe}>Reload</button>
<button on:click={fetchJs}>fetch http://ext.localhost/main.js</button>

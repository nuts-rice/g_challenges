<script lang="ts">
import {invoke} from '@tauri-apps/api/tauri';
let profile = '';
let response = '';
let favorites = [];
async function getProfile() {
  response = await invoke('getProfile');
}
</script>

<div>
</div>


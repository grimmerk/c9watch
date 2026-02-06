<script lang="ts">
	import { onMount } from 'svelte';
	import { initializeSessionListeners, sessions } from '$lib/stores/sessions';
	import { getSessions } from '$lib/api';
	import '../app.css';

	onMount(async () => {
		// Initialize Tauri event listeners for real-time updates
		await initializeSessionListeners();

		// Fetch initial session data and update store
		const initialSessions = await getSessions();
		sessions.set(initialSessions);
	});
</script>

<slot />

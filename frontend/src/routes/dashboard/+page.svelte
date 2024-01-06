<script>
	import { page } from '$app/stores';
	import { signOut } from '@auth/sveltekit/client';
	import { goto } from '$app/navigation';
	import { redirect } from '@sveltejs/kit';
	import { onMount } from 'svelte';

	$: console.log($page);

	onMount(() => {
		if (!$page.data.session) {
			goto('/');
		}
	});
</script>

{#if $page.data.session}
	<span class="signedInText"> </span>
{/if}

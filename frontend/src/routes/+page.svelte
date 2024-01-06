<script>
	import { goto } from '$app/navigation';
	import { signIn } from '@auth/sveltekit/client';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';

	onMount(() => {
		// TODO: Check if user is a new user based on whether the name field is empty
		if ($page.data.session?.user?.email) {
			goto('/dashboard');
		} else {
			signIn('cognito');
		}
	});
</script>

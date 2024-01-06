<script>
	import { signOut } from '@auth/sveltekit/client';
</script>

<div class="flex flex-row justify-start gap-x-10">
	<nav class="border-r w-80 h-[calc(100vh-1.5rem)] flex flex-col gap-y-52 text-center pt-56">
		<div class="flex flex-col gap-y-4">
			<a href="/dashboard/">Servers</a>
			<a href="/dashboard/new-server">New Server</a>
		</div>

		<div class="border-t mx-8 py-2">
			<button on:click={() => signOut()}>Sign out</button>
		</div>
	</nav>
	<slot />
</div>

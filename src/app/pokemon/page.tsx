'use client';

import { emit } from '@tauri-apps/api/event';
import { useRouter } from 'next/navigation';

export default function Pokemon() {
	const router = useRouter();

	function handleClick() {
		console.log('emitting event to backend');
		emit('close', {
			theMessage: 'closing window'
		});
		router.push("/");
	}

	return (
		<>
			<h1>Pokemon page</h1>
			<button onClick={handleClick} className="bg-green-300 rounded-s" >Close</button>
		</>
	)
}

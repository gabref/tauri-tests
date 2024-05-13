'use client';

import { emit } from '@tauri-apps/api/event';

export default function Pokemon() {

	function handleClick() {
		emit('close', {
			theMessage: 'closing window'
		});
	}

	return (
		<>
			<h1>Pokemon page</h1>
			<button onClick={handleClick} className="bg-green-300 rounded-s" >Close</button>
		</>
	)
}

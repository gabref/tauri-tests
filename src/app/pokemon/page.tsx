'use client';

import { emit } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri'
import { useRouter } from 'next/navigation';
import { useEffect, useState } from 'react';

type OutputData = {
	id: number;
	name: string;
	is_last: boolean;
};

async function* getData(): AsyncGenerator<OutputData> {
	while (true) {
		await new Promise((resolve) => setTimeout(resolve, 2000));
		const res = await invoke<string>('create_json_string');
		const resJson = JSON.parse(res) as OutputData;
		if (resJson.is_last) {
			resJson.name = resJson.name.concat(" - the end");
			yield resJson;
			return;
		}
		yield resJson;
	}
}

export default function Pokemon() {
	const router = useRouter();
	const [lastData, setLastData] = useState<OutputData | null>(null);
	const [randomString, setRandomString] = useState('');
	const appState = getData();

	function handleClick() {
		console.log('emitting event to backend');
		if (lastData != null)
			emit('close', { lastData })
		else
			emit('close', { });
		new Promise(() => setTimeout(() => { router.push("/") }, 2000));
	}

	useEffect(() => {
		const update = async () => {
			let lastStateInScope;
			for await (const state of appState) {
				lastStateInScope = state;
				setLastData(state);
				setRandomString(state.id + ' ' + state.name)
			}
			console.log('FOR AWAIT ENDED');
			await new Promise(resolve => setTimeout(resolve, 3000));
			console.log('3S, CLOSING WINDOW');
			if (lastStateInScope != null)
				emit('close', { lastStateInScope });
			else
				emit('close', { });
		}
		update();
	}, []);

	return (
		<>
			<h1>Pokemon page</h1>
			<p>{randomString}</p>
			<button onClick={handleClick} className="bg-green-300 rounded-s" >Close</button>
		</>
	)
}

'use client'

import { listen } from '@tauri-apps/api/event'
import { useRouter } from 'next/navigation';
import { useEffect } from 'react';

type Payload = {
	to_page: string;
}

export default function Home() {
	const router = useRouter();

	useEffect(() => {
		const unlisten = async() => await listen('push', event => {
			console.log('event', {event} );
			const { payload } = event as { payload: Payload };
			console.log('payload', { payload });
			router.push(payload.to_page);
		})
		console.log('waiting for event')
		return () => {
			unlisten();
		}
	}, [])

	return (
		<h1 className="mt-10">Hello World!</h1>
	);
}

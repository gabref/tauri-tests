'use client'

import { listen } from '@tauri-apps/api/event'
import { useRouter } from 'next/router';
import { useEffect } from 'react';

export default function Home() {
	const router = useRouter();

	useEffect(() => {
		const unlisten = async() => await listen('push', event => {
			console.log('payload', event.payload);
			router.push('/pokemon')
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

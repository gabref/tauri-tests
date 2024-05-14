'use client';

import { BaseDirectory } from '@tauri-apps/api/path';
import { readDir } from '@tauri-apps/api/fs';
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { useRouter } from 'next/navigation';
import { useEffect, useState } from 'react';

type OutputData = {
	id: number;
	name: string;
	is_last: boolean;
};

export default function Waifu() {
	const router = useRouter();
	const [lastData, setLastData] = useState<OutputData | null>(null);
	const [randomString, setRandomString] = useState('');

	function handleClick() {
		const getImages = async () => {
			console.log('base dir', BaseDirectory.AppCache);
			const images_in_cache = await readDir('images',
				{ dir: BaseDirectory.AppCache, recursive: false }
			);
			console.log('images in cache obj', images_in_cache);
			images_in_cache.forEach(async (entry, i) => {
				console.log(i + ' ' + { entry });
			});
		}
		getImages();
	}

	return (
		<>
			<h1>Waifu page</h1>
			<p>{randomString}</p>
			<button onClick={handleClick} className="bg-green-300 rounded-s" >Run</button>
		</>
	)
}

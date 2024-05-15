import { displayCachedImage } from "@/services/image-caching";
import { useEffect, useState } from "react";

interface ImageProps {
	src: string;
	className: string;
}

export function ImageCache({ src, className }: ImageProps) {
	const [image, setImage] = useState("/thumbnail.png");

	useEffect(() => {
		loadImage();
	}, []);

	async function loadImage() {
		const loadedImage = await displayCachedImage(src);
		setImage(loadedImage);
	}

	return (
		<div>
			<img src={image} className={className} />
		</div>
	);
}

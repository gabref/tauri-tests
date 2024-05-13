import Image from 'next/image';
import Link from 'next/link'

type Logo = {
	id: string;
	src: string;
	alt: string;
}

const logo: Logo = { 
	id: 'embed', 
	src: '/logos/embed_verde_positivo.png', 
	alt: 'Logo Embed' 
};

export default function Header() {
	return (
		<header>
			<nav className='py-8 h-20 flex justify-between'>
				<ul className='container flex gap-10'>
					<li>
						<Link href='/'>Home</Link>
					</li>
					<li>
						<Link href='/configure'>Configurar</Link>
					</li>
					<li>
						<Link href='/about'>Sobre</Link>
					</li>
				</ul>
				<ul className='container flex gap-4 px-16 items-center justify-end'>
					<li key={'logo'}>
						<Image
							src={logo.src}
							alt={logo.alt}
							width={logo.id === 'embed' ? 296 : 200}
							height={0}
						/>
					</li>
				</ul>
			</nav>
		</header>
	)
}

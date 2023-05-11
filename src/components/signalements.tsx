import { FilledButton } from "@/components/controls/buttons";

export function Signalements() {
	let signalements = [];

	if (!signalements.length) {
		return <div>
			<div style={{
				width: '100%',
				textAlign: 'center',
				marginTop: 50,
				opacity: '75%',
				fontStyle: 'italic',
			}}>
				Wow, c'est vide ici!
			</div>
			<div>
				<FilledButton />
			</div>
		</div>
	}
}


import { CSSProperties } from "react";

export function FilledButton () {

	return <button style={filledButtonStyle}>Ajouter un signalement</button>
}

const filledButtonStyle: CSSProperties = {
	height: 50,
	borderRadius: 10,
	borderWidth: 3,
	borderColor: 'black',
	borderStyle: 'solid',
	backgroundColor: '#E1F8FF',
};


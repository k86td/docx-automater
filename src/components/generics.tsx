import { Link } from "react-router-dom";

interface ActionProps {
	title: string,
	description: string,
}

interface HeaderTitleProps {
	pages?: string[],
}

export function HeaderTitle(props: HeaderTitleProps) {
	const { pages = [] } = props;

	let navigationPath = "> " + pages.join(" > ");

	return <div style={{
		width: 450,
	}}>
		<span style={{
			fontSize: 40,
		}}>
			<Link to='/'>
				<span style={{
					color: "#000",
					textDecoration: '',
				}}>Le truc pour automatiser tes trucs</span>
			</Link>
		</span>

		<br />

		{
			// removed because don't want to figure how to setup correctly
			/*<span style={{
				fontSize: 25,
				opacity: "75%",
			}}>
				{navigationPath}
			</span>*/
		}
	</div >
}


export function Action(props: ActionProps) {
	return <Link to='/signalements'><div style={{
		width: '300px',
	}}>
		<div>
			<span className="title-description">{props.description}</span>
		</div>
		<span className="title-title">{props.title}</span>
	</div ></Link>
}


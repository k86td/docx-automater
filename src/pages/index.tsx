
import { BrowserRouter, Route, Routes } from 'react-router-dom';
import { Action, HeaderTitle } from '@/components/generics';
import React from 'react';
import { Signalements } from '@/components/signalements';

function Main() {
	return <main>
		<div id='menu-actions' className='horizontal-centered'>
			<Action title='Signalements' description="Ajout, modification et suppression de signalement" />
		</div>
	</main>
}

function App() {
	return (
		<BrowserRouter>
			<HeaderTitle />
			<Routes>
				<Route path="/" element={Main()} />
				<Route path="/signalements" element={Signalements()} />
			</Routes>
		</BrowserRouter>
	);
}

module.exports = App;


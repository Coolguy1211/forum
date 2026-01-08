import React from 'react';
import Feed from './components/Feed';
import NewPostForm from './components/NewPostForm';
import IdentityInfo from './components/IdentityInfo';
import './App.css';

const App: React.FC = () => {
    return (
        <div className="App">
            <header className="App-header">
                <h1>Local Community Feed</h1>
            </header>
            <main>
                <IdentityInfo />
                <NewPostForm />
                <Feed />
            </main>
        </div>
    );
};

export default App;
